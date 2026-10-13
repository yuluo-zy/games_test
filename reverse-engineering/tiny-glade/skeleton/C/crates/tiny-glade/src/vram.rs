// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/vram.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 1b8e54dbc3f8ead0253f8906adfd85d5ec1f3aff890399a39bbd61685bfdb052
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: vram.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 25, 246
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::vram::calculate_resolution_scale_mem_estimate_text
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":246,"record_offset":23072,"rva":6765392,"va":5375474512}
// 编译实例 RVAs: 0x673b50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9319536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375474512,"raw_hex":"e870348e00","size":5,"soft_fail":false,"target_va":5384794864,"va":5375475323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::physical_device::PhysicalDevice::device_local_memory_budget_bytes
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t8459791","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375474512,"raw_hex":"e80f168100","size":5,"soft_fail":false,"target_va":5383934704,"va":5375474908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::physical_device::PhysicalDevice::host_memory_budget_bytes
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t8458883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375474512,"raw_hex":"e883128100","size":5,"soft_fail":false,"target_va":5383933840,"va":5375474952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":246,"record_offset":23524,"parent_record_offset":23072,"ancestor_record_offset":23072,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::enums::impl$538::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":246,"record_offset":23344,"parent_record_offset":23072,"ancestor_record_offset":23072,"inlinee":17758,"source_line":610,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::physical_device
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/device.rs
//     evidence={"module":246,"record_offset":23372,"parent_record_offset":23072,"ancestor_record_offset":23072,"inlinee":17756,"source_line":710,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::vram_constants::MemoryScalingModel::resolution_scale_mem_estimate
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/vram.rs
//     evidence={"module":246,"record_offset":23232,"parent_record_offset":23072,"ancestor_record_offset":23072,"inlinee":7091,"source_line":27,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":246,"record_offset":23308,"parent_record_offset":23284,"ancestor_record_offset":23072,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":246,"record_offset":23420,"parent_record_offset":23396,"ancestor_record_offset":23072,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$84::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":246,"record_offset":23396,"parent_record_offset":23372,"ancestor_record_offset":23072,"inlinee":12534,"source_line":4190,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":246,"record_offset":23284,"parent_record_offset":23232,"ancestor_record_offset":23072,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::vram_constants::MemoryScalingModel::conservative_bytes_per_world_pixel
// 记录形式: inlinee_declaration; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B9B
// 首条位置: {"module":25,"inlinee":7067}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::vram_constants::MemoryScalingModel::conservative_bytes_per_ui_pixel
// 记录形式: inlinee_declaration; instances=1
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B9A
// 首条位置: {"module":25,"inlinee":7066}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::vram_constants::MemoryScalingModel::resolution_scale_mem_estimate
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1BB3
// 首条位置: {"module":246,"inlinee":7091}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":246,"record_offset":23284,"parent_record_offset":23232,"ancestor_record_offset":23232,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":246,"record_offset":23308,"parent_record_offset":23284,"ancestor_record_offset":23232,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::vram::calculate_max_resolution_scale
// 记录形式: compiled_procedure; instances=1
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":246,"record_offset":22040,"rva":6764592,"va":5375473712}
// 编译实例 RVAs: 0x673830
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: vram.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41583342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375473712,"raw_hex":"e8ee827a02","size":5,"soft_fail":false,"target_va":5417057792,"va":5375474445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::physical_device::PhysicalDevice::device_local_memory_budget_bytes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t8460916","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375473712,"raw_hex":"e8741a8100","size":5,"soft_fail":false,"target_va":5383934704,"va":5375473783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::physical_device::PhysicalDevice::host_memory_budget_bytes
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t8459806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375473712,"raw_hex":"e81e168100","size":5,"soft_fail":false,"target_va":5383933840,"va":5375474029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::vk::enums::impl$538::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":246,"record_offset":22272,"parent_record_offset":22040,"ancestor_record_offset":22040,"inlinee":17758,"source_line":610,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":246,"record_offset":22748,"parent_record_offset":22040,"ancestor_record_offset":22040,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::physical_device
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/device.rs
//     evidence={"module":246,"record_offset":22188,"parent_record_offset":22040,"ancestor_record_offset":22040,"inlinee":17756,"source_line":710,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::floor
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":246,"record_offset":23012,"parent_record_offset":22040,"ancestor_record_offset":22040,"inlinee":9785,"source_line":49,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::vram::calculate_max_resolution_scale_from_model
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/vram.rs
//     evidence={"module":246,"record_offset":22300,"parent_record_offset":22040,"ancestor_record_offset":22040,"inlinee":17760,"source_line":69,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":246,"record_offset":22236,"parent_record_offset":22212,"ancestor_record_offset":22040,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$84::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":246,"record_offset":22212,"parent_record_offset":22188,"ancestor_record_offset":22040,"inlinee":12534,"source_line":4190,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::floor
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":246,"record_offset":23036,"parent_record_offset":23012,"ancestor_record_offset":22040,"inlinee":9786,"source_line":1721,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::max
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":246,"record_offset":22356,"parent_record_offset":22300,"ancestor_record_offset":22040,"inlinee":16202,"source_line":1009,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::min
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":246,"record_offset":22384,"parent_record_offset":22300,"ancestor_record_offset":22040,"inlinee":16200,"source_line":1036,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::math::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":246,"record_offset":22464,"parent_record_offset":22440,"ancestor_record_offset":22040,"inlinee":17762,"source_line":2038,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] std::f64::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f64.rs
//     evidence={"module":246,"record_offset":22440,"parent_record_offset":22300,"ancestor_record_offset":22040,"inlinee":17761,"source_line":384,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::vram::calculate_max_resolution_scale_from_model
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4560
// 首条位置: {"module":246,"inlinee":17760}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f64::impl$0::max
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":246,"record_offset":22356,"parent_record_offset":22300,"ancestor_record_offset":22300,"inlinee":16202,"source_line":1009,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] core::f64::impl$0::min
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":246,"record_offset":22384,"parent_record_offset":22300,"ancestor_record_offset":22300,"inlinee":16200,"source_line":1036,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] std::f64::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f64.rs
//     evidence={"module":246,"record_offset":22440,"parent_record_offset":22300,"ancestor_record_offset":22300,"inlinee":17761,"source_line":384,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::math::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f64.rs
//     evidence={"module":246,"record_offset":22464,"parent_record_offset":22440,"ancestor_record_offset":22300,"inlinee":17762,"source_line":2038,"archive":"_pdb/C/modules/00246.jsonl.gz"}
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
// 完整结构及原始类型记录: vram.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: vram.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: vram.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 6; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,enum2$<core::option::Option<ref$<utils::components::transform::Transform> > >,ref$<country_core::systems::debug::human_scale_ref::HumanScaleRef> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,enum2$<core::option::Option<ref$<utils::components::transform::Transform> > >,ref$<country_core::systems::debug::human_scale_ref::HumanScaleRef> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,enum2$<core::option::Option<ref$<utils::components::transform::Transform> > >,ref$<country_core::systems::debug::human_scale_ref::HumanScaleRef> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::brush_preview::BrushPreview>,enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > > >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::brush_preview::OverwriteBrushPreview>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>,tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser> >)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::brush_preview::BrushPreview>,enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > > >,tuple$<> >", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::brush_preview::OverwriteBrushPreview>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>", "bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>", "bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>", "tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser> >"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::systems::brush_preview::BrushPreview>,enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > > >,tuple$<> >
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::brush_preview::OverwriteBrushPreview>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::temp_eraser::TempEraser>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2DFollowMouse> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2DFollowMouse> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2DFollowMouse> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<country_core::systems::look_at_camera::LookAtCamera> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref_mut$<country_core::components::ArrowGizmo> >,tuple$<> >,bevy_ecs::message::message_reader::MessageReader<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::CursorMovedScreenSpace>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref_mut$<country_core::components::ArrowGizmo> >,tuple$<> >", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::CursorMovedScreenSpace>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<utils::components::transform::Transform>,ref_mut$<country_core::components::ArrowGizmo> >,tuple$<> >
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::CursorMovedScreenSpace>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<ref$<enum2$<country_core::components::drawable::DrawableMeshData> >,bevy_ecs::query::filter::With<country_core::components::MeshFollowTerrain> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)
// 候选参数类型: ["bevy_ecs::system::query::Query<ref$<enum2$<country_core::components::drawable::DrawableMeshData> >,bevy_ecs::query::filter::With<country_core::components::MeshFollowTerrain> >", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::components::MeshFollowTerrain>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<enum2$<country_core::components::drawable::DrawableMeshData> >,bevy_ecs::query::filter::With<country_core::components::MeshFollowTerrain> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >

// ------------------------------------------------------------------
// 函数: tiny_glade::vram::calculate_max_resolution_scale
// 信息 ID: e4d43ced55495d4df6ba6b9c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::settings::ui_settings_video @ 0x3288aa -> 0x673830 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x42e99d -> 0x673830 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::game_loop::GameLoop::main_loop @ 0x57e668 -> 0x673830 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::vram::calculate_max_resolution_scale_from_model
// 信息 ID: 2dd3d9af42a3f7c947e6995a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::vram::calculate_max_resolution_scale; module=246, inline_record=22300; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::vram::calculate_max_resolution_scale; module=246, inline_record=22528; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::vram::calculate_max_resolution_scale; module=246, inline_record=22776; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::vram::calculate_resolution_scale_mem_estimate_text
// 信息 ID: 8cff73dd575127154974c89e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::settings::ui_settings_video @ 0x328882 -> 0x673b50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::vram_constants::MemoryScalingModel::conservative_bytes_per_ui_pixel
// 信息 ID: 6c05cfb431ffb47507daacbc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::vram_constants::MemoryScalingModel::conservative_bytes_per_world_pixel
// 信息 ID: 518b27a0d27f444b7f230c73
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::vram_constants::MemoryScalingModel::resolution_scale_mem_estimate
// 信息 ID: 9691723f95ccf2fee133c3a9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::vram::calculate_resolution_scale_mem_estimate_text; module=246, inline_record=23232; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::vram::calculate_resolution_scale_mem_estimate_text; module=246, inline_record=23456; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::vram::calculate_resolution_scale_mem_estimate_text; module=246, inline_record=23484; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::vram::calculate_resolution_scale_mem_estimate_text; module=246, inline_record=23608; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
