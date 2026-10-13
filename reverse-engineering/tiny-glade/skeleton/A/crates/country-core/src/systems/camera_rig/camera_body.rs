// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/camera_rig/camera_body.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 436de1d819a47211417cdb1576e50ff26cf985d02b08bae9f24c79a1a2b580f7
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: camera_body.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 391, 415, 1202
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::camera_rig::camera_body::pitch_yaw_rot
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x56D7
// 首条位置: {"module":391,"inlinee":22231}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::to_radians
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":391,"record_offset":15924,"parent_record_offset":15896,"ancestor_record_offset":15896,"inlinee":22232,"source_line":903,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_euler
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":391,"record_offset":15980,"parent_record_offset":15896,"ancestor_record_offset":15896,"inlinee":22233,"source_line":185,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=41
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":391,"record_offset":16612,"parent_record_offset":16332,"ancestor_record_offset":15896,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":391,"record_offset":17464,"parent_record_offset":17232,"ancestor_record_offset":15896,"inlinee":21870,"source_line":100,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=83
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":391,"record_offset":16444,"parent_record_offset":16332,"ancestor_record_offset":15896,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=58
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":391,"record_offset":16388,"parent_record_offset":16332,"ancestor_record_offset":15896,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sqrt_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":391,"record_offset":17436,"parent_record_offset":17232,"ancestor_record_offset":15896,"inlinee":22244,"source_line":124,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] glam::euler::impl$10::new_quat
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/euler.rs
//     evidence={"module":391,"record_offset":16004,"parent_record_offset":15980,"ancestor_record_offset":15896,"inlinee":22235,"source_line":252,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] glam::euler::impl$10::new_quat::rot_y
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/euler.rs
//     evidence={"module":391,"record_offset":16032,"parent_record_offset":16004,"ancestor_record_offset":15896,"inlinee":22237,"source_line":259,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":391,"record_offset":16092,"parent_record_offset":16068,"ancestor_record_offset":15896,"inlinee":21674,"source_line":177,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_x
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":391,"record_offset":16184,"parent_record_offset":16032,"ancestor_record_offset":15896,"inlinee":22238,"source_line":161,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_y
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":391,"record_offset":16068,"parent_record_offset":16032,"ancestor_record_offset":15896,"inlinee":21673,"source_line":169,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":391,"record_offset":16300,"parent_record_offset":16184,"ancestor_record_offset":15896,"inlinee":14673,"source_line":72,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_quat
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":391,"record_offset":16332,"parent_record_offset":16032,"ancestor_record_offset":15896,"inlinee":22239,"source_line":675,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::normalize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":391,"record_offset":17208,"parent_record_offset":16004,"ancestor_record_offset":15896,"inlinee":22240,"source_line":485,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::normalize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":391,"record_offset":17232,"parent_record_offset":17208,"ancestor_record_offset":15896,"inlinee":22241,"source_line":522,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot4_in_x
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":391,"record_offset":17288,"parent_record_offset":17260,"ancestor_record_offset":15896,"inlinee":22243,"source_line":52,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot4_into_m128
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":391,"record_offset":17260,"parent_record_offset":17232,"ancestor_record_offset":15896,"inlinee":22242,"source_line":77,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":391,"record_offset":16140,"parent_record_offset":16116,"ancestor_record_offset":15896,"inlinee":13074,"source_line":713,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":391,"record_offset":16116,"parent_record_offset":16092,"ancestor_record_offset":15896,"inlinee":21675,"source_line":915,"archive":"_pdb/A/modules/00391.jsonl.gz"}
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
// 完整结构及原始类型记录: camera_body.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: camera_body.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: camera_body.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::camera_rig::camera_body::pitch_yaw_rot
// 信息 ID: 9c91ad965b18d5e62f814159
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::camera_rig::MainCamera::load_parameters; module=391, inline_record=15896; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=1202, inline_record=31456; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=1202, inline_record=33380; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=1202, inline_record=38612; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=1202, inline_record=41508; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=1202, inline_record=45500; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::camera_rig::normal::impl$1::update; module=1202, inline_record=46852; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
