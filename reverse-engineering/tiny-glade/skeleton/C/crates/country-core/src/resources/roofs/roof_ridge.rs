// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/resources/roofs/roof_ridge.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: bb54c965635762d1fc8bd4dfc52e579bf8fd061890ddb1074ffe4c64dbcc5ee0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: roof_ridge.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 522, 549, 570, 584
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use emath; // namespace candidate
//   package identities: [{"name":"emath","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.27.2"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":549,"record_offset":70904,"rva":11265232,"va":5379974352}
// 编译实例 RVAs: 0xabe4d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: roof_ridge.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3350353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379974352,"raw_hex":"e8511f3300","size":5,"soft_fail":false,"target_va":5383325024,"va":5379974666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36645621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379974352,"raw_hex":"e8f52a2f02","size":5,"soft_fail":false,"target_va":5416622240,"va":5379976614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3339915","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379974352,"raw_hex":"e88bf63200","size":5,"soft_fail":false,"target_va":5383314512,"va":5379974592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_ridge::RoofRidgeDims::max_dim
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_ridge.rs
//     evidence={"module":549,"record_offset":71084,"parent_record_offset":70904,"ancestor_record_offset":70904,"inlinee":34144,"source_line":27,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_ridge::calc_roof_tip_center_ls
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_ridge.rs
//     evidence={"module":549,"record_offset":71140,"parent_record_offset":70904,"ancestor_record_offset":70904,"inlinee":34146,"source_line":118,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_ridge::impl$0::to_world_space::closure$0
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_ridge.rs
//     evidence={"module":549,"record_offset":71488,"parent_record_offset":70904,"ancestor_record_offset":70904,"inlinee":34149,"source_line":72,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_shape::Roof::roof_tip_offset_xz
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":549,"record_offset":73000,"parent_record_offset":70904,"ancestor_record_offset":70904,"inlinee":34153,"source_line":265,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":549,"record_offset":71400,"parent_record_offset":70904,"ancestor_record_offset":70904,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":71308,"parent_record_offset":70904,"ancestor_record_offset":70904,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":549,"record_offset":71456,"parent_record_offset":71400,"ancestor_record_offset":70904,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":549,"record_offset":71428,"parent_record_offset":71400,"ancestor_record_offset":70904,"inlinee":27811,"source_line":974,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":549,"record_offset":71788,"parent_record_offset":71760,"ancestor_record_offset":70904,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\panic.rs
//     evidence={"module":549,"record_offset":73912,"parent_record_offset":73864,"ancestor_record_offset":70904,"inlinee":11859,"source_line":166,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic::runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":549,"record_offset":73940,"parent_record_offset":73912,"ancestor_record_offset":70904,"inlinee":11861,"source_line":2434,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":549,"record_offset":71108,"parent_record_offset":71084,"ancestor_record_offset":70904,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":71336,"parent_record_offset":71308,"ancestor_record_offset":70904,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=30
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":71216,"parent_record_offset":71192,"ancestor_record_offset":70904,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":71848,"parent_record_offset":71824,"ancestor_record_offset":70904,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::Roof::height
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":549,"record_offset":71612,"parent_record_offset":71488,"ancestor_record_offset":70904,"inlinee":34151,"source_line":210,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::Roof::pivot_3d
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":549,"record_offset":71584,"parent_record_offset":71488,"ancestor_record_offset":70904,"inlinee":34150,"source_line":196,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::Roof::shape_dims
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":549,"record_offset":71164,"parent_record_offset":71140,"ancestor_record_offset":70904,"inlinee":34147,"source_line":275,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::Roof::ty
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":549,"record_offset":71656,"parent_record_offset":71612,"ancestor_record_offset":70904,"inlinee":31232,"source_line":201,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::RoofShapeParams::height_ws
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":549,"record_offset":71688,"parent_record_offset":71612,"ancestor_record_offset":70904,"inlinee":34152,"source_line":418,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":549,"record_offset":73176,"parent_record_offset":73000,"ancestor_record_offset":70904,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$18::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":71192,"parent_record_offset":71140,"ancestor_record_offset":70904,"inlinee":15311,"source_line":1259,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":73120,"parent_record_offset":73000,"ancestor_record_offset":70904,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":72912,"parent_record_offset":72308,"ancestor_record_offset":70904,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":71524,"parent_record_offset":71488,"ancestor_record_offset":70904,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":549,"record_offset":71824,"parent_record_offset":71688,"ancestor_record_offset":70904,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] utils::smoothstep
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":549,"record_offset":71760,"parent_record_offset":71688,"ancestor_record_offset":70904,"inlinee":11673,"source_line":233,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_ridge::RoofRidgeDims::get_inner
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":549,"record_offset":76976,"rva":832880,"va":5369542000}
// 编译实例 RVAs: 0xcb570
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_ridge::RoofRidgeDims::max_dim
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8560
// 首条位置: {"module":549,"inlinee":34144}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":549,"record_offset":71108,"parent_record_offset":71084,"ancestor_record_offset":71084,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_ridge::RoofRidgeDims::from
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=8
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7889
// 首条位置: {"module":522,"inlinee":30857}
// 编译实例 RVAs: 0xabeeb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::roofs::roof_ridge::RoofRidgeDims::calc
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_ridge.rs
//     evidence={"module":522,"record_offset":29780,"parent_record_offset":29748,"ancestor_record_offset":29748,"inlinee":30858,"source_line":42,"archive":"_pdb/C/modules/00522.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_shape::Roof::shape_dims
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":549,"record_offset":76764,"parent_record_offset":76612,"ancestor_record_offset":76612,"inlinee":34147,"source_line":275,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":522,"record_offset":29944,"parent_record_offset":29836,"ancestor_record_offset":29748,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":522,"record_offset":29888,"parent_record_offset":29836,"ancestor_record_offset":29748,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":522,"record_offset":29860,"parent_record_offset":29836,"ancestor_record_offset":29748,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":522,"record_offset":29836,"parent_record_offset":29780,"ancestor_record_offset":29748,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00522.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_ridge::RoofRidgeDims::calc
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=10
// 源码行: 42
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x788A
// 首条位置: {"module":522,"inlinee":30858}
// 编译实例 RVAs: 0xabee60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":522,"record_offset":29836,"parent_record_offset":29780,"ancestor_record_offset":29780,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":522,"record_offset":29944,"parent_record_offset":29836,"ancestor_record_offset":29780,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":522,"record_offset":29888,"parent_record_offset":29836,"ancestor_record_offset":29780,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":522,"record_offset":29860,"parent_record_offset":29836,"ancestor_record_offset":29780,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00522.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_ridge::impl$0::to_world_space::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8565
// 首条位置: {"module":549,"inlinee":34149}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::roofs::roof_shape::Roof::height
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":549,"record_offset":71612,"parent_record_offset":71488,"ancestor_record_offset":71488,"inlinee":34151,"source_line":210,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_shape::Roof::pivot_3d
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":549,"record_offset":71584,"parent_record_offset":71488,"ancestor_record_offset":71488,"inlinee":34150,"source_line":196,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":72912,"parent_record_offset":72308,"ancestor_record_offset":72308,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":549,"record_offset":71524,"parent_record_offset":71488,"ancestor_record_offset":71488,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":549,"record_offset":71788,"parent_record_offset":71760,"ancestor_record_offset":71488,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\panic.rs
//     evidence={"module":549,"record_offset":73912,"parent_record_offset":73864,"ancestor_record_offset":73376,"inlinee":11859,"source_line":166,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic::runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":549,"record_offset":73940,"parent_record_offset":73912,"ancestor_record_offset":73376,"inlinee":11861,"source_line":2434,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":549,"record_offset":71732,"parent_record_offset":71688,"ancestor_record_offset":71488,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":71552,"parent_record_offset":71524,"ancestor_record_offset":71488,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":71876,"parent_record_offset":71824,"ancestor_record_offset":71488,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":71848,"parent_record_offset":71824,"ancestor_record_offset":71488,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::Roof::ty
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":549,"record_offset":71656,"parent_record_offset":71612,"ancestor_record_offset":71488,"inlinee":31232,"source_line":201,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::RoofShapeParams::height_ws
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":549,"record_offset":71688,"parent_record_offset":71612,"ancestor_record_offset":71488,"inlinee":34152,"source_line":418,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":549,"record_offset":71824,"parent_record_offset":71688,"ancestor_record_offset":71488,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] utils::smoothstep
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":549,"record_offset":71760,"parent_record_offset":71688,"ancestor_record_offset":71488,"inlinee":11673,"source_line":233,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_ridge::calc_roof_tip_center_ls
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 118
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8562
// 首条位置: {"module":549,"inlinee":34146}
// 编译实例 RVAs: 0xabefa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::roofs::roof_shape::Roof::shape_dims
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":549,"record_offset":71164,"parent_record_offset":71140,"ancestor_record_offset":71140,"inlinee":34147,"source_line":275,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$18::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":549,"record_offset":71192,"parent_record_offset":71140,"ancestor_record_offset":71140,"inlinee":15311,"source_line":1259,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":549,"record_offset":71216,"parent_record_offset":71192,"ancestor_record_offset":71140,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_ridge::inverse_ridge_01_param
// 记录形式: compiled_procedure; instances=1
// 源码行: 122
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":549,"record_offset":77120,"rva":11267872,"va":5379976992}
// 编译实例 RVAs: 0xabef20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36644514","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379976992,"raw_hex":"e8a2262f02","size":5,"soft_fail":false,"target_va":5416621600,"va":5379977081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36645098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379976992,"raw_hex":"e8ea282f02","size":5,"soft_fail":false,"target_va":5416622208,"va":5379977105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":549,"record_offset":77372,"parent_record_offset":77120,"ancestor_record_offset":77120,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":549,"record_offset":77400,"parent_record_offset":77120,"ancestor_record_offset":77120,"inlinee":11774,"source_line":651,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":549,"record_offset":77340,"parent_record_offset":77120,"ancestor_record_offset":77120,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] emath::inverse_lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"emath","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/emath-0.27.2/src/lib.rs
//     evidence={"module":549,"record_offset":77284,"parent_record_offset":77120,"ancestor_record_offset":77120,"inlinee":34154,"source_line":125,"archive":"_pdb/C/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$34::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":549,"record_offset":77312,"parent_record_offset":77284,"ancestor_record_offset":77120,"inlinee":7407,"source_line":1878,"archive":"_pdb/C/modules/00549.jsonl.gz"}
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
// 完整结构及原始类型记录: roof_ridge.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::roofs::roof_ridge::RoofRidgeDims
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 6; 过程引用数: 4
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: roof_ridge.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: roof_ridge.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_ridge::RoofRidgeDims::calc
// 信息 ID: b5d85df51313881cbe27bf53
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [静态直接引用] timberframe::timberframe_rectangle::construct_timberframe_rectangle @ 0x1f477fa -> 0xabee60 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::resources::roofs::roof_visual_shape::get_roof_expanded_top_and_bottom; module=522, inline_record=29780; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_ridge::RoofRidgeDims::from; module=549, inline_record=76792; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::snap_to_roof_ridge; module=570, inline_record=72452; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_shape::Roof::roof_shape_at_intersection_y; module=584, inline_record=23980; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_shape::Roof::get_roof_shape; module=584, inline_record=26616; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_ridge::RoofRidgeDims::from
// 信息 ID: 8722175f1f0b6752bb31b0a9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> > >,system_roof::visual::place_spires::place_roof_spires::closure_env$0> @ 0x1f074d5 -> 0xabeeb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_edit_roof_ridge_length::create_ui_widget @ 0x1f1e21c -> 0xabeeb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused @ 0x1f36afb -> 0xabeeb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_edit_roof_ridge_dir::ui_edit_roof_ridge_dir @ 0x1f388c4 -> 0xabeeb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_edit_roof_ridge_dir::do_ui_edit_roof_ridge_dir @ 0x1f39647 -> 0xabeeb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$1> @ 0x282c70a -> 0xabeeb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren::update_roof_bird_spots @ 0x293ec86 -> 0xabeeb0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::resources::roofs::roof_visual_shape::get_roof_expanded_top_and_bottom; module=522, inline_record=29748; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::snap_to_roof_ridge; module=570, inline_record=72396; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_shape::Roof::roof_shape_at_intersection_y; module=584, inline_record=23948; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_shape::Roof::get_roof_shape; module=584, inline_record=26572; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_ridge::RoofRidgeDims::get_inner
// 信息 ID: 018fcbdd098edd525c3c7932
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 388 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::buffer::Buffer> @ 0x9f196 -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::sampler::Sampler> @ 0xa1750 -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::texture::Texture> @ 0xa1a29 -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::shader::RasterPipeline> @ 0xa7c26 -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::binding_set::BindingSet> @ 0xa9c9f -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::render_pass::RenderPass> @ 0xaa1a0 -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::shader::ComputePipeline> @ 0xaa470 -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::swapchain::FallibleSwapchain> @ 0xaff6f -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::binding_set::BindingSetLayout> @ 0xb17d8 -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x186dcb -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x186ebf -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x186f87 -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x187051 -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::impl$5::tod_circle<u64> @ 0x222070 -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$27::label_with_id<ref$<str$> > @ 0x223560 -> 0xcb570 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$27::image<ref$<str$> > @ 0x223a5c -> 0xcb570 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_ridge::RoofRidgeDims::max_dim
// 信息 ID: 22eeba6d484d0b6ce08fe215
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space; module=549, inline_record=71084; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space
// 信息 ID: 7f1e36d0ce16ea4a90028745
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_roof::ui_edit_roof_ridge_length::create_ui_widget @ 0x1f1e239 -> 0xabe4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused @ 0x1f36b19 -> 0xabe4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_edit_roof_ridge_dir::ui_edit_roof_ridge_dir @ 0x1f388e6 -> 0xabe4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_edit_roof_ridge_dir::do_ui_edit_roof_ridge_dir @ 0x1f39665 -> 0xabe4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$1> @ 0x282c72c -> 0xabe4d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_ridge::calc_roof_tip_center_ls
// 信息 ID: 20e9c94d9370f2af1d217dff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::half_timber::roof_outline @ 0x1439ed0 -> 0xabefa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::roof_trigger_volume::calculate_roof_trigger_volumes @ 0x2822563 -> 0xabefa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::roof_trigger_volume::calculate_roof_trigger_volumes @ 0x2824c70 -> 0xabefa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$1> @ 0x282bbd5 -> 0xabefa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren::update_roof_bird_spots @ 0x293ecb8 -> 0xabefa0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space; module=549, inline_record=71140; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_ridge::impl$0::to_world_space::closure$0
// 信息 ID: 51fc48be72fbbf711e0cf0c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::roofs::roof_ridge::impl$0::to_world_space", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space; module=549, inline_record=71488; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space; module=549, inline_record=72308; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space; module=549, inline_record=72972; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space; module=549, inline_record=73376; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space; module=549, inline_record=74388; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space; module=549, inline_record=75208; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space; module=549, inline_record=75872; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space; module=549, inline_record=75900; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_ridge::inverse_ridge_01_param
// 信息 ID: a94de40e770fc78da9fc1e5c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
