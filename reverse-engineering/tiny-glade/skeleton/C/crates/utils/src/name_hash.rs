// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/utils/src/name_hash.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 26ce7a4dfdf5438fd0f4938a811d0374435de402472978b31d502db02c291f0c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: name_hash.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 39, 178, 255, 530, 599, 614, 765, 1281, 1454, 1464, 1550, 1658, 1700, 1713, 2421, 2450, 2690, 2713, 3222, 3273, 3797, 3927, 4087, 4148, 4259, 4731, 4749, 4869
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use xxhash_rust; // namespace candidate
//   package identities: [{"name":"xxhash-rust","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.10"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::name_hash::impl$8::fmt
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7D45
// 首条位置: {"module":530,"record_offset":92468,"rva":10584112,"va":5379293232}
// 编译实例 RVAs: 0xa18030, 0x25be570
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::fmt::Formatter>::debug_struct_field1_finish
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5527881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379293232,"raw_hex":"e849595400","size":5,"soft_fail":false,"target_va":5384821184,"va":5379293298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::name_hash::impl$0::hash
// 记录形式: inline_instance, inlinee_declaration; instances=47
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x214C
// 首条位置: {"module":178,"inlinee":8524}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::name_hash::impl$2::eq
// 记录形式: compiled_procedure; instances=1
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":765,"record_offset":524,"rva":14615664,"va":5383324784}
// 编译实例 RVAs: 0xdf0470
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::name_hash::NameHash::compute
// 记录形式: compiled_procedure; instances=1
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":765,"record_offset":376,"rva":14615616,"va":5383324736}
// 编译实例 RVAs: 0xdf0440
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] xxhash_rust::xxh3::xxh3_64_internal
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"xxhash-rust","version":"0.8.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2595576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383324736,"raw_hex":"e8f89a2700","size":5,"soft_fail":false,"target_va":5385920352,"va":5383324771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] xxhash_rust::xxh3::xxh3_64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"xxhash-rust","version":"0.8.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/xxhash-rust-0.8.10/src/xxh3.rs
//     evidence={"module":765,"record_offset":496,"parent_record_offset":376,"ancestor_record_offset":376,"inlinee":49274,"source_line":661,"archive":"_pdb/C/modules/00765.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::name_hash::NameHash::debug_name
// 记录形式: compiled_procedure; instances=1
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":765,"record_offset":260,"rva":14615600,"va":5383324720}
// 编译实例 RVAs: 0xdf0430
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
// 完整结构及原始类型记录: name_hash.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::name_hash::NameHash
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 11
// [PDB 泛型关系，非字段关系] {"type": "nohash_hasher::NoHashHasher<utils::name_hash::NameHash>", "constructor": "nohash_hasher::NoHashHasher", "arguments": ["utils::name_hash::NameHash"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> >", "constructor": "core::hash::BuildHasherDefault", "arguments": ["nohash_hasher::NoHashHasher<utils::name_hash::NameHash>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::HashMap<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> >,alloc::alloc::Global>", "constructor": "std::collections::hash::map::HashMap", "arguments": ["utils::name_hash::NameHash", "country_core::asset_libraries::texture_library::NamedTexture", "core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::collections::hash::map::HashMap<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::collections::hash::map::HashMap<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture>", "constructor": "tuple$", "arguments": ["utils::name_hash::NameHash", "country_core::asset_libraries::texture_library::NamedTexture"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::equivalent_key::closure_env$0<utils::name_hash::NameHash,utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture>", "constructor": "hashbrown::map::equivalent_key::closure_env$0", "arguments": ["utils::name_hash::NameHash", "utils::name_hash::NameHash", "country_core::asset_libraries::texture_library::NamedTexture"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::remove_entry<tuple$<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$0<utils::name_hash::NameHash,utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture> >", "constructor": "hashbrown::raw::RawTable::remove_entry", "arguments": ["tuple$<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture>", "alloc::alloc::Global", "hashbrown::map::equivalent_key::closure_env$0<utils::name_hash::NameHash,utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> > >", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["utils::name_hash::NameHash", "country_core::asset_libraries::texture_library::NamedTexture", "core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> > > >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<utils::name_hash::NameHash>", "constructor": "ref$", "arguments": ["utils::name_hash::NameHash"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$28::fmt<ref$<utils::name_hash::NameHash>,alloc::alloc::Global>", "constructor": "alloc::vec::impl$28::fmt", "arguments": ["ref$<utils::name_hash::NameHash>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::Keys<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture>", "constructor": "std::collections::hash::map::Keys", "arguments": ["utils::name_hash::NameHash", "country_core::asset_libraries::texture_library::NamedTexture"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<ref$<utils::name_hash::NameHash>,std::collections::hash::map::Keys<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["ref$<utils::name_hash::NameHash>", "std::collections::hash::map::Keys<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> >,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["utils::name_hash::NameHash", "country_core::asset_libraries::texture_library::NamedTexture", "core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<utils::name_hash::NameHash,u32,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> >,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["utils::name_hash::NameHash", "u32", "core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$80::fmt<ref$<utils::name_hash::NameHash> >", "constructor": "core::fmt::impl$80::fmt", "arguments": ["ref$<utils::name_hash::NameHash>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$80::fmt<utils::name_hash::NameHash>", "constructor": "core::fmt::impl$80::fmt", "arguments": ["utils::name_hash::NameHash"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: name_hash.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: name_hash.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 22; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter> >,tuple$<country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::change_detection::Res<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SaveGameCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SavingGameComplete>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >)
// 候选参数类型: ["tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallSelectionGroups>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>,bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter> >", "tuple$<country_core::resources::world_raster_defs::AllWorldRasterRead,bevy_ecs::change_detection::Res<country_core::resources::plant_sample_raster::PlantSampleRaster>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>,bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>,bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState> >", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SaveGameCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SavingGameComplete>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallSelectionGroups>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::weather::Weather>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::GateColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::systems::prefab::PrefabInstanceData>,ref$<country_core::resources::prefab::PrefabInstance>,ref$<country_core::components::TreeComponent> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_color_customization::GladeColorCustomization>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::plant_sample_raster::PlantSampleRaster>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_state::StairsState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::save::save_game::SaveGameReplayModTracker>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::image_formation::ImageFormationConfigs>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::StylePickerPreviewState>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::SaveGameCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::events::ShutdownGameCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::save::save_game::SavingGameComplete>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::sheep::Sheep>,ref_mut$<country_core::resources::critters::CritterGoal> >,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::ShapePlacement>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::SnapMode>,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld> > >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemSnapping>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::shapes::shape_placement::DontRecalculateShapePlacement>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::ShapePlacement>", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::SnapMode>", "bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld> > >", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemSnapping>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::shapes::shape_placement::DontRecalculateShapePlacement>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::ShapePlacement>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::SnapMode>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemSnapping>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::shapes::shape_placement::DontRecalculateShapePlacement>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>", "bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>", "bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafRng>,bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>", "bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafRng>", "bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafRng>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::system::system_param::Local<country_core::resources::ivy::ivy_direction_proposer::IvyDirectionProposer>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::PrevWallHoles>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>", "bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>", "bevy_ecs::system::system_param::Local<country_core::resources::ivy::ivy_direction_proposer::IvyDirectionProposer>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::PrevWallHoles>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::resources::ivy::ivy_direction_proposer::IvyDirectionProposer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::PrevWallHoles>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::shape_placement::ShapePlacement>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::shape_placement::ShapePlacement>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::shape_placement::ShapePlacement>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::ShapePlacement>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::ShapePlacement>", "bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::shape_placement::ShapePlacement>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: utils::name_hash::NameHash::compute
// 信息 ID: 8c583919ee367df0732b37dd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 52a61717991191b755af9e07: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>)
// 引用方: 原程序直接转移 167 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x39864f -> 0xdf0440 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x3986ab -> 0xdf0440 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x3987a9 -> 0xdf0440 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x39882d -> 0xdf0440 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x3988d7 -> 0xdf0440 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x3989ca -> 0xdf0440 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x398a6d -> 0xdf0440 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x398bf0 -> 0xdf0440 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$9::prepare_layout @ 0x721605 -> 0xdf0440 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_dormer::generate_gothic_dormers @ 0x88158a -> 0xdf0440 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_wall_window::generate_gothic_wall_windows @ 0x8be753 -> 0xdf0440 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::loading_screen::update_loading_screen @ 0x8f7521 -> 0xdf0440 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke @ 0x9b8663 -> 0xdf0440 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke @ 0x9b8743 -> 0xdf0440 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke @ 0x9b7ff7 -> 0xdf0440 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_nani::update_world_subset_material_args @ 0x9f0fe5 -> 0xdf0440 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::name_hash::NameHash::debug_name
// 信息 ID: 9e8292ba229e5c5e2dfe35a6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<tuple$<country_core::resources::texture_presets::TexturePreset,lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,country_core::asset_libraries::texture_library::PresetTextureState> >,enum_iterator::All<country_core::resources::texture_presets::TexturePreset>,country_core::asset_libraries::texture_library::impl$1::create_preset_textures::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::texture_presets::TexturePreset,lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,country_core::asset_libraries::texture_library::PresetTextureState> >,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::texture_presets::TexturePreset,lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,country_core::asset_libraries::texture_library::PresetTextureState>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<enum_iterator::All<country_core::resources::texture_presets::TexturePreset>,country_core::asset_libraries::texture_library::impl$1::create_preset_textures::closure_env$0> > > > @ 0x18100fb -> 0xdf0430 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::name_hash::impl$0::hash
// 信息 ID: 6a42710088419bec11c8c563
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 22 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] frozen::FrozenMap::get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >,country_core::asset_libraries::ssbo_library::SsboId>; module=178, inline_record=6652; 不是独立调用边。
// [PDB 内联宿主] frozen::FrozenMap::get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >,country_core::asset_libraries::ssbo_library::SsboId>; module=599, inline_record=65596; 不是独立调用边。
// [PDB 内联宿主] hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,alloc::vec::Vec<country_core::resources::render::mesh_atlas_library::PrefabMeshAtlasPart,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,alloc::vec::Vec<country_core::resources::render::mesh_atlas_library::PrefabMeshAtlasPart,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::resources::render::mesh_atlas_library::AtlassedMeshName> > > >; module=614, inline_record=482656; 不是独立调用边。
// [PDB 内联宿主] hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,alloc::vec::Vec<country_core::resources::render::mesh_atlas_library::PrefabMeshAtlasPart,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,alloc::vec::Vec<country_core::resources::render::mesh_atlas_library::PrefabMeshAtlasPart,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::resources::render::mesh_atlas_library::AtlassedMeshName> > > >; module=614, inline_record=484936; 不是独立调用边。
// [PDB 内联宿主] hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::render::nani::mesh_atlas::MeshAtlasPart>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::render::nani::mesh_atlas::MeshAtlasPart,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::resources::render::mesh_atlas_library::AtlassedMeshName> > > >; module=614, inline_record=548460; 不是独立调用边。
// [PDB 内联宿主] hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::render::nani::mesh_atlas::MeshAtlasPart>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::render::nani::mesh_atlas::MeshAtlasPart,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::resources::render::mesh_atlas_library::AtlassedMeshName> > > >; module=614, inline_record=550736; 不是独立调用边。
// [PDB 内联宿主] hashbrown::raw::RawTable::reserve_rehash<tuple$<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> > > >; module=614, inline_record=619244; 不是独立调用边。
// [PDB 内联宿主] hashbrown::raw::RawTable::reserve_rehash<tuple$<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> > > >; module=614, inline_record=621492; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::name_hash::impl$2::eq
// 信息 ID: 71ef1fbde08a020824cc10b1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 101 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] frozen::FrozenMap::get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >,country_core::asset_libraries::ssbo_library::SsboId> @ 0x5181cd -> 0xdf0470 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::gpu_flush_plank_buffers @ 0x818a83 -> 0xdf0470 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::gpu_flush_plank_buffers @ 0x818b85 -> 0xdf0470 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::gpu_flush_plank_buffers @ 0x818c83 -> 0xdf0470 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke @ 0x9b86ea -> 0xdf0470 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke @ 0x9b87cd -> 0xdf0470 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gather_wall_draw_batches @ 0x9be677 -> 0xdf0470 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::texture_library::impl$2::image_name_to_index @ 0xa113b4 -> 0xdf0470 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::texture_library::AssetTextureLibrary::add_or_replace @ 0xa14136 -> 0xdf0470 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::texture_library::AssetTextureLibrary::remove_by_name @ 0xa146fa -> 0xdf0470 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_by_name_hash @ 0xa148ef -> 0xdf0470 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::texture_library::AssetTextureLibrary::try_create_name_mapping @ 0xa14e6c -> 0xdf0470 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_handle_and_texture_by_name @ 0xa154a1 -> 0xdf0470 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::texture_library::AssetTextureLibrary::add @ 0xa1560d -> 0xdf0470 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_bindless_handle_and_type_by_name_hash @ 0xa15d64 -> 0xdf0470 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::final_post::final_post @ 0xa2c4f9 -> 0xdf0470 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::name_hash::impl$8::fmt
// 信息 ID: cc3409d80925c898caf3ffe7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$80::fmt<ref$<utils::name_hash::NameHash> >; module=1713, inline_record=170836; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$80::fmt<utils::name_hash::NameHash>; module=1713, inline_record=171008; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
