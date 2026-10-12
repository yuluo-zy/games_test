// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/decorator/src/utils/mod.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 2c3e4ad3de434fbb9e7e788be20a01f14b52c04dc8d3bebbc029d6d4cfa38c23
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 442, 1510, 1552, 1559, 3070
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::subdivide_wall_decorator
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6332
// 首条位置: {"module":442,"inlinee":25394}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":442,"record_offset":15416,"parent_record_offset":15328,"ancestor_record_offset":15328,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::with_offset_unbounded
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":442,"record_offset":15380,"parent_record_offset":15328,"ancestor_record_offset":15328,"inlinee":25129,"source_line":247,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":442,"record_offset":15440,"parent_record_offset":15416,"ancestor_record_offset":15328,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::utils::subdivide_wall_decorator::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1172D
// 首条位置: {"module":1510,"inlinee":71469}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1510,"record_offset":3188,"parent_record_offset":3160,"ancestor_record_offset":3160,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/01510.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::with_offset_unbounded
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1510,"record_offset":3244,"parent_record_offset":3160,"ancestor_record_offset":3160,"inlinee":25129,"source_line":247,"archive":"_pdb/B/modules/01510.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1510,"record_offset":3212,"parent_record_offset":3188,"ancestor_record_offset":3160,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01510.jsonl.gz"}
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
// 完整结构及原始类型记录: mod.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mod.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mod.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_decorator::utils::subdivide_wall_decorator
// 信息 ID: ae2610d92ba00246aad022b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows; module=442, inline_record=15328; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::utils::subdivide_wall_decorator::closure$0
// 信息 ID: 97290dabcd9e54790c22dcfb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::utils::subdivide_wall_decorator", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::utils::subdivide_wall_decorator::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<utils::components::transform::Transform,core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::utils::subdivide_wall_decorator::closure_env$0 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::utils::subdivide_wall_decorator::closure_env$0 <- itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0> > > >; module=1510, inline_record=3160; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0> > > >; module=1510, inline_record=3868; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >; module=1552, inline_record=4476; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
