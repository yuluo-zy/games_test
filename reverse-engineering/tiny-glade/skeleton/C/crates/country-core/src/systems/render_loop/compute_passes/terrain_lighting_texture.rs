// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/render_loop/compute_passes/terrain_lighting_texture.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 3f4a8ee49b6e14a76897acee70a0ac527703ad666e9b899de706ccf90a54e9d8
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: terrain_lighting_texture.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 544
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use rhapsody_core; // namespace candidate
//   package identities: [{"name":"rhapsody-core","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use typed_index_collections; // namespace candidate
//   package identities: [{"name":"typed-index-collections","source":"registry+https://github.com/rust-lang/crates.io-index","version":"3.4.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::terrain_lighting_texture::compute_terrain_lighting_texture
// 记录形式: compiled_procedure; instances=1
// 源码行: 8
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":544,"record_offset":85680,"rva":11065856,"va":5379774976}
// 编译实例 RVAs: 0xa8da00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..texture..Texture$u20$as$u20$rhapsody..rg..resource..GraphResource$GT$::init_as_debug_garbage
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3697579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379774976,"raw_hex":"e8ab6b3800","size":5,"soft_fail":false,"target_va":5383472768,"va":5379775184,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14176eed0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_lighting_texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13505269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379774976,"raw_hex":"e8f512ce00","size":5,"soft_fail":false,"target_va":5393280720,"va":5379775446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141770e40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: terrain_lighting_texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13513483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379774976,"raw_hex":"e80b33ce00","size":5,"soft_fail":false,"target_va":5393288768,"va":5379775280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14177a260
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: terrain_lighting_texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13551346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379774976,"raw_hex":"e8f2c6ce00","size":5,"soft_fail":false,"target_va":5393326688,"va":5379775337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36595840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379774976,"raw_hex":"e880682e02","size":5,"soft_fail":false,"target_va":5416370864,"va":5379775019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::api::GraphApi::create_raw_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3773007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379774976,"raw_hex":"e84f923900","size":5,"soft_fail":false,"target_va":5383548144,"va":5379775132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3761274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379774976,"raw_hex":"e87a643900","size":5,"soft_fail":false,"target_va":5383536672,"va":5379775393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3763834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379774976,"raw_hex":"e87a6e3900","size":5,"soft_fail":false,"target_va":5383539152,"va":5379775313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_core::reduction::generate_mips
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3641311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379774976,"raw_hex":"e8df8f3700","size":5,"soft_fail":false,"target_va":5383416720,"va":5379775404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::label
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":544,"record_offset":85876,"parent_record_offset":85680,"ancestor_record_offset":85680,"inlinee":16150,"source_line":436,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":544,"record_offset":86024,"parent_record_offset":85992,"ancestor_record_offset":85680,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":544,"record_offset":86072,"parent_record_offset":86048,"ancestor_record_offset":85680,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":544,"record_offset":86048,"parent_record_offset":86024,"ancestor_record_offset":85680,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":544,"record_offset":85992,"parent_record_offset":85956,"ancestor_record_offset":85680,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":544,"record_offset":85932,"parent_record_offset":85908,"ancestor_record_offset":85680,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":544,"record_offset":85956,"parent_record_offset":85932,"ancestor_record_offset":85680,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":544,"record_offset":86112,"parent_record_offset":85956,"ancestor_record_offset":85680,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::terrain_lighting_texture::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":544,"record_offset":86268,"parent_record_offset":86232,"ancestor_record_offset":85680,"inlinee":33772,"source_line":37380,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::compute_passes::terrain_lighting_texture::compute_terrain_lighting_texture::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/terrain_lighting_texture.rs
//     evidence={"module":544,"record_offset":86152,"parent_record_offset":85876,"ancestor_record_offset":85680,"inlinee":33770,"source_line":9,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":544,"record_offset":86232,"parent_record_offset":86152,"ancestor_record_offset":85680,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::impl$6::create
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":544,"record_offset":86192,"parent_record_offset":86152,"ancestor_record_offset":85680,"inlinee":12324,"source_line":1280,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":544,"record_offset":86300,"parent_record_offset":86152,"ancestor_record_offset":85680,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] typed_index_collections::vec::TiVec::push
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"typed-index-collections","version":"3.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/typed-index-collections-3.4.0/src/vec.rs
//     evidence={"module":544,"record_offset":85908,"parent_record_offset":85876,"ancestor_record_offset":85680,"inlinee":14011,"source_line":489,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::terrain_lighting_texture::compute_terrain_lighting_texture::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x83EA
// 首条位置: {"module":544,"inlinee":33770}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":544,"record_offset":86232,"parent_record_offset":86152,"ancestor_record_offset":86152,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::impl$6::create
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":544,"record_offset":86192,"parent_record_offset":86152,"ancestor_record_offset":86152,"inlinee":12324,"source_line":1280,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":544,"record_offset":86300,"parent_record_offset":86152,"ancestor_record_offset":86152,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::terrain_lighting_texture::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":544,"record_offset":86268,"parent_record_offset":86232,"ancestor_record_offset":86152,"inlinee":33772,"source_line":37380,"archive":"_pdb/C/modules/00544.jsonl.gz"}
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
// 完整结构及原始类型记录: terrain_lighting_texture.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: terrain_lighting_texture.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/compute_passes/terrain_lighting_texture.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: terrain_lighting_texture.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::terrain_lighting_texture::compute_terrain_lighting_texture
// [PDB Shader 类型/入口] {"shader_family":"country_core/terrain_lighting_texture","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::terrain_lighting_texture::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":544,"record_offset":86268,"parent_record_offset":86232,"ancestor_record_offset":85680,"inlinee":33772,"source_line":37380,"archive":"_pdb/C/modules/00544.jsonl.gz"}}
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::compute_passes::terrain_lighting_texture::compute_terrain_lighting_texture::closure$0
// [PDB Shader 类型/入口] {"shader_family":"country_core/terrain_lighting_texture","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::terrain_lighting_texture::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":544,"record_offset":86268,"parent_record_offset":86232,"ancestor_record_offset":86152,"inlinee":33772,"source_line":37380,"archive":"_pdb/C/modules/00544.jsonl.gz"}}
// [Shader 映射] {"shader_id":"b54780c643251b452c201a82","relative_path":"country-core/compiled/terrain_lighting_texture.precomp","sha256":"92a6f75c4b1c357e5766fa7e1f59f377ba0d371c3f01cc2998909bf76464bec2","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::terrain_lighting_texture::compute_terrain_lighting_texture","hint":{"shader_family":"country_core/terrain_lighting_texture","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::terrain_lighting_texture::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":544,"record_offset":86268,"parent_record_offset":86232,"ancestor_record_offset":85680,"inlinee":33772,"source_line":37380,"archive":"_pdb/C/modules/00544.jsonl.gz"}},"shader_uri":"country_core/terrain_lighting_texture:main"}
// [Shader 映射] {"shader_id":"b54780c643251b452c201a82","relative_path":"country-core/compiled/terrain_lighting_texture.precomp","sha256":"92a6f75c4b1c357e5766fa7e1f59f377ba0d371c3f01cc2998909bf76464bec2","status":"PDB_typed_shader_identity_matches_artifact_header; actual runtime binding/permutation unknown","host_symbol":"country_core::systems::render_loop::compute_passes::terrain_lighting_texture::compute_terrain_lighting_texture::closure$0","hint":{"shader_family":"country_core/terrain_lighting_texture","entry":"unknown","basis":"PDB_shader_permutation_type","symbol":"country_core::shader::terrain_lighting_texture::impl$3::to_shader_permutation_key","relationship":"pdb_inline_transitive","evidence":{"module":544,"record_offset":86268,"parent_record_offset":86232,"ancestor_record_offset":86152,"inlinee":33772,"source_line":37380,"archive":"_pdb/C/modules/00544.jsonl.gz"}},"shader_uri":"country_core/terrain_lighting_texture:main"}
// [Shader 声明接口；Pass 实际绑定见映射状态] country-core/compiled/terrain_lighting_texture.precomp
// Shader stages / entries / workgroups: {"entries":[{"execution_model":"GLCompute","function_id":2,"entry_name":"main","interface_ids":[70]}],"execution_modes":[{"entry_id":2,"mode":"LocalSize","operands":[8,8,1],"operand_kind":"literal"}]}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":41,"name":"globalParams","storage_class":"Uniform","type":{"id":40,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,39]},"decorations":[],"pointee":{"id":39,"name":"GlobalParams_natural","record":{"opcode":"OpTypeStruct","operands":[11]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"img_output_dims","type_id":11,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":0,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":49,"name":"color_library.data","storage_class":"Uniform","type":{"id":48,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,47]},"decorations":[],"pointee":{"id":47,"name":"ColorLibraryData_natural","record":{"opcode":"OpTypeStruct","operands":[14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"temp_color_1","type_id":14,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"temp_color_2","type_id":14,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"temp_color_3","type_id":14,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"roof_color_1","type_id":14,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"roof_color_2","type_id":14,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"sky_low_day","type_id":14,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"sky_high_day","type_id":14,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"sun_tint_day","type_id":14,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"sky_low_dusk","type_id":14,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"sky_high_dusk","type_id":14,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":10,"name":"sun_tint_dusk","type_id":14,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":11,"name":"indirect_light_tint","type_id":14,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":12,"name":"fence_color_1","type_id":14,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":13,"name":"fence_color_2","type_id":14,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":14,"name":"flower_color_1","type_id":14,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":15,"name":"flower_color_2","type_id":14,"decorations":[{"decoration":"Offset","operands":[240]}]},{"index":16,"name":"canopy_color_1","type_id":14,"decorations":[{"decoration":"Offset","operands":[256]}]},{"index":17,"name":"canopy_color_2","type_id":14,"decorations":[{"decoration":"Offset","operands":[272]}]},{"index":18,"name":"grass_1","type_id":14,"decorations":[{"decoration":"Offset","operands":[288]}]},{"index":19,"name":"grass_2","type_id":14,"decorations":[{"decoration":"Offset","operands":[304]}]},{"index":20,"name":"path","type_id":14,"decorations":[{"decoration":"Offset","operands":[320]}]},{"index":21,"name":"sky_base","type_id":14,"decorations":[{"decoration":"Offset","operands":[336]}]},{"index":22,"name":"sky_horizon","type_id":14,"decorations":[{"decoration":"Offset","operands":[352]}]},{"index":23,"name":"sky_clouds_1","type_id":14,"decorations":[{"decoration":"Offset","operands":[368]}]},{"index":24,"name":"sky_clouds_2","type_id":14,"decorations":[{"decoration":"Offset","operands":[384]}]},{"index":25,"name":"terrain_1","type_id":14,"decorations":[{"decoration":"Offset","operands":[400]}]},{"index":26,"name":"terrain_2","type_id":14,"decorations":[{"decoration":"Offset","operands":[416]}]},{"index":27,"name":"terrain_flattened","type_id":14,"decorations":[{"decoration":"Offset","operands":[432]}]},{"index":28,"name":"wall_1","type_id":14,"decorations":[{"decoration":"Offset","operands":[448]}]},{"index":29,"name":"wall_2","type_id":14,"decorations":[{"decoration":"Offset","operands":[464]}]},{"index":30,"name":"wall_moss","type_id":14,"decorations":[{"decoration":"Offset","operands":[480]}]},{"index":31,"name":"wall_bricks","type_id":14,"decorations":[{"decoration":"Offset","operands":[496]}]},{"index":32,"name":"ivy_branch","type_id":14,"decorations":[{"decoration":"Offset","operands":[512]}]},{"index":33,"name":"ivy_leaf_1","type_id":14,"decorations":[{"decoration":"Offset","operands":[528]}]},{"index":34,"name":"ivy_leaf_2","type_id":14,"decorations":[{"decoration":"Offset","operands":[544]}]},{"index":35,"name":"terrain_highlight","type_id":14,"decorations":[{"decoration":"Offset","operands":[560]}]},{"index":36,"name":"night_tint","type_id":14,"decorations":[{"decoration":"Offset","operands":[576]}]}]}},"descriptor_set":6,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[6]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":105,"name":"terrain_heightmap","storage_class":"Uniform","type":{"id":104,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,102]},"decorations":[],"pointee":{"id":102,"name":"TerrainHeightmap_natural","record":{"opcode":"OpTypeStruct","operands":[11,103,103,13,14]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"terrain_heightmap_dims","type_id":11,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"bg_terrain_heightmap","type_id":103,"decorations":[{"decoration":"Offset","operands":[8]}]},{"index":2,"name":"bg_terrain_fog_height","type_id":103,"decorations":[{"decoration":"Offset","operands":[12]}]},{"index":3,"name":"bg_terrain_height_bounds_ws","type_id":13,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":4,"name":"bg_water_footprint_ws","type_id":14,"decorations":[{"decoration":"Offset","operands":[24]}]}]}},"descriptor_set":19,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[19]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":142,"name":"terrain_heightmap.terrain_heightmap","storage_class":"UniformConstant","type":{"id":141,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,139]},"decorations":[],"pointee":{"id":139,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[5,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":19,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[19]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":146,"name":"samplers.lnc_iso","storage_class":"UniformConstant","type":{"id":145,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,143]},"decorations":[],"pointee":{"id":143,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":5,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[5]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":158,"name":"visual_terrain_heightmap","storage_class":"Uniform","type":{"id":157,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,156]},"decorations":[],"pointee":{"id":156,"name":"VisualTerrainHeightmap_natural","record":{"opcode":"OpTypeStruct","operands":[11]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"visual_terrain_heightmap_dims","type_id":11,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":20,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[20]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":173,"name":"samplers.lnc","storage_class":"UniformConstant","type":{"id":145,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,143]},"decorations":[],"pointee":{"id":143,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":175,"name":"visual_terrain_heightmap.visual_terrain_heightmap","storage_class":"UniformConstant","type":{"id":141,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,139]},"decorations":[],"pointee":{"id":139,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[5,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":20,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[20]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":263,"name":"frame_constants","storage_class":"Uniform","type":{"id":262,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,261]},"decorations":[],"pointee":{"id":261,"name":"FrameConstants_natural","record":{"opcode":"OpTypeStruct","operands":[14,14,14,14,14,14,14,14,5,5,5,10,11,10,5,6,5,6,10,6,10,6,5,14,5,10,11,5,10,10,10,6,5,10,10,10,10,10,5,5,5,14,6,5]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_color","type_id":14,"decorations":[{"decoration":"Offset","operands":[0]}]},{"index":1,"name":"sky_low_color","type_id":14,"decorations":[{"decoration":"Offset","operands":[16]}]},{"index":2,"name":"sky_high_color","type_id":14,"decorations":[{"decoration":"Offset","operands":[32]}]},{"index":3,"name":"fog_tint_thickness","type_id":14,"decorations":[{"decoration":"Offset","operands":[48]}]},{"index":4,"name":"fake_light_color","type_id":14,"decorations":[{"decoration":"Offset","operands":[64]}]},{"index":5,"name":"fake_light_dir","type_id":14,"decorations":[{"decoration":"Offset","operands":[80]}]},{"index":6,"name":"fake_light2_color","type_id":14,"decorations":[{"decoration":"Offset","operands":[96]}]},{"index":7,"name":"fake_light2_dir","type_id":14,"decorations":[{"decoration":"Offset","operands":[112]}]},{"index":8,"name":"time_since_startup_secs","type_id":5,"decorations":[{"decoration":"Offset","operands":[128]}]},{"index":9,"name":"delta_time_seconds","type_id":5,"decorations":[{"decoration":"Offset","operands":[132]}]},{"index":10,"name":"ev_mult","type_id":5,"decorations":[{"decoration":"Offset","operands":[136]}]},{"index":11,"name":"glade_theme_id","type_id":10,"decorations":[{"decoration":"Offset","operands":[140]}]},{"index":12,"name":"world_render_resolution","type_id":11,"decorations":[{"decoration":"Offset","operands":[144]}]},{"index":13,"name":"game_time_sec","type_id":10,"decorations":[{"decoration":"Offset","operands":[152]}]},{"index":14,"name":"game_time_frac","type_id":5,"decorations":[{"decoration":"Offset","operands":[156]}]},{"index":15,"name":"editing_highlight_color","type_id":6,"decorations":[{"decoration":"Offset","operands":[160]}]},{"index":16,"name":"time_of_day","type_id":5,"decorations":[{"decoration":"Offset","operands":[172]}]},{"index":17,"name":"dir_to_sun","type_id":6,"decorations":[{"decoration":"Offset","operands":[176]}]},{"index":18,"name":"camera_idx","type_id":10,"decorations":[{"decoration":"Offset","operands":[188]}]},{"index":19,"name":"dir_to_moon","type_id":6,"decorations":[{"decoration":"Offset","operands":[192]}]},{"index":20,"name":"render_overrides","type_id":10,"decorations":[{"decoration":"Offset","operands":[204]}]},{"index":21,"name":"dir_to_primary_directional_light","type_id":6,"decorations":[{"decoration":"Offset","operands":[208]}]},{"index":22,"name":"main_view_lod_scale","type_id":5,"decorations":[{"decoration":"Offset","operands":[220]}]},{"index":23,"name":"main_view_eye_position","type_id":14,"decorations":[{"decoration":"Offset","operands":[224]}]},{"index":24,"name":"fog_relative_extinction","type_id":5,"decorations":[{"decoration":"Offset","operands":[240]}]},{"index":25,"name":"highlighted_transient_stair_edge_id","type_id":10,"decorations":[{"decoration":"Offset","operands":[244]}]},{"index":26,"name":"swapchain_dims","type_id":11,"decorations":[{"decoration":"Offset","operands":[248]}]},{"index":27,"name":"swapchain_aspect_ratio","type_id":5,"decorations":[{"decoration":"Offset","operands":[256]}]},{"index":28,"name":"is_photomode","type_id":10,"decorations":[{"decoration":"Offset","operands":[260]}]},{"index":29,"name":"is_topdown_mode","type_id":10,"decorations":[{"decoration":"Offset","operands":[264]}]},{"index":30,"name":"is_firstperson_mode","type_id":10,"decorations":[{"decoration":"Offset","operands":[268]}]},{"index":31,"name":"brush_position","type_id":6,"decorations":[{"decoration":"Offset","operands":[272]}]},{"index":32,"name":"eraser_size_multiplier","type_id":5,"decorations":[{"decoration":"Offset","operands":[284]}]},{"index":33,"name":"roof_highlighted_slot_id","type_id":10,"decorations":[{"decoration":"Offset","operands":[288]}]},{"index":34,"name":"roof_highlight_mode","type_id":10,"decorations":[{"decoration":"Offset","operands":[292]}]},{"index":35,"name":"wall_highlighted_slot_id","type_id":10,"decorations":[{"decoration":"Offset","operands":[296]}]},{"index":36,"name":"wall_highlight_mode","type_id":10,"decorations":[{"decoration":"Offset","operands":[300]}]},{"index":37,"name":"highlight_bricks_to_delete","type_id":10,"decorations":[{"decoration":"Offset","operands":[304]}]},{"index":38,"name":"wind_strength","type_id":5,"decorations":[{"decoration":"Offset","operands":[308]}]},{"index":39,"name":"tree_canopy_cone_angle_mult","type_id":5,"decorations":[{"decoration":"Offset","operands":[312]}]},{"index":40,"name":"tree_canopy_max_cone_cull_dist_range","type_id":5,"decorations":[{"decoration":"Offset","operands":[316]}]},{"index":41,"name":"debug_tweakers","type_id":14,"decorations":[{"decoration":"Offset","operands":[320]}]},{"index":42,"name":"ground_fog","type_id":6,"decorations":[{"decoration":"Offset","operands":[336]}]},{"index":43,"name":"moonswap","type_id":5,"decorations":[{"decoration":"Offset","operands":[348]}]}]}},"descriptor_set":4,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[4]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":431,"name":"resource_handles","storage_class":"UniformConstant","type":{"id":430,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,429]},"decorations":[],"pointee":{"id":429,"name":"unknown","record":{"opcode":"OpTypeRuntimeArray","operands":[139]},"decorations":[],"element":{"id":139,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[5,1,2,0,0,1,0]},"decorations":[]}}},"descriptor_set":1,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[1]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":696,"name":"current_view","storage_class":"Uniform","type":{"id":695,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,693]},"decorations":[],"pointee":{"id":693,"name":"ViewConstants_natural","record":{"opcode":"OpTypeStruct","operands":[694,694,694,694,694,694,694,694,694,694,694,694,694,13,13,13,13,694,694,14,5,5,5,5]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"view_to_clip","type_id":694,"decorations":[{"decoration":"Offset","operands":[0]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":1,"name":"clip_to_view","type_id":694,"decorations":[{"decoration":"Offset","operands":[64]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":2,"name":"view_to_sample","type_id":694,"decorations":[{"decoration":"Offset","operands":[128]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":3,"name":"sample_to_view","type_id":694,"decorations":[{"decoration":"Offset","operands":[192]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":4,"name":"world_to_view","type_id":694,"decorations":[{"decoration":"Offset","operands":[256]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":5,"name":"view_to_world","type_id":694,"decorations":[{"decoration":"Offset","operands":[320]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":6,"name":"world_to_sample","type_id":694,"decorations":[{"decoration":"Offset","operands":[384]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":7,"name":"clip_to_prev_clip","type_id":694,"decorations":[{"decoration":"Offset","operands":[448]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":8,"name":"prev_clip_to_clip","type_id":694,"decorations":[{"decoration":"Offset","operands":[512]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":9,"name":"prev_view_to_prev_clip","type_id":694,"decorations":[{"decoration":"Offset","operands":[576]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":10,"name":"prev_clip_to_prev_view","type_id":694,"decorations":[{"decoration":"Offset","operands":[640]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":11,"name":"prev_world_to_prev_view","type_id":694,"decorations":[{"decoration":"Offset","operands":[704]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":12,"name":"prev_view_to_prev_world","type_id":694,"decorations":[{"decoration":"Offset","operands":[768]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":13,"name":"sample_offset_pixels","type_id":13,"decorations":[{"decoration":"Offset","operands":[832]}]},{"index":14,"name":"sample_offset_clip","type_id":13,"decorations":[{"decoration":"Offset","operands":[840]}]},{"index":15,"name":"prev_sample_offset_pixels","type_id":13,"decorations":[{"decoration":"Offset","operands":[848]}]},{"index":16,"name":"prev_sample_offset_clip","type_id":13,"decorations":[{"decoration":"Offset","operands":[856]}]},{"index":17,"name":"pad0","type_id":694,"decorations":[{"decoration":"Offset","operands":[864]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":18,"name":"pad1","type_id":694,"decorations":[{"decoration":"Offset","operands":[928]},{"decoration":"RowMajor","operands":[]},{"decoration":"MatrixStride","operands":[16]}]},{"index":19,"name":"pad2","type_id":14,"decorations":[{"decoration":"Offset","operands":[992]}]},{"index":20,"name":"pad3","type_id":5,"decorations":[{"decoration":"Offset","operands":[1008]}]},{"index":21,"name":"pad4","type_id":5,"decorations":[{"decoration":"Offset","operands":[1012]}]},{"index":22,"name":"pad5","type_id":5,"decorations":[{"decoration":"Offset","operands":[1016]}]},{"index":23,"name":"mip_bias","type_id":5,"decorations":[{"decoration":"Offset","operands":[1020]}]}]}},"descriptor_set":21,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[21]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":776,"name":"csm","storage_class":"Uniform","type":{"id":775,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,774]},"decorations":[],"pointee":{"id":774,"name":"CascadedShadows_natural","record":{"opcode":"OpTypeStruct","operands":[11]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"sun_shadow_map_dims","type_id":11,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":23,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"DescriptorSet","operands":[23]},{"decoration":"Binding","operands":[0]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":796,"name":"csm_setup.sun_view","storage_class":"Uniform","type":{"id":795,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,793]},"decorations":[],"pointee":{"id":793,"name":"StructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[794]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":794,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":22,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"declared_read_only","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[22]},{"decoration":"NonWritable","operands":[]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet","NonWritable"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":808,"name":"frame_counter","storage_class":"Uniform","type":{"id":807,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,806]},"decorations":[],"pointee":{"id":806,"name":"FrameCounter_natural","record":{"opcode":"OpTypeStruct","operands":[10]},"decorations":[{"decoration":"Block","operands":[]}],"members":[{"index":0,"name":"index","type_id":10,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":3,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"uniform_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[3]}],"access_qualifiers":["Binding","Block","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":839,"name":"blue_noise_textures.uniform01","storage_class":"UniformConstant","type":{"id":141,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,139]},"decorations":[],"pointee":{"id":139,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[5,1,2,0,0,1,0]},"decorations":[]}},"descriptor_set":27,"binding":0,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[0]},{"decoration":"DescriptorSet","operands":[27]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":871,"name":"csm_setup.cascade_splits_buf","storage_class":"Uniform","type":{"id":870,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[2,868]},"decorations":[],"pointee":{"id":868,"name":"StructuredBuffer","record":{"opcode":"OpTypeStruct","operands":[869]},"decorations":[{"decoration":"BufferBlock","operands":[]}],"members":[{"index":0,"name":"__member0","type_id":869,"decorations":[{"decoration":"Offset","operands":[0]}]}]}},"descriptor_set":22,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"declared_read_only","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[22]},{"decoration":"NonWritable","operands":[]}],"access_qualifiers":["Binding","BufferBlock","DescriptorSet","NonWritable"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":1066,"name":"csm.sun_shadow_map","storage_class":"UniformConstant","type":{"id":1065,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,1063]},"decorations":[],"pointee":{"id":1063,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[5,1,2,1,0,1,0]},"decorations":[]}},"descriptor_set":23,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[23]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":1085,"name":"csm.sun_shadow_stencil_map","storage_class":"UniformConstant","type":{"id":1084,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,1082]},"decorations":[],"pointee":{"id":1082,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[10,1,2,1,0,1,0]},"decorations":[]}},"descriptor_set":23,"binding":2,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[2]},{"decoration":"DescriptorSet","operands":[23]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":1214,"name":"samplers.shadow_linear","storage_class":"UniformConstant","type":{"id":145,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,143]},"decorations":[],"pointee":{"id":143,"name":"unknown","record":{"opcode":"OpTypeSampler","operands":[]},"decorations":[]}},"descriptor_set":2,"binding":11,"location":"unknown","builtin":"unknown","declared_direction":"sampled_input","decorations":[{"decoration":"Binding","operands":[11]},{"decoration":"DescriptorSet","operands":[2]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":1655,"name":"img_output","storage_class":"UniformConstant","type":{"id":1654,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[0,1652]},"decorations":[],"pointee":{"id":1652,"name":"unknown","record":{"opcode":"OpTypeImage","operands":[5,1,2,0,0,2,8]},"decorations":[]}},"descriptor_set":0,"binding":1,"location":"unknown","builtin":"unknown","declared_direction":"unknown_read_write_direction","decorations":[{"decoration":"Binding","operands":[1]},{"decoration":"DescriptorSet","operands":[0]}],"access_qualifiers":["Binding","DescriptorSet"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [声明 I/O；不是已确认的渲染图资源边] {"variable_id":70,"name":"unknown","storage_class":"Input","type":{"id":69,"name":"unknown","record":{"opcode":"OpTypePointer","operands":[1,67]},"decorations":[],"pointee":{"id":67,"name":"unknown","record":{"opcode":"OpTypeVector","operands":[10,3]},"decorations":[]}},"descriptor_set":"unknown","binding":"unknown","location":"unknown","builtin":28,"declared_direction":"stage_input","decorations":[{"decoration":"BuiltIn","operands":[28]}],"access_qualifiers":["BuiltIn"],"runtime_resource_handle":"unknown","actually_used_by_entry":"unknown; declaration only"}
// [父函数静态调用位置] {"caller_entry_rva":5678848,"caller_aliases":{"alias_group_rva":5678848,"count":1,"display_examples":["tiny_glade::render_loop::render_world"],"complete_catalog":"_pdb/C/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5686200,"position_among_mapped_render_targets":47,"mapped_render_target_count":74,"previous_callsite":5685987,"next_callsite":5686270,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/shadow_pass.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/visibility_queries.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: terrain_lighting_texture.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::terrain_lighting_texture::compute_terrain_lighting_texture
// 信息 ID: 0d6355dd1bd50523876b25c7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x56c3b8 -> 0xa8da00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::terrain_lighting_texture::compute_terrain_lighting_texture::closure$0
// 信息 ID: 656fa7aa7f1b3568586d1be3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::compute_passes::terrain_lighting_texture::compute_terrain_lighting_texture", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::terrain_lighting_texture::compute_terrain_lighting_texture; module=544, inline_record=86152; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
