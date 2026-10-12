// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/wall-constructor/src/construct_platform_stairs/bottom_riser.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 78d6109280cd14736953d4f747e568a7780903dd3dd4c62153f6977f2c075086
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: bottom_riser.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1521, 3141, 3144
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::bottom_riser::extend_bottom_risers_to_terrain<ref$<system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$5> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 8
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3141,"record_offset":1880,"rva":38653840,"va":5407362960}
// 编译实例 RVAs: 0x24dcf90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141419bb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: bottom_riser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17577162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407362960,"raw_hex":"e836cbf3fe","size":5,"soft_fail":false,"target_va":5389786032,"va":5407363189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bda0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: bottom_riser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9694312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407362960,"raw_hex":"e868ec9300","size":5,"soft_fail":false,"target_va":5417057696,"va":5407363379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bef0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: bottom_riser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9694636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407362960,"raw_hex":"e8aced9300","size":5,"soft_fail":false,"target_va":5417058032,"va":5407363391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::array::impl$24::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3141,"record_offset":2828,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":12061,"source_line":581,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3141,"record_offset":2772,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_direct] core::ops::function::impls::impl$0::call
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":3141,"record_offset":2660,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":43376,"source_line":262,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3141,"record_offset":2188,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3141,"record_offset":2308,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3141,"record_offset":2572,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3141,"record_offset":2484,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$88::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3141,"record_offset":2252,"parent_record_offset":1880,"ancestor_record_offset":1880,"inlinee":13898,"source_line":1853,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_transitive] core::array::drain::impl$2::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\drain.rs
//     evidence={"module":3141,"record_offset":2956,"parent_record_offset":2932,"ancestor_record_offset":1880,"inlinee":7563,"source_line":71,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$24::try_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3141,"record_offset":2852,"parent_record_offset":2828,"ancestor_record_offset":1880,"inlinee":12064,"source_line":622,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_transitive] core::array::try_from_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3141,"record_offset":2908,"parent_record_offset":2852,"ancestor_record_offset":1880,"inlinee":12066,"source_line":148,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_transitive] core::array::try_from_fn_erased
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3141,"record_offset":2932,"parent_record_offset":2908,"ancestor_record_offset":1880,"inlinee":12067,"source_line":928,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":3141,"record_offset":2880,"parent_record_offset":2852,"ancestor_record_offset":1880,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3141,"record_offset":2600,"parent_record_offset":2572,"ancestor_record_offset":1880,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3141,"record_offset":2336,"parent_record_offset":2308,"ancestor_record_offset":1880,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$327::neg
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3141,"record_offset":2276,"parent_record_offset":2252,"ancestor_record_offset":1880,"inlinee":8939,"source_line":720,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3141,"record_offset":2512,"parent_record_offset":2484,"ancestor_record_offset":1880,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$1::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\try_trait.rs
//     evidence={"module":3141,"record_offset":2980,"parent_record_offset":2956,"ancestor_record_offset":1880,"inlinee":9885,"source_line":416,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3141,"record_offset":2216,"parent_record_offset":2188,"ancestor_record_offset":1880,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3141,"record_offset":3084,"parent_record_offset":3004,"ancestor_record_offset":1880,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3141,"record_offset":3140,"parent_record_offset":3004,"ancestor_record_offset":1880,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::cos
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3141,"record_offset":3028,"parent_record_offset":3004,"ancestor_record_offset":1880,"inlinee":11241,"source_line":723,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3141,"record_offset":3056,"parent_record_offset":3004,"ancestor_record_offset":1880,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_platform_stairs::bottom_riser::extend_bottom_risers_to_terrain::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/bottom_riser.rs
//     evidence={"module":3141,"record_offset":3004,"parent_record_offset":2980,"ancestor_record_offset":1880,"inlinee":80033,"source_line":22,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure$5
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/wooden.rs
//     evidence={"module":3141,"record_offset":2684,"parent_record_offset":2660,"ancestor_record_offset":1880,"inlinee":117755,"source_line":214,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::bottom_riser::extend_bottom_risers_to_terrain<system_wall_constructor::construct_platform_stairs::wooden::build_circle::closure_env$5>
// 记录形式: compiled_procedure; instances=1
// 源码行: 8
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3141,"record_offset":280,"rva":38653248,"va":5407362368}
// 编译实例 RVAs: 0x24dcd40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141419bb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: bottom_riser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17576570","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407362368,"raw_hex":"e886cdf3fe","size":5,"soft_fail":false,"target_va":5389786032,"va":5407362597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bda0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: bottom_riser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9694904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407362368,"raw_hex":"e8b8ee9300","size":5,"soft_fail":false,"target_va":5417057696,"va":5407362787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bef0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: bottom_riser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9695228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407362368,"raw_hex":"e8fcef9300","size":5,"soft_fail":false,"target_va":5417058032,"va":5407362799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::array::impl$24::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3141,"record_offset":1164,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":12061,"source_line":581,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3141,"record_offset":1108,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":3141,"record_offset":580,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3141,"record_offset":700,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3141,"record_offset":964,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3141,"record_offset":876,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$88::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3141,"record_offset":644,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":13898,"source_line":1853,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_platform_stairs::wooden::build_circle::closure$5
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/wooden.rs
//     evidence={"module":3141,"record_offset":1052,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":117754,"source_line":427,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_transitive] core::array::drain::impl$2::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\drain.rs
//     evidence={"module":3141,"record_offset":1292,"parent_record_offset":1268,"ancestor_record_offset":280,"inlinee":7563,"source_line":71,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$24::try_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3141,"record_offset":1188,"parent_record_offset":1164,"ancestor_record_offset":280,"inlinee":12064,"source_line":622,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_transitive] core::array::try_from_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3141,"record_offset":1244,"parent_record_offset":1188,"ancestor_record_offset":280,"inlinee":12066,"source_line":148,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_transitive] core::array::try_from_fn_erased
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":3141,"record_offset":1268,"parent_record_offset":1244,"ancestor_record_offset":280,"inlinee":12067,"source_line":928,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":3141,"record_offset":1216,"parent_record_offset":1188,"ancestor_record_offset":280,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3141,"record_offset":992,"parent_record_offset":964,"ancestor_record_offset":280,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3141,"record_offset":728,"parent_record_offset":700,"ancestor_record_offset":280,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$327::neg
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3141,"record_offset":668,"parent_record_offset":644,"ancestor_record_offset":280,"inlinee":8939,"source_line":720,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3141,"record_offset":904,"parent_record_offset":876,"ancestor_record_offset":280,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$1::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\try_trait.rs
//     evidence={"module":3141,"record_offset":1316,"parent_record_offset":1292,"ancestor_record_offset":280,"inlinee":9885,"source_line":416,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":3141,"record_offset":608,"parent_record_offset":580,"ancestor_record_offset":280,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3141,"record_offset":1420,"parent_record_offset":1340,"ancestor_record_offset":280,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3141,"record_offset":1476,"parent_record_offset":1340,"ancestor_record_offset":280,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::cos
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3141,"record_offset":1364,"parent_record_offset":1340,"ancestor_record_offset":280,"inlinee":11241,"source_line":723,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3141,"record_offset":1392,"parent_record_offset":1340,"ancestor_record_offset":280,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_platform_stairs::bottom_riser::extend_bottom_risers_to_terrain::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/bottom_riser.rs
//     evidence={"module":3141,"record_offset":1340,"parent_record_offset":1316,"ancestor_record_offset":280,"inlinee":80033,"source_line":22,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_platform_stairs::bottom_riser::extend_bottom_risers_to_terrain::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x138A1
// 首条位置: {"module":1521,"inlinee":80033}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3141,"record_offset":1420,"parent_record_offset":1340,"ancestor_record_offset":1340,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3141,"record_offset":1476,"parent_record_offset":1340,"ancestor_record_offset":1340,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::cos
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3141,"record_offset":1364,"parent_record_offset":1340,"ancestor_record_offset":1340,"inlinee":11241,"source_line":723,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sin
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":3141,"record_offset":1392,"parent_record_offset":1340,"ancestor_record_offset":1340,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3141,"record_offset":1504,"parent_record_offset":1476,"ancestor_record_offset":1340,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/03141.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3141,"record_offset":1444,"parent_record_offset":1420,"ancestor_record_offset":1340,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/03141.jsonl.gz"}
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
// 完整结构及原始类型记录: bottom_riser.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: bottom_riser.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-segmentation/three-version-platform-creator-owner-map.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/construct_platform_stairs/bottom_riser.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: bottom_riser.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::bottom_riser::extend_bottom_risers_to_terrain::closure$0
// 信息 ID: b84bf636a23bff57bdc74c09
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_platform_stairs::bottom_riser::extend_bottom_risers_to_terrain", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::bottom_riser::extend_bottom_risers_to_terrain<system_wall_constructor::construct_platform_stairs::wooden::build_circle::closure_env$5>; module=3141, inline_record=1340; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::bottom_riser::extend_bottom_risers_to_terrain<system_wall_constructor::construct_platform_stairs::wooden::build_circle::closure_env$5>; module=3141, inline_record=1624; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::bottom_riser::extend_bottom_risers_to_terrain<ref$<system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$5> >; module=3141, inline_record=3004; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::bottom_riser::extend_bottom_risers_to_terrain<ref$<system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$5> >; module=3141, inline_record=3288; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::bottom_riser::extend_bottom_risers_to_terrain<ref$<system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$5> >
// 信息 ID: 191c10a033e6e4a81ca7ca65
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$5"]; 字段布局 = unknown。
// [类型关系] system_wall_constructor::construct_platform_stairs::bottom_riser::extend_bottom_risers_to_terrain -> ["ref$<system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$5>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::wooden_corners::build_corners<system_wall_constructor::construct_platform_stairs::wooden::build_rectangle::closure_env$5> @ 0x24ddc7c -> 0x24dcf90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_platform_stairs::bottom_riser::extend_bottom_risers_to_terrain<system_wall_constructor::construct_platform_stairs::wooden::build_circle::closure_env$5>
// 信息 ID: cc767aedf10903567c5dc07f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::construct_platform_stairs::bottom_riser::extend_bottom_risers_to_terrain -> ["system_wall_constructor::construct_platform_stairs::wooden::build_circle::closure_env$5"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs @ 0x141cbb5 -> 0x24dcd40 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs @ 0x141c07e -> 0x24dcd40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
