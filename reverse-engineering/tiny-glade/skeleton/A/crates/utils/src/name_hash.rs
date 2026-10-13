// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/utils/src/name_hash.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 7041ff534f617593d7b8cf219deaac8592867710cb6aaa27583316645b3336e3
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: name_hash.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 200, 219, 230, 393, 407, 410, 475, 495, 618, 745, 1205, 1212, 1323, 1370, 2212, 2774, 2830, 3212, 3421, 3531, 3603, 3697, 3840, 4106, 4208, 4219, 4250, 4630
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use xxhash_rust; // namespace candidate
//   package identities: [{"name":"xxhash-rust","source":"PDB_registry_path_only","version":"0.8.10"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::name_hash::impl$7::fmt
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x57B5
// 首条位置: {"module":393,"inlinee":22453}
// 编译实例 RVAs: 0x561380, 0x1817b60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field1_finish
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t3233737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374350208,"raw_hex":"e8c9573100","size":5,"soft_fail":false,"target_va":5377584016,"va":5374350274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::name_hash::impl$0::hash
// 记录形式: inline_instance, inlinee_declaration; instances=46
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x31C8
// 首条位置: {"module":219,"inlinee":12744}
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
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":618,"record_offset":244,"rva":8237280,"va":5376946400}
// 编译实例 RVAs: 0x7db0e0
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
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":618,"record_offset":348,"rva":8237296,"va":5376946416}
// 编译实例 RVAs: 0x7db0f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] xxhash_rust::xxh3::xxh3_64_internal
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1723016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376946416,"raw_hex":"e8884a1a00","size":5,"soft_fail":false,"target_va":5378669472,"va":5376946451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] xxhash_rust::xxh3::xxh3_64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"xxhash-rust","version":"0.8.10","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/xxhash-rust-0.8.10/src/xxh3.rs
//     evidence={"module":618,"record_offset":468,"parent_record_offset":348,"ancestor_record_offset":348,"inlinee":36126,"source_line":661,"archive":"_pdb/A/modules/00618.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::name_hash::NameHash::debug_name
// 记录形式: compiled_procedure; instances=1
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":618,"record_offset":496,"rva":8237344,"va":5376946464}
// 编译实例 RVAs: 0x7db120
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
// [PDB 泛型关系，非字段关系] {"type": "ref$<utils::name_hash::NameHash>", "constructor": "ref$", "arguments": ["utils::name_hash::NameHash"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$51::fmt<ref$<utils::name_hash::NameHash> >", "constructor": "core::fmt::impl$51::fmt", "arguments": ["ref$<utils::name_hash::NameHash>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$51::fmt<utils::name_hash::NameHash>", "constructor": "core::fmt::impl$51::fmt", "arguments": ["utils::name_hash::NameHash"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::get_by_name<utils::name_hash::NameHash>", "constructor": "country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::get_by_name", "arguments": ["utils::name_hash::NameHash"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "nohash_hasher::NoHashHasher<utils::name_hash::NameHash>", "constructor": "nohash_hasher::NoHashHasher", "arguments": ["utils::name_hash::NameHash"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> >", "constructor": "core::hash::BuildHasherDefault", "arguments": ["nohash_hasher::NoHashHasher<utils::name_hash::NameHash>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::HashMap<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> > >", "constructor": "std::collections::hash::map::HashMap", "arguments": ["utils::name_hash::NameHash", "country_core::asset_libraries::texture_library::NamedTexture", "core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::collections::hash::map::HashMap<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::collections::hash::map::HashMap<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$26::fmt<ref$<utils::name_hash::NameHash>,alloc::alloc::Global>", "constructor": "alloc::vec::impl$26::fmt", "arguments": ["ref$<utils::name_hash::NameHash>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::Keys<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture>", "constructor": "std::collections::hash::map::Keys", "arguments": ["utils::name_hash::NameHash", "country_core::asset_libraries::texture_library::NamedTexture"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<ref$<utils::name_hash::NameHash>,std::collections::hash::map::Keys<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["ref$<utils::name_hash::NameHash>", "std::collections::hash::map::Keys<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture>", "constructor": "tuple$", "arguments": ["utils::name_hash::NameHash", "country_core::asset_libraries::texture_library::NamedTexture"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> > >", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["utils::name_hash::NameHash", "country_core::asset_libraries::texture_library::NamedTexture", "core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> > > >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<ref$<utils::name_hash::NameHash> >", "constructor": "core::ptr::drop_in_place", "arguments": ["ref$<utils::name_hash::NameHash>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> >,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["utils::name_hash::NameHash", "country_core::asset_libraries::texture_library::NamedTexture", "core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] country_core::resources::walls::decorator::DecoratorId (*)(ref$<country_core::resources::walls::decorator::DecoratorId>)
// 候选参数类型: ["ref$<country_core::resources::walls::decorator::DecoratorId>"]
// [文件级接口候选，具体函数归属 unknown] core::char::EscapeDebug (*)(char)
// 候选参数类型: ["char"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<tiny_glade::app_config::ApplyVideoSettingsCmd>)
// 候选参数类型: ["bevy_ecs::event::EventReader<tiny_glade::app_config::ApplyVideoSettingsCmd>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<tiny_glade::app_config::ApplyVideoSettingsCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallDeleted>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>

// ------------------------------------------------------------------
// 函数: utils::name_hash::NameHash::compute
// 信息 ID: 7bef1347934c1510f5a57ccc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 22 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::impl$11::prepare @ 0x43b763 -> 0x7db0f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::add<ref$<str$> > @ 0x541778 -> 0x7db0f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery @ 0x555cc9 -> 0x7db0f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::texture_library::AssetTextureLibrary::add @ 0x561db1 -> 0x7db0f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::texture_library::AssetTextureLibrary::add @ 0x561f2a -> 0x7db0f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::texture_library::AssetTextureLibrary::create_name_mapping @ 0x5621fa -> 0x7db0f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::texture_library::AssetTextureLibrary::create_name_mapping @ 0x5622c5 -> 0x7db0f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::texture_library::impl$2::image_name_to_index @ 0x5631f2 -> 0x7db0f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::render::mesh_atlas_library::impl$1::from @ 0x5633d0 -> 0x7db0f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::render::mesh_atlas_library::impl$3::from @ 0x5634fd -> 0x7db0f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass @ 0x687419 -> 0x7db0f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass @ 0x6877dd -> 0x7db0f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::raster_single::raster_single_object<1,array$<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::final_post::final_post::closure_env$1> @ 0x6a0dd4 -> 0x7db0f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::raster_single::raster_single_object<1,array$<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::final_post::final_post::closure_env$1> @ 0x6a0fcf -> 0x7db0f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::clearing::init_clearing @ 0x6c87e1 -> 0x7db0f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_dormer::generate_gothic_dormers @ 0xd0d23e -> 0x7db0f0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::name_hash::NameHash::debug_name
// 信息 ID: 92a07e538950656012ffc840
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<tuple$<country_core::resources::texture_presets::TexturePreset,country_core::asset_libraries::texture_library::PresetTextureState>,enum_iterator::All<country_core::resources::texture_presets::TexturePreset>,country_core::asset_libraries::texture_library::impl$1::create_preset_textures::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::texture_presets::TexturePreset,country_core::asset_libraries::texture_library::PresetTextureState>,hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::texture_presets::TexturePreset,country_core::asset_libraries::texture_library::PresetTextureState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<enum_iterator::All<country_core::resources::texture_presets::TexturePreset>,country_core::asset_libraries::texture_library::impl$1::create_preset_textures::closure_env$0> > > > @ 0xde3606 -> 0x7db120 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::name_hash::impl$0::hash
// 信息 ID: 42d3e76a176577d70021d146
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 21 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] frozen::FrozenMap::get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >,country_core::asset_libraries::ssbo_library::SsboId>; module=219, inline_record=11028; 不是独立调用边。
// [PDB 内联宿主] frozen::FrozenMap::get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >,country_core::asset_libraries::ssbo_library::SsboId>; module=475, inline_record=6836; 不是独立调用边。
// [PDB 内联宿主] frozen::FrozenMap::get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >,country_core::asset_libraries::ssbo_library::SsboId>; module=745, inline_record=2012; 不是独立调用边。
// [PDB 内联宿主] frozen::FrozenMap::get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >,country_core::asset_libraries::ssbo_library::SsboId>; module=1212, inline_record=2024; 不是独立调用边。
// [PDB 内联宿主] hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> > > >; module=1323, inline_record=380940; 不是独立调用边。
// [PDB 内联宿主] hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> > > >; module=1323, inline_record=382716; 不是独立调用边。
// [PDB 内联宿主] hashbrown::raw::RawTable::reserve_rehash<tuple$<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> > > >; module=1323, inline_record=606400; 不是独立调用边。
// [PDB 内联宿主] hashbrown::raw::RawTable::reserve_rehash<tuple$<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> > > >; module=1323, inline_record=608184; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::name_hash::impl$2::eq
// 信息 ID: 2732693d660b76b89ee300d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 33 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] frozen::FrozenMap::get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >,country_core::asset_libraries::ssbo_library::SsboId> @ 0x339800 -> 0x7db0e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::add<ref$<str$> > @ 0x541917 -> 0x7db0e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::add<asset::prefab::PrefabName> @ 0x542027 -> 0x7db0e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::get_by_name<utils::name_hash::NameHash> @ 0x5425f5 -> 0x7db0e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::get_by_name<ref$<asset::prefab::PrefabNameRef> > @ 0x542785 -> 0x7db0e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::texture_library::AssetTextureLibrary::add @ 0x561e4c -> 0x7db0e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::texture_library::AssetTextureLibrary::create_name_mapping @ 0x562292 -> 0x7db0e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_handle_and_texture_by_name @ 0x5624ab -> 0x7db0e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_by_name @ 0x5625f4 -> 0x7db0e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::texture_library::impl$2::image_name_to_index @ 0x563282 -> 0x7db0e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] frozen::FrozenMap::get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >,country_core::asset_libraries::ssbo_library::SsboId> @ 0x667450 -> 0x7db0e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] frozen::FrozenMap::insert<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> > > @ 0x667bbf -> 0x7db0e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_mut<country_core::resources::roof_tile_ssbo::RoofTileSsbo> @ 0x8d5154 -> 0x7db0e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] frozen::FrozenMap::get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >,country_core::asset_libraries::ssbo_library::SsboId> @ 0x8d5350 -> 0x7db0e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_mut<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo> @ 0xc98124 -> 0x7db0e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] frozen::FrozenMap::get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >,country_core::asset_libraries::ssbo_library::SsboId> @ 0xc98320 -> 0x7db0e0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::name_hash::impl$7::fmt
// 信息 ID: 607ff911e34b47ab7d57eca0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$51::fmt<ref$<utils::name_hash::NameHash> >; module=393, inline_record=416; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$51::fmt<utils::name_hash::NameHash>; module=393, inline_record=588; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
