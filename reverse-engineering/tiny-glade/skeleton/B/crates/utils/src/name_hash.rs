// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/utils/src/name_hash.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 26ce7a4dfdf5438fd0f4938a811d0374435de402472978b31d502db02c291f0c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: name_hash.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 32, 250, 446, 510, 517, 559, 599, 770, 1423, 1492, 1568, 1606, 2627, 2816, 3098, 3471, 3481, 3697, 3873, 3975, 4424, 4441, 4464
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use xxhash_rust; // namespace candidate
//   package identities: [{"name":"xxhash-rust","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.10"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::name_hash::impl$7::fmt
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x71B8
// 首条位置: {"module":510,"record_offset":4720,"rva":9672416,"va":5378381536}
// 编译实例 RVAs: 0x9396e0, 0x206f290
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field1_finish
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4510921","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378381536,"raw_hex":"e8c9d44400","size":5,"soft_fail":false,"target_va":5382892528,"va":5378381602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::name_hash::impl$0::hash
// 记录形式: inline_instance, inlinee_declaration; instances=38
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x202A
// 首条位置: {"module":250,"inlinee":8234}
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
// 首条位置: {"module":770,"record_offset":252,"rva":13225328,"va":5381934448}
// 编译实例 RVAs: 0xc9cd70
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
// 首条位置: {"module":770,"record_offset":356,"rva":13225344,"va":5381934464}
// 编译实例 RVAs: 0xc9cd80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] xxhash_rust::xxh3::xxh3_64_internal
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"xxhash-rust","version":"0.8.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1889048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381934464,"raw_hex":"e818d31c00","size":5,"soft_fail":false,"target_va":5383823552,"va":5381934499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] xxhash_rust::xxh3::xxh3_64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"xxhash-rust","version":"0.8.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/xxhash-rust-0.8.10/src/xxh3.rs
//     evidence={"module":770,"record_offset":476,"parent_record_offset":356,"ancestor_record_offset":356,"inlinee":45104,"source_line":661,"archive":"_pdb/B/modules/00770.jsonl.gz"}
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
// 首条位置: {"module":770,"record_offset":504,"rva":13225392,"va":5381934512}
// 编译实例 RVAs: 0xc9cdb0
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "nohash_hasher::NoHashHasher<utils::name_hash::NameHash>", "constructor": "nohash_hasher::NoHashHasher", "arguments": ["utils::name_hash::NameHash"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> >", "constructor": "core::hash::BuildHasherDefault", "arguments": ["nohash_hasher::NoHashHasher<utils::name_hash::NameHash>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::HashMap<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> > >", "constructor": "std::collections::hash::map::HashMap", "arguments": ["utils::name_hash::NameHash", "country_core::asset_libraries::texture_library::NamedTexture", "core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::collections::hash::map::HashMap<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::collections::hash::map::HashMap<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::get_by_name<utils::name_hash::NameHash>", "constructor": "country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::get_by_name", "arguments": ["utils::name_hash::NameHash"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture>", "constructor": "tuple$", "arguments": ["utils::name_hash::NameHash", "country_core::asset_libraries::texture_library::NamedTexture"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> > >", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["utils::name_hash::NameHash", "country_core::asset_libraries::texture_library::NamedTexture", "core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> > > >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<utils::name_hash::NameHash>", "constructor": "ref$", "arguments": ["utils::name_hash::NameHash"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$27::fmt<ref$<utils::name_hash::NameHash>,alloc::alloc::Global>", "constructor": "alloc::vec::impl$27::fmt", "arguments": ["ref$<utils::name_hash::NameHash>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::Keys<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture>", "constructor": "std::collections::hash::map::Keys", "arguments": ["utils::name_hash::NameHash", "country_core::asset_libraries::texture_library::NamedTexture"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<ref$<utils::name_hash::NameHash>,std::collections::hash::map::Keys<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["ref$<utils::name_hash::NameHash>", "std::collections::hash::map::Keys<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$74::fmt<ref$<utils::name_hash::NameHash> >", "constructor": "core::fmt::impl$74::fmt", "arguments": ["ref$<utils::name_hash::NameHash>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$74::fmt<utils::name_hash::NameHash>", "constructor": "core::fmt::impl$74::fmt", "arguments": ["utils::name_hash::NameHash"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 14; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_watch::TextureWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_watch::TextureWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_watch::TextureWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::writer::EventWriter<country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::event::writer::EventWriter<country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher> > >,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::configwatch::ConfigWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::configwatch::ConfigWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::configwatch::ConfigWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::commands::Commands"]
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: utils::name_hash::NameHash::compute
// 信息 ID: ac3e0950139cc53dec2f03e0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 29 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::impl$9::prepare_layout @ 0x67ec25 -> 0xc9cd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_dormer::generate_gothic_dormers @ 0x7c680f -> 0xc9cd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_wall_window::generate_gothic_wall_windows @ 0x7e976d -> 0xc9cd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery @ 0x8b101d -> 0xc9cd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::texture_library::AssetTextureLibrary::add @ 0x93c262 -> 0xc9cd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::texture_library::AssetTextureLibrary::add @ 0x93c4a8 -> 0xc9cd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::texture_library::AssetTextureLibrary::create_name_mapping @ 0x93c77a -> 0xc9cd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::texture_library::AssetTextureLibrary::create_name_mapping @ 0x93c848 -> 0xc9cd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::texture_library::impl$2::image_name_to_index @ 0x93d6d2 -> 0xc9cd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass @ 0x9c4ce1 -> 0xc9cd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass @ 0x9c5766 -> 0xc9cd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass @ 0x9c5114 -> 0xc9cd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::raster_single::raster_single_object<1,array$<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::final_post::final_post::closure_env$1> @ 0x9cf16b -> 0xc9cd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::raster_single::raster_single_object<1,array$<enum2$<country_core::render::pass_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::final_post::final_post::closure_env$1> @ 0x9cf369 -> 0xc9cd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::add<ref$<str$> > @ 0xa37216 -> 0xc9cd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::backend::texture_watch::texture_watch @ 0xaab5eb -> 0xc9cd80 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::name_hash::NameHash::debug_name
// 信息 ID: d93c4f15a7c7d2f86f8192a6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<tuple$<country_core::resources::texture_presets::TexturePreset,country_core::asset_libraries::texture_library::PresetTextureState>,enum_iterator::All<country_core::resources::texture_presets::TexturePreset>,country_core::asset_libraries::texture_library::impl$1::create_preset_textures::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::texture_presets::TexturePreset,country_core::asset_libraries::texture_library::PresetTextureState>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::texture_presets::TexturePreset,country_core::asset_libraries::texture_library::PresetTextureState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<enum_iterator::All<country_core::resources::texture_presets::TexturePreset>,country_core::asset_libraries::texture_library::impl$1::create_preset_textures::closure_env$0> > > > @ 0x1474c76 -> 0xc9cdb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::name_hash::impl$0::hash
// 信息 ID: 1431e6116788a25a244ca216
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 19 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] frozen::FrozenMap::get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >,country_core::asset_libraries::ssbo_library::SsboId>; module=250, inline_record=35340; 不是独立调用边。
// [PDB 内联宿主] frozen::FrozenMap::get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >,country_core::asset_libraries::ssbo_library::SsboId>; module=517, inline_record=10064; 不是独立调用边。
// [PDB 内联宿主] hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,alloc::vec::Vec<country_core::resources::render::mesh_atlas_library::PrefabMeshAtlasPart,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,alloc::vec::Vec<country_core::resources::render::mesh_atlas_library::PrefabMeshAtlasPart,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::resources::render::mesh_atlas_library::AtlassedMeshName> > > >; module=599, inline_record=370508; 不是独立调用边。
// [PDB 内联宿主] hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,alloc::vec::Vec<country_core::resources::render::mesh_atlas_library::PrefabMeshAtlasPart,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,alloc::vec::Vec<country_core::resources::render::mesh_atlas_library::PrefabMeshAtlasPart,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::resources::render::mesh_atlas_library::AtlassedMeshName> > > >; module=599, inline_record=372992; 不是独立调用边。
// [PDB 内联宿主] hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::render::nani::mesh_atlas::MeshAtlasPart>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::render::nani::mesh_atlas::MeshAtlasPart,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::resources::render::mesh_atlas_library::AtlassedMeshName> > > >; module=599, inline_record=384176; 不是独立调用边。
// [PDB 内联宿主] hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::render::nani::mesh_atlas::MeshAtlasPart>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::render::nani::mesh_atlas::MeshAtlasPart,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::resources::render::mesh_atlas_library::AtlassedMeshName> > > >; module=599, inline_record=386656; 不是独立调用边。
// [PDB 内联宿主] hashbrown::raw::RawTable::reserve_rehash<tuple$<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> > > >; module=599, inline_record=642168; 不是独立调用边。
// [PDB 内联宿主] hashbrown::raw::RawTable::reserve_rehash<tuple$<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<utils::name_hash::NameHash,country_core::asset_libraries::texture_library::NamedTexture,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<utils::name_hash::NameHash> > > >; module=599, inline_record=644656; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::name_hash::impl$2::eq
// 信息 ID: 6e95b232fdbfcd8ac5db39d0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 43 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] frozen::FrozenMap::get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >,country_core::asset_libraries::ssbo_library::SsboId> @ 0x5ca955 -> 0xc9cd70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::texture_library::AssetTextureLibrary::add @ 0x93c2ee -> 0xc9cd70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::texture_library::AssetTextureLibrary::create_name_mapping @ 0x93c7fa -> 0xc9cd70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_handle_and_texture_by_name @ 0x93ca0f -> 0xc9cd70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_by_name @ 0x93cb38 -> 0xc9cd70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::texture_library::impl$2::image_name_to_index @ 0x93d74f -> 0xc9cd70 (direct_call); 同地址别名未裁定。
// [静态直接引用] frozen::FrozenMap::get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >,country_core::asset_libraries::ssbo_library::SsboId> @ 0x969805 -> 0xc9cd70 (direct_call); 同地址别名未裁定。
// [静态直接引用] frozen::FrozenMap::insert<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> > > @ 0x969cf6 -> 0xc9cd70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::add<ref$<str$> > @ 0xa37500 -> 0xc9cd70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::add<asset::prefab::PrefabName> @ 0xa37a90 -> 0xc9cd70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::clear_atlas_by_name<ref$<asset::prefab::PrefabName> > @ 0xa37efe -> 0xc9cd70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::clear_atlas_by_name<ref$<asset::prefab::PrefabName> > @ 0xa380a3 -> 0xc9cd70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::get_by_name<utils::name_hash::NameHash> @ 0xa381f4 -> 0xc9cd70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::get_by_name<ref$<asset::prefab::PrefabNameRef> > @ 0xa38382 -> 0xc9cd70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::backend::texture_watch::texture_watch @ 0xaab684 -> 0xc9cd70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_mut<country_core::resources::sparkle::SparkleSsbo> @ 0x1209948 -> 0xc9cd70 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::name_hash::impl$7::fmt
// 信息 ID: 199f0d709fae86204c1de6a4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$74::fmt<ref$<utils::name_hash::NameHash> >; module=1606, inline_record=424; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$74::fmt<utils::name_hash::NameHash>; module=1606, inline_record=596; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
