// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/render/nani/capacity.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 8473e9da8131d606042437287f220ade99da8cc929a714632e28aa163ab5a1c1
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: capacity.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 512, 513, 522, 526, 1806
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::capacity::impl$0::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x73F4
// 首条位置: {"module":522,"inlinee":29684}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorldSubsetKind::default_max_instances
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":522,"record_offset":25948,"parent_record_offset":25908,"ancestor_record_offset":25908,"inlinee":29621,"source_line":353,"archive":"_pdb/C/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::GpuWorldSubsetKind::capacity_policy
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":522,"record_offset":25972,"parent_record_offset":25948,"ancestor_record_offset":25908,"inlinee":29622,"source_line":293,"archive":"_pdb/C/modules/00522.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::capacity::NaniSubsetCapacities::get
// 记录形式: inline_instance, inlinee_declaration; instances=14
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x73B8
// 首条位置: {"module":512,"inlinee":29624}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::capacity::NaniSubsetCapacities::set
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x740E
// 首条位置: {"module":512,"record_offset":5444,"rva":9755936,"va":5378465056}
// 编译实例 RVAs: 0x94dd20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38157048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378465056,"raw_hex":"e8f83a4602","size":5,"soft_fail":false,"target_va":5416622240,"va":5378465187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::nani::capacity::assert_valid_capacity
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/capacity.rs
//     evidence={"module":512,"record_offset":5680,"parent_record_offset":5444,"ancestor_record_offset":5444,"inlinee":29629,"source_line":77,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorldSubsetKind::capacity_policy
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":512,"record_offset":5600,"parent_record_offset":5444,"ancestor_record_offset":5444,"inlinee":29622,"source_line":293,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorldSubsetKind::max_instances_ceiling
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":512,"record_offset":5624,"parent_record_offset":5444,"ancestor_record_offset":5444,"inlinee":29627,"source_line":359,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::is_multiple_of
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":512,"record_offset":5712,"parent_record_offset":5680,"ancestor_record_offset":5444,"inlinee":9486,"source_line":3742,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::GpuWorldSubsetKind::capacity_policy
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":512,"record_offset":5648,"parent_record_offset":5624,"ancestor_record_offset":5444,"inlinee":29622,"source_line":293,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::capacity::assert_valid_capacity
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 77
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x73BD
// 首条位置: {"module":512,"inlinee":29629}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::num::impl$11::is_multiple_of
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":512,"record_offset":5712,"parent_record_offset":5680,"ancestor_record_offset":5680,"inlinee":9486,"source_line":3742,"archive":"_pdb/C/modules/00512.jsonl.gz"}
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
// 完整结构及原始类型记录: capacity.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::capacity::NaniSubsetCapacities
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 1
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: capacity.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: capacity.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_writer::MessageWriter<country_core::systems::backend::grow_grass::GrowGrass>,bevy_ecs::system::system_param::Local<country_core::systems::backend::grow_grass::platform::Timer>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::message::message_writer::MessageWriter<country_core::systems::backend::grow_grass::GrowGrass>", "bevy_ecs::system::system_param::Local<country_core::systems::backend::grow_grass::platform::Timer>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::backend::grow_grass::GrowGrass>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::backend::grow_grass::platform::Timer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >

// ------------------------------------------------------------------
// 函数: country_core::render::nani::capacity::NaniSubsetCapacities::get
// 信息 ID: 5c9db3b661f0964455b51ca3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::nani::buffers::nani_calculate_draw_list_params; module=512, inline_record=2852; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::create_instances<country_core::resources::placement_data::PlacementDataSsbo,country_core::systems::prefab::create_prefab_gpu_instances::closure_env$0>; module=513, inline_record=52340; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::create_instances_no_data<country_core::render::nani::systems::nani_maintain_instances::closure_env$1>; module=513, inline_record=60828; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::subset_capacity; module=522, inline_record=19772; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::apply_grown_capacity; module=522, inline_record=20772; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::startup_nani::startup_nani; module=526, inline_record=18884; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::startup_nani::startup_nani; module=526, inline_record=20640; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::startup_nani::nani_on_scene_load; module=526, inline_record=21068; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::capacity::NaniSubsetCapacities::set
// 信息 ID: a999b2d512265657493cf733
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::world::World::resource_scope<country_core::render::nani::gpu_world::GpuWorld,tuple$<>,country_core::render::nani::maintenance::nani_frame_boundary_maintenance::closure_env$1> @ 0x19ec4c6 -> 0x94dd20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::resource_scope<country_core::render::nani::gpu_world::GpuWorld,tuple$<>,country_core::render::nani::maintenance::nani_frame_boundary_maintenance::closure_env$1> @ 0x19ec54f -> 0x94dd20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::resource_scope<country_core::render::nani::gpu_world::GpuWorld,tuple$<>,country_core::render::nani::maintenance::nani_frame_boundary_maintenance::closure_env$1> @ 0x19ec5d8 -> 0x94dd20 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::apply_grown_capacity; module=522, inline_record=20800; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::capacity::assert_valid_capacity
// 信息 ID: eb140324325909bb01ca5d62
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::nani::capacity::NaniSubsetCapacities::set; module=512, inline_record=5680; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::apply_grown_capacity; module=522, inline_record=20904; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::startup_nani::startup_nani; module=526, inline_record=19028; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::startup_nani::nani_on_scene_load; module=526, inline_record=21212; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::capacity::impl$0::default
// 信息 ID: ff04f7acb956ec28c59ddd09
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::new; module=522, inline_record=25908; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
