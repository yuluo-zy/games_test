// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/utils/src/geometry/ray.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: e7203270dc4bae751c0ffed81f5fb70c2b15ffc08712d285a385b0ad74ed258c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ray.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1578
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::geometry::ray::Ray::from_mouse_ss
// 记录形式: compiled_procedure; instances=1
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1578,"record_offset":260,"rva":21847952,"va":5390557072}
// 编译实例 RVAs: 0x14d5f90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::mat4::Mat4::project_point3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat4.rs
//     evidence={"module":1578,"record_offset":2876,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":32183,"source_line":1258,"archive":"_pdb/C/modules/01578.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat4::impl$16::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat4.rs
//     evidence={"module":1578,"record_offset":644,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":42077,"source_line":1585,"archive":"_pdb/C/modules/01578.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1578,"record_offset":484,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01578.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1578,"record_offset":408,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":8895,"source_line":1124,"archive":"_pdb/C/modules/01578.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1578,"record_offset":564,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01578.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1578,"record_offset":3736,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/01578.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1578,"record_offset":3680,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/01578.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::simd::Simd::splat
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\simd.rs
//     evidence={"module":1578,"record_offset":2956,"parent_record_offset":2932,"ancestor_record_offset":260,"inlinee":17835,"source_line":55,"archive":"_pdb/C/modules/01578.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1578,"record_offset":932,"parent_record_offset":836,"ancestor_record_offset":260,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/01578.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1578,"record_offset":3356,"parent_record_offset":3276,"ancestor_record_offset":260,"inlinee":32186,"source_line":113,"archive":"_pdb/C/modules/01578.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1578,"record_offset":804,"parent_record_offset":740,"ancestor_record_offset":260,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/01578.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1578,"record_offset":2932,"parent_record_offset":2908,"ancestor_record_offset":260,"inlinee":17833,"source_line":934,"archive":"_pdb/C/modules/01578.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1578,"record_offset":772,"parent_record_offset":740,"ancestor_record_offset":260,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/01578.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1578,"record_offset":3876,"parent_record_offset":3852,"ancestor_record_offset":260,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01578.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1578,"record_offset":512,"parent_record_offset":484,"ancestor_record_offset":260,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01578.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1578,"record_offset":432,"parent_record_offset":408,"ancestor_record_offset":260,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/01578.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1578,"record_offset":592,"parent_record_offset":564,"ancestor_record_offset":260,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::Mat4::mul_vec4
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat4.rs
//     evidence={"module":1578,"record_offset":704,"parent_record_offset":680,"ancestor_record_offset":260,"inlinee":20184,"source_line":1357,"archive":"_pdb/C/modules/01578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat4::impl$22::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat4.rs
//     evidence={"module":1578,"record_offset":680,"parent_record_offset":644,"ancestor_record_offset":260,"inlinee":30714,"source_line":1636,"archive":"_pdb/C/modules/01578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$25::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":1578,"record_offset":2908,"parent_record_offset":2876,"ancestor_record_offset":260,"inlinee":7188,"source_line":1326,"archive":"_pdb/C/modules/01578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$35::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":1578,"record_offset":3164,"parent_record_offset":2876,"ancestor_record_offset":260,"inlinee":7193,"source_line":1404,"archive":"_pdb/C/modules/01578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$9::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":1578,"record_offset":3276,"parent_record_offset":2876,"ancestor_record_offset":260,"inlinee":32185,"source_line":1202,"archive":"_pdb/C/modules/01578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1578,"record_offset":3820,"parent_record_offset":3788,"ancestor_record_offset":260,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1578,"record_offset":3788,"parent_record_offset":3760,"ancestor_record_offset":260,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/01578.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1578,"record_offset":3760,"parent_record_offset":3736,"ancestor_record_offset":260,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/01578.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::wwww
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":1578,"record_offset":1092,"parent_record_offset":704,"ancestor_record_offset":260,"inlinee":20188,"source_line":1916,"archive":"_pdb/C/modules/01578.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::xxxx
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":1578,"record_offset":740,"parent_record_offset":704,"ancestor_record_offset":260,"inlinee":20185,"source_line":646,"archive":"_pdb/C/modules/01578.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::yyyy
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":1578,"record_offset":836,"parent_record_offset":704,"ancestor_record_offset":260,"inlinee":20186,"source_line":1066,"archive":"_pdb/C/modules/01578.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::zzzz
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":1578,"record_offset":964,"parent_record_offset":704,"ancestor_record_offset":260,"inlinee":20187,"source_line":1491,"archive":"_pdb/C/modules/01578.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1578,"record_offset":3852,"parent_record_offset":3820,"ancestor_record_offset":260,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01578.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::geometry::ray::Ray::xz
// 记录形式: compiled_procedure; instances=1
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1578,"record_offset":3956,"rva":10463600,"va":5379172720}
// 编译实例 RVAs: 0x9fa970
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
// 完整结构及原始类型记录: ray.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::geometry::ray::Ray
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "tuple$<utils::geometry::ray::Ray,f32>", "constructor": "tuple$", "arguments": ["utils::geometry::ray::Ray", "f32"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::impls::impl$0::call<tuple$<utils::geometry::ray::Ray,f32>,system_critters::wren_boids::sim_wren_boids::closure_env$3>", "constructor": "core::ops::function::impls::impl$0::call", "arguments": ["tuple$<utils::geometry::ray::Ray,f32>", "system_critters::wren_boids::sim_wren_boids::closure_env$3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ray.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ray.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: utils::geometry::ray::Ray::from_mouse_ss
// 信息 ID: 8df4c081dcb1582c26bd2d6a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x862a3a -> 0x14d5f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x867cb5 -> 0x14d5f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::main_camera::main_camera_update @ 0xa210c0 -> 0x14d5f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::hover_intersection::hover_intersection @ 0xc41405 -> 0x14d5f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::mouse_terrain_raycast::camera_mouse_ray @ 0xc43e24 -> 0x14d5f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::calculate_decorator_grab_offset::raycast_with_ss_offset @ 0x14d1967 -> 0x14d5f90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::geometry::ray::Ray::xz
// 信息 ID: 6b0a3256c1ebd0469a64bb9a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 16 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> > @ 0x30951d -> 0x9fa970 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> > @ 0x309a3a -> 0x9fa970 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::ui_edit_rectangle_dims @ 0xb3fb55 -> 0x9fa970 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::ui_edit_rectangle_rotate @ 0xb41de9 -> 0x9fa970 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> > @ 0x1417b7d -> 0x9fa970 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> > @ 0x141809a -> 0x9fa970 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::walls::WallId,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> > @ 0x1418dbe -> 0x9fa970 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::walls::WallId,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> > @ 0x14192df -> 0x9fa970 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::walls::decorator::DecoratorId,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> > @ 0x14f289f -> 0x9fa970 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::walls::decorator::DecoratorId,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> > @ 0x14f2dbd -> 0x9fa970 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::stairs::stairs_state::StairGraphEdge,utils::IsoTransform> @ 0x14f3b0d -> 0x9fa970 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::stairs::stairs_state::StairGraphEdge,utils::IsoTransform> @ 0x14f402a -> 0x9fa970 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::walls::decorator::DecoratorId,country_core::systems::collision::world::OffsetColliderTransform> @ 0x14f4d3f -> 0x9fa970 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<country_core::resources::walls::decorator::DecoratorId,country_core::systems::collision::world::OffsetColliderTransform> @ 0x14f525d -> 0x9fa970 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<tuple$<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>,utils::IsoTransform> @ 0x14f5ee7 -> 0x9fa970 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::upsert_collider<tuple$<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>,utils::IsoTransform> @ 0x14f63ec -> 0x9fa970 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
