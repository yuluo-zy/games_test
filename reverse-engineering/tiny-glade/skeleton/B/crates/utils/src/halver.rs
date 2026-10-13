// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/utils/src/halver.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 32db71b288197ef9612e59193a348fd99eb644c55153c08603b2b26c39c66c6d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: halver.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 757, 771, 2073, 3077, 3078
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use half; // namespace candidate
//   package identities: [{"name":"half","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.4.1"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::halver::impl$0::to_f16
// 记录形式: compiled_procedure; instances=1
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2073,"record_offset":2216,"rva":9150240,"va":5377859360}
// 编译实例 RVAs: 0x8b9f20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] std_detect::detect::cache::detect_and_initialize
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t33816800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377859360,"raw_hex":"e8e0000402","size":5,"soft_fail":false,"target_va":5411676464,"va":5377859659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] half::binary16::arch::x86::f32_to_f16_x86_f16c
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"half","version":"2.4.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tjmp\t3","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5377859360,"raw_hex":"e903000000","size":5,"soft_fail":false,"target_va":5377859696,"va":5377859688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] half::binary16::f16::from_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"half","version":"2.4.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16.rs
//     evidence={"module":2073,"record_offset":2332,"parent_record_offset":2216,"ancestor_record_offset":2216,"inlinee":27367,"source_line":65,"archive":"_pdb/B/modules/02073.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2073,"record_offset":2584,"parent_record_offset":2472,"ancestor_record_offset":2216,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/02073.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":2073,"record_offset":2528,"parent_record_offset":2504,"ancestor_record_offset":2216,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/02073.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":2073,"record_offset":2548,"parent_record_offset":2528,"ancestor_record_offset":2216,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/02073.jsonl.gz"}
//   [pdb_inline_transitive] half::binary16::arch::f32_to_f16
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"half","version":"2.4.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs
//     evidence={"module":2073,"record_offset":2368,"parent_record_offset":2332,"ancestor_record_offset":2216,"inlinee":27307,"source_line":64,"archive":"_pdb/B/modules/02073.jsonl.gz"}
//   [pdb_inline_transitive] half::binary16::arch::f32_to_f16_fallback
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"half","version":"2.4.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs
//     evidence={"module":2073,"record_offset":2652,"parent_record_offset":2368,"ancestor_record_offset":2216,"inlinee":27316,"source_line":482,"archive":"_pdb/B/modules/02073.jsonl.gz"}
//   [pdb_inline_transitive] std_detect::detect::arch::x86::__is_feature_detected::f16c
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\stdarch\crates\std_detect\src\detect\macros.rs
//     evidence={"module":2073,"record_offset":2416,"parent_record_offset":2368,"ancestor_record_offset":2216,"inlinee":27309,"source_line":197,"archive":"_pdb/B/modules/02073.jsonl.gz"}
//   [pdb_inline_transitive] std_detect::detect::cache::Cache::test
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\stdarch\crates\std_detect\src\detect\cache.rs
//     evidence={"module":2073,"record_offset":2504,"parent_record_offset":2472,"ancestor_record_offset":2216,"inlinee":27315,"source_line":105,"archive":"_pdb/B/modules/02073.jsonl.gz"}
//   [pdb_inline_transitive] std_detect::detect::cache::test
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\stdarch\crates\std_detect\src\detect\cache.rs
//     evidence={"module":2073,"record_offset":2472,"parent_record_offset":2444,"ancestor_record_offset":2216,"inlinee":27313,"source_line":212,"archive":"_pdb/B/modules/02073.jsonl.gz"}
//   [pdb_inline_transitive] std_detect::detect::cache::test::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\stdarch\crates\std_detect\src\detect\cache.rs
//     evidence={"module":2073,"record_offset":2608,"parent_record_offset":2584,"ancestor_record_offset":2216,"inlinee":27318,"source_line":222,"archive":"_pdb/B/modules/02073.jsonl.gz"}
//   [pdb_inline_transitive] std_detect::detect::check_for
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\stdarch\crates\std_detect\src\detect\mod.rs
//     evidence={"module":2073,"record_offset":2444,"parent_record_offset":2416,"ancestor_record_offset":2216,"inlinee":27311,"source_line":81,"archive":"_pdb/B/modules/02073.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::halver::impl$1::to_f16
// 记录形式: inlinee_declaration; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB03B
// 首条位置: {"module":771,"inlinee":45115}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::halver::impl$2::to_f16
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xAE95
// 首条位置: {"module":757,"inlinee":44693}
// 编译实例 RVAs: 0x1940140
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14193fe70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: halver.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395185984,"raw_hex":"e805fdffff","size":5,"soft_fail":false,"target_va":5395185264,"va":5395186022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::halver::impl$3::to_f16
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1613D
// 首条位置: {"module":2073,"record_offset":2856,"rva":26476928,"va":5395186048}
// 编译实例 RVAs: 0x1940180
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14193fe70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: halver.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395186048,"raw_hex":"e8cbfcffff","size":5,"soft_fail":false,"target_va":5395185264,"va":5395186080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::halver::impl$5::to_f16
// 记录形式: compiled_procedure; instances=1
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":757,"record_offset":376,"rva":13172656,"va":5381881776}
// 编译实例 RVAs: 0xc8ffb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14193fe70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: halver.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13303422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381881776,"raw_hex":"e87efeca00","size":5,"soft_fail":false,"target_va":5395185264,"va":5381881837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::to_array
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":757,"record_offset":496,"parent_record_offset":376,"ancestor_record_offset":376,"inlinee":44691,"source_line":121,"archive":"_pdb/B/modules/00757.jsonl.gz"}
//   [pdb_inline_direct] utils::halver::impl$2::to_f16
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/halver.rs
//     evidence={"module":757,"record_offset":520,"parent_record_offset":376,"ancestor_record_offset":376,"inlinee":44693,"source_line":28,"archive":"_pdb/B/modules/00757.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::halver::impl$6::to_f16
// 记录形式: compiled_procedure; instances=1
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3078,"record_offset":252,"rva":33583600,"va":5402292720}
// 编译实例 RVAs: 0x20071f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14193fe70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: halver.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7107503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402292720,"raw_hex":"e8518c93ff","size":5,"soft_fail":false,"target_va":5395185264,"va":5402292762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::sse2::vec4::Vec4::to_array
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":3078,"record_offset":372,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":106302,"source_line":138,"archive":"_pdb/B/modules/03078.jsonl.gz"}
//   [pdb_inline_direct] utils::halver::impl$3::to_f16
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/halver.rs
//     evidence={"module":3078,"record_offset":396,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":90429,"source_line":37,"archive":"_pdb/B/modules/03078.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::halver::impl$7::to_f16
// 记录形式: compiled_procedure; instances=1
// 源码行: 67
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3077,"record_offset":252,"rva":33583536,"va":5402292656}
// 编译实例 RVAs: 0x20071b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14193fe70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: halver.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7107439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402292656,"raw_hex":"e8918c93ff","size":5,"soft_fail":false,"target_va":5395185264,"va":5402292698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::to_array
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":3077,"record_offset":372,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":106297,"source_line":433,"archive":"_pdb/B/modules/03077.jsonl.gz"}
//   [pdb_inline_direct] utils::halver::impl$3::to_f16
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/halver.rs
//     evidence={"module":3077,"record_offset":396,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":90429,"source_line":37,"archive":"_pdb/B/modules/03077.jsonl.gz"}
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
// 完整结构及原始类型记录: halver.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: halver.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: halver.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: utils::halver::impl$0::to_f16
// 信息 ID: 0be8da702be2cff59891be5a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 38 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::prefab::create_prefab_gpu_instances @ 0x841c76 -> 0x8b9f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::pack_2x16f_uint @ 0xc90dfd -> 0x8b9f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] utils::pack_2x16f_uint @ 0xc90e08 -> 0x8b9f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<country_core::resources::rt::GpuRtTriangle,core::slice::iter::Iter<u32>,country_core::resources::rt::bvh::build_gpu_bvh::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::resources::rt::GpuRtTriangle,alloc::vec::impl$20::extend_trusted::closure_env$0<country_core::resources::rt::GpuRtTriangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<u32>,country_core::resources::rt::bvh::build_gpu_bvh::closure_env$1> > > > @ 0x147687e -> 0x8b9f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<country_core::resources::rt::GpuRtTriangle,core::slice::iter::Iter<u32>,country_core::resources::rt::bvh::build_gpu_bvh::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::resources::rt::GpuRtTriangle,alloc::vec::impl$20::extend_trusted::closure_env$0<country_core::resources::rt::GpuRtTriangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<u32>,country_core::resources::rt::bvh::build_gpu_bvh::closure_env$1> > > > @ 0x147688d -> 0x8b9f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<country_core::resources::rt::GpuRtTriangle,core::slice::iter::Iter<u32>,country_core::resources::rt::bvh::build_gpu_bvh::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::resources::rt::GpuRtTriangle,alloc::vec::impl$20::extend_trusted::closure_env$0<country_core::resources::rt::GpuRtTriangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<u32>,country_core::resources::rt::bvh::build_gpu_bvh::closure_env$1> > > > @ 0x1476899 -> 0x8b9f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<country_core::resources::rt::GpuRtTriangle,core::slice::iter::Iter<u32>,country_core::resources::rt::bvh::build_gpu_bvh::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::resources::rt::GpuRtTriangle,alloc::vec::impl$20::extend_trusted::closure_env$0<country_core::resources::rt::GpuRtTriangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<u32>,country_core::resources::rt::bvh::build_gpu_bvh::closure_env$1> > > > @ 0x14768a8 -> 0x8b9f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<country_core::resources::rt::GpuRtTriangle,core::slice::iter::Iter<u32>,country_core::resources::rt::bvh::build_gpu_bvh::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::resources::rt::GpuRtTriangle,alloc::vec::impl$20::extend_trusted::closure_env$0<country_core::resources::rt::GpuRtTriangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<u32>,country_core::resources::rt::bvh::build_gpu_bvh::closure_env$1> > > > @ 0x14768b7 -> 0x8b9f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<country_core::resources::rt::GpuRtTriangle,core::slice::iter::Iter<u32>,country_core::resources::rt::bvh::build_gpu_bvh::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::resources::rt::GpuRtTriangle,alloc::vec::impl$20::extend_trusted::closure_env$0<country_core::resources::rt::GpuRtTriangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<u32>,country_core::resources::rt::bvh::build_gpu_bvh::closure_env$1> > > > @ 0x14768c5 -> 0x8b9f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::update_instance_data<country_core::render::nani::instance_data::UberInstanceData> @ 0x156dcb4 -> 0x8b9f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::update_instance_data<country_core::render::nani::instance_data::SolidVertexColorInstanceData> @ 0x156e004 -> 0x8b9f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::update_instance_data<country_core::render::nani::instance_data::SolidVertexColorBricksInstanceData> @ 0x156e354 -> 0x8b9f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::update_instance_data<country_core::render::nani::instance_data::PlasterInstanceData> @ 0x156e6a4 -> 0x8b9f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::update_instance_data<country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData> @ 0x156e9f4 -> 0x8b9f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::update_instance_data<country_core::render::nani::instance_data::GothicWindowBricksInstanceData> @ 0x156ed46 -> 0x8b9f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::update_instance_data<country_core::render::nani::instance_data::BalconyDoorInstanceData> @ 0x156f0b4 -> 0x8b9f20 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::halver::impl$1::to_f16
// 信息 ID: 74f2b39fdb261103bd616328
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::halver::impl$2::to_f16
// 信息 ID: 6a3f3bcd0b709cb2f547ec0d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::util_assemble_gothic_dormer::assemble_gothic_dormer_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$6> @ 0x1291094 -> 0x1940140 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::util_assemble_gothic_dormer::assemble_gothic_dormer_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$6> @ 0x1291b41 -> 0x1940140 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x21afb06 -> 0x1940140 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x21b02bb -> 0x1940140 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x21b04d8 -> 0x1940140 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::halver::impl$5::to_f16; module=757, inline_record=520; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::halver::impl$3::to_f16
// 信息 ID: a544752540eea4f40f3b5d5c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x21b1a7f -> 0x1940180 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x21b1dc2 -> 0x1940180 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::halver::impl$7::to_f16; module=3077, inline_record=396; 不是独立调用边。
// [PDB 内联宿主] utils::halver::impl$6::to_f16; module=3078, inline_record=396; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::halver::impl$5::to_f16
// 信息 ID: acb800b064a71578934872e9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 9 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> @ 0x128f475 -> 0xc8ffb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> @ 0x128f69d -> 0xc8ffb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> @ 0x128f969 -> 0xc8ffb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> @ 0x128ef39 -> 0xc8ffb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles::closure$3<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> @ 0x12909f1 -> 0xc8ffb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x21b1abf -> 0xc8ffb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x21b1dd1 -> 0xc8ffb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x21b3b15 -> 0xc8ffb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x21b3dae -> 0xc8ffb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::halver::impl$6::to_f16
// 信息 ID: e833e906be8be446382d4a03
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles::closure$3<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> @ 0x12909e4 -> 0x20071f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::halver::impl$7::to_f16
// 信息 ID: fe4c1c12fa1dbcdf01fe42c5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 14 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> @ 0x128f466 -> 0x20071b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> @ 0x128f66c -> 0x20071b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> @ 0x128f93a -> 0x20071b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> @ 0x128ef22 -> 0x20071b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles::closure$3<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> @ 0x12909be -> 0x20071b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::util_assemble_gothic_dormer::assemble_gothic_dormer_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$6> @ 0x1291059 -> 0x20071b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::util_assemble_gothic_dormer::assemble_gothic_dormer_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$6> @ 0x1291ad6 -> 0x20071b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x21afad4 -> 0x20071b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x21b1a5c -> 0x20071b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x21b1db6 -> 0x20071b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x21b01ee -> 0x20071b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x21b04a6 -> 0x20071b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x21b3ac1 -> 0x20071b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x21b3d9f -> 0x20071b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
