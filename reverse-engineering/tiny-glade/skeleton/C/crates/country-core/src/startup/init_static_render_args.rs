// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/startup/init_static_render_args.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 4b5a94c91c12b7316b6087b4ef3a87e7cc52fcbec59722b12b2f7054e317a37d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: init_static_render_args.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 576, 1671, 1698
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.14"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.5"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.12"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::init_static_render_args::init_static_render_args
// 记录形式: compiled_procedure; instances=1
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":576,"record_offset":47456,"rva":12142928,"va":5380852048}
// 编译实例 RVAs: 0xb94950
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140395120
//     owner=unresolved_namespace; count=1
//     target_aliases=18 candidates; complete list: init_static_render_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8388053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380852048,"raw_hex":"e82b0280ff","size":5,"soft_fail":false,"target_va":5372465440,"va":5380853488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415622c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init_static_render_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10278036","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380852048,"raw_hex":"e894d49c00","size":5,"soft_fail":false,"target_va":5391131328,"va":5380853287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141578110
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init_static_render_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10367637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380852048,"raw_hex":"e895329e00","size":5,"soft_fail":false,"target_va":5391221008,"va":5380853366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415b1d40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init_static_render_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10605361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380852048,"raw_hex":"e831d3a100","size":5,"soft_fail":false,"target_va":5391457600,"va":5380852234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415c7e60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init_static_render_args.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10695238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380852048,"raw_hex":"e84632a300","size":5,"soft_fail":false,"target_va":5391548000,"va":5380852757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_by_name_hash
//     owner=package_namespace_candidate; count=26
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1573211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380852048,"raw_hex":"e8a5fee7ff","size":5,"soft_fail":false,"target_va":5379278928,"va":5380852134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_preset
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1577393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380852048,"raw_hex":"e84feee7ff","size":5,"soft_fail":false,"target_va":5379275920,"va":5380853308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t35718698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380852048,"raw_hex":"e82a062102","size":5,"soft_fail":false,"target_va":5416572144,"va":5380853441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3131976","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380852048,"raw_hex":"e848ca2f00","size":5,"soft_fail":false,"target_va":5383985360,"va":5380853379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::name_hash::NameHash::compute
//     owner=package_namespace_candidate; count=26
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2472608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380852048,"raw_hex":"e8a0ba2500","size":5,"soft_fail":false,"target_va":5383324736,"va":5380852123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":576,"record_offset":48704,"parent_record_offset":47456,"ancestor_record_offset":47456,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_by_name
//     owner=private_source_ownership_candidate; count=26
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/texture_library.rs
//     evidence={"module":576,"record_offset":47680,"parent_record_offset":47456,"ancestor_record_offset":47456,"inlinee":12354,"source_line":298,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":576,"record_offset":47624,"parent_record_offset":47456,"ancestor_record_offset":47456,"inlinee":27991,"source_line":42,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::binding_set::BindingSet::update
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/binding_set.rs
//     evidence={"module":576,"record_offset":47736,"parent_record_offset":47456,"ancestor_record_offset":47456,"inlinee":27992,"source_line":406,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":576,"record_offset":47648,"parent_record_offset":47624,"ancestor_record_offset":47456,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":576,"record_offset":48884,"parent_record_offset":48760,"ancestor_record_offset":47456,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":576,"record_offset":48908,"parent_record_offset":48884,"ancestor_record_offset":47456,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$46::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":576,"record_offset":48732,"parent_record_offset":48704,"ancestor_record_offset":47456,"inlinee":14416,"source_line":2924,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":576,"record_offset":48796,"parent_record_offset":48760,"ancestor_record_offset":47456,"inlinee":6104,"source_line":1154,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":576,"record_offset":48760,"parent_record_offset":48732,"ancestor_record_offset":47456,"inlinee":12295,"source_line":129,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":576,"record_offset":48824,"parent_record_offset":48796,"ancestor_record_offset":47456,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::init_static_render_args::init_minimal_startup_static_render_args
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8F37
// 首条位置: {"module":1671,"inlinee":36663}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_by_name
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/texture_library.rs
//     evidence={"module":1671,"record_offset":22184,"parent_record_offset":22092,"ancestor_record_offset":22092,"inlinee":12354,"source_line":298,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":1671,"record_offset":22128,"parent_record_offset":22092,"ancestor_record_offset":22092,"inlinee":27991,"source_line":42,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::binding_set::BindingSet::update
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/binding_set.rs
//     evidence={"module":1671,"record_offset":22240,"parent_record_offset":22092,"ancestor_record_offset":22092,"inlinee":27992,"source_line":406,"archive":"_pdb/C/modules/01671.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1671,"record_offset":22152,"parent_record_offset":22128,"ancestor_record_offset":22092,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/01671.jsonl.gz"}
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
// 完整结构及原始类型记录: init_static_render_args.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: init_static_render_args.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: init_static_render_args.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 282; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform>,ref$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::SplashParticle> >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::SplashParticle> >,tuple$<> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<tuple$<ref_mut$<country_core::components::drawable::MaterialArgs>,ref$<country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >,ref$<utils::components::transform::Transform>,ref$<country_core::components::SplashParticle> >,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> > >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,bevy_ecs::system::system_param::Local<country_core::systems::ui_move_shapes::MoveShapeCache>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> > >,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemShapeWasMoved>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>,bevy_ecs::change_detection::Res<country_core::resources::shape_intersections::ShapeIntersections>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> > >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >", "tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >", "bevy_ecs::system::system_param::Local<country_core::systems::ui_move_shapes::MoveShapeCache>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound> >", "tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>,bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> > >", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemShapeWasMoved>", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >", "bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>", "bevy_ecs::change_detection::Res<country_core::resources::shape_intersections::ShapeIntersections>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::AudioCursorMoved>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::ui_move_shapes::MoveShapeCache>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesinAreaCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<enum2$<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter> >
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemShapeWasMoved>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::mod_key::Modifiers>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::shape_intersections::ShapeIntersections>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::building_audio::PlayGroupBuildingMoveAudio>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::building_audio::BuildingAudioState>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::building_audio::PlayBuildingAudio>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::building_audio::PlayGroupBuildingMoveAudio>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::building_audio::BuildingAudioState>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::building_audio::PlayBuildingAudio>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::audio::building_audio::PlayGroupBuildingMoveAudio>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::building_audio::BuildingAudioState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::startup::install_mod_clutter::ModClutterInstalls>,bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::InstallModClutterCmd>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::UninstallModClutterCmd>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::startup::install_mod_clutter::ModClutterInstalls>", "bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::InstallModClutterCmd>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>", "bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::UninstallModClutterCmd>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::install_mod_clutter::ModClutterInstalls>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::InstallModClutterCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::startup::install_mod_clutter::UninstallModClutterCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>,bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>,bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>", "bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>", "bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>", "bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::clutter_registry::ClutterRegistry>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::uber_materials::UberMaterialPool>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::clutter_registry::RefreshClutterVisualsCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::steam::missing_mods::MissingMods>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::clutter_diagnostics::ClutterBuildDiagnostics>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::startup::init_static_render_args::init_minimal_startup_static_render_args
// 信息 ID: 09ed99656891903ea281b5a5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 52a61717991191b755af9e07: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>)
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>)>; module=1671, inline_record=22092; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>)> >; module=1698, inline_record=3379468; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::init_static_render_args::init_static_render_args
// 信息 ID: 2bfc060693a63a9161f7248e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 52a61717991191b755af9e07: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>)> @ 0x17deabf -> 0xb94950 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>)> > @ 0x195fbd8 -> 0xb94950 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
