// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/render/nani/buffers.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: d8fa3235393719d3c7928431314120ece43d2b39069f72f90aca6c3df500469b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: buffers.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 512, 514, 518, 522, 1657
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use enum_iterator; // namespace candidate
//   package identities: [{"name":"enum-iterator","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.2.0"}]
// use half; // namespace candidate
//   package identities: [{"name":"half","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.4.1"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::buffers::GpuInstance::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":512,"record_offset":1136,"rva":9753856,"va":5378462976}
// 编译实例 RVAs: 0x94d500
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140954d20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: buffers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378462976,"raw_hex":"e8e6760000","size":5,"soft_fail":false,"target_va":5378493728,"va":5378463285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std_detect::detect::cache::detect_and_initialize
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t38167374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378462976,"raw_hex":"e84e634602","size":5,"soft_fail":false,"target_va":5416630640,"va":5378463261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] half::binary16::f16::from_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"half","version":"2.4.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16.rs
//     evidence={"module":512,"record_offset":1276,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":19898,"source_line":65,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":512,"record_offset":1456,"parent_record_offset":1420,"ancestor_record_offset":1136,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":512,"record_offset":1480,"parent_record_offset":1456,"ancestor_record_offset":1136,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] half::binary16::arch::f32_to_f16
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"half","version":"2.4.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs
//     evidence={"module":512,"record_offset":1300,"parent_record_offset":1276,"ancestor_record_offset":1136,"inlinee":19900,"source_line":64,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] half::binary16::arch::f32_to_f16_fallback
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"half","version":"2.4.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs
//     evidence={"module":512,"record_offset":1556,"parent_record_offset":1300,"ancestor_record_offset":1136,"inlinee":19924,"source_line":482,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] std_detect::detect::arch::x86::__is_feature_detected::f16c
//     owner=unresolved_namespace; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std_detect\src\detect\macros.rs
//     evidence={"module":512,"record_offset":1336,"parent_record_offset":1300,"ancestor_record_offset":1136,"inlinee":19902,"source_line":197,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] std_detect::detect::cache::Cache::test
//     owner=unresolved_namespace; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std_detect\src\detect\cache.rs
//     evidence={"module":512,"record_offset":1420,"parent_record_offset":1392,"ancestor_record_offset":1136,"inlinee":19908,"source_line":90,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] std_detect::detect::cache::test
//     owner=unresolved_namespace; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std_detect\src\detect\cache.rs
//     evidence={"module":512,"record_offset":1392,"parent_record_offset":1364,"ancestor_record_offset":1136,"inlinee":19906,"source_line":194,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] std_detect::detect::cache::test::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std_detect\src\detect\cache.rs
//     evidence={"module":512,"record_offset":1512,"parent_record_offset":1420,"ancestor_record_offset":1136,"inlinee":19910,"source_line":202,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] std_detect::detect::check_for
//     owner=unresolved_namespace; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std_detect\src\detect\mod.rs
//     evidence={"module":512,"record_offset":1364,"parent_record_offset":1336,"ancestor_record_offset":1136,"inlinee":19904,"source_line":89,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::buffers::GpuInstance::set_mesh_atlas_part
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x73FD
// 首条位置: {"module":522,"inlinee":29693}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::geometry::instanced_wall::impl$70::clone_behavior
// 记录形式: compiled_procedure; instances=1
// 源码行: 76
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":512,"record_offset":268,"rva":1065216,"va":5369774336}
// 编译实例 RVAs: 0x104100
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::buffers::_::impl$0::first
// 记录形式: compiled_procedure; instances=1
// 源码行: 76
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":512,"record_offset":1944,"rva":1065216,"va":5369774336}
// 编译实例 RVAs: 0x104100
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::buffers::_::impl$0::next
// 记录形式: compiled_procedure; instances=1
// 源码行: 76
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":512,"record_offset":1816,"rva":9754240,"va":5378463360}
// 编译实例 RVAs: 0x94d680
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::buffers::NaniDrawListPhase::from_pass
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 86
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7401
// 首条位置: {"module":518,"inlinee":29697}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::buffers::NaniDrawListPhase::name
// 记录形式: compiled_procedure; instances=1
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":512,"record_offset":1680,"rva":9754208,"va":5378463328}
// 编译实例 RVAs: 0x94d660
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::buffers::NaniDrawListParams::list_draw_merge_window_range
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 163
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7403
// 首条位置: {"module":1657,"inlinee":29699}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1657,"record_offset":22840,"parent_record_offset":22792,"ancestor_record_offset":22792,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1657,"record_offset":23008,"parent_record_offset":22792,"ancestor_record_offset":22792,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1657,"record_offset":22936,"parent_record_offset":22792,"ancestor_record_offset":22792,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1657,"record_offset":22868,"parent_record_offset":22840,"ancestor_record_offset":22792,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1657,"record_offset":22968,"parent_record_offset":22936,"ancestor_record_offset":22792,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1657,"record_offset":22896,"parent_record_offset":22868,"ancestor_record_offset":22792,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01657.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::buffers::NaniDrawListParams::list_draw_merge_window_subtotal_range
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 173
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7404
// 首条位置: {"module":1657,"inlinee":29700}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1657,"record_offset":23092,"parent_record_offset":23048,"ancestor_record_offset":23048,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1657,"record_offset":23264,"parent_record_offset":23048,"ancestor_record_offset":23048,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1657,"record_offset":23200,"parent_record_offset":23048,"ancestor_record_offset":23048,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1657,"record_offset":23124,"parent_record_offset":23092,"ancestor_record_offset":23048,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1657,"record_offset":23228,"parent_record_offset":23200,"ancestor_record_offset":23048,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1657,"record_offset":23156,"parent_record_offset":23124,"ancestor_record_offset":23048,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01657.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::buffers::nani_calculate_draw_list_params
// 记录形式: compiled_procedure; instances=1
// 源码行: 187
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":512,"record_offset":2076,"rva":9754256,"va":5378463376}
// 编译实例 RVAs: 0x94d690
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-2786248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378463376,"raw_hex":"e8387cd5ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5378464947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=16
//     evidence={"assembly":"\tcallq\t38061159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378463376,"raw_hex":"e867c44402","size":5,"soft_fail":false,"target_va":5416524832,"va":5378463668,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":512,"record_offset":2256,"parent_record_offset":2076,"ancestor_record_offset":2076,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":512,"record_offset":2424,"parent_record_offset":2076,"ancestor_record_offset":2076,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::div_ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":512,"record_offset":2368,"parent_record_offset":2076,"ancestor_record_offset":2076,"inlinee":13025,"source_line":3657,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":512,"record_offset":4272,"parent_record_offset":2076,"ancestor_record_offset":2076,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::capacity::NaniSubsetCapacities::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/capacity.rs
//     evidence={"module":512,"record_offset":2852,"parent_record_offset":2076,"ancestor_record_offset":2076,"inlinee":29624,"source_line":58,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorldSubsetKind::max_draws_per_list
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":512,"record_offset":3268,"parent_record_offset":2076,"ancestor_record_offset":2076,"inlinee":29625,"source_line":383,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorldSubsetKind::use_fine_grained_sorting
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":512,"record_offset":3828,"parent_record_offset":2076,"ancestor_record_offset":2076,"inlinee":29626,"source_line":366,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_direct] enum_iterator::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":512,"record_offset":2796,"parent_record_offset":2076,"ancestor_record_offset":2076,"inlinee":10549,"source_line":221,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":512,"record_offset":4460,"parent_record_offset":4436,"ancestor_record_offset":2076,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":512,"record_offset":4484,"parent_record_offset":4460,"ancestor_record_offset":2076,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":512,"record_offset":4508,"parent_record_offset":4484,"ancestor_record_offset":2076,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":512,"record_offset":4436,"parent_record_offset":4344,"ancestor_record_offset":2076,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":512,"record_offset":2608,"parent_record_offset":2568,"ancestor_record_offset":2076,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":512,"record_offset":4372,"parent_record_offset":4344,"ancestor_record_offset":2076,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":512,"record_offset":4344,"parent_record_offset":4320,"ancestor_record_offset":2076,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":512,"record_offset":2688,"parent_record_offset":2648,"ancestor_record_offset":2076,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":512,"record_offset":2648,"parent_record_offset":2608,"ancestor_record_offset":2076,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":512,"record_offset":4320,"parent_record_offset":4296,"ancestor_record_offset":2076,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":512,"record_offset":2568,"parent_record_offset":2472,"ancestor_record_offset":2076,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":512,"record_offset":2472,"parent_record_offset":2424,"ancestor_record_offset":2076,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":512,"record_offset":4404,"parent_record_offset":4372,"ancestor_record_offset":2076,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":512,"record_offset":2820,"parent_record_offset":2796,"ancestor_record_offset":2076,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":512,"record_offset":4296,"parent_record_offset":4272,"ancestor_record_offset":2076,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":512,"record_offset":2744,"parent_record_offset":2472,"ancestor_record_offset":2076,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::GpuWorldSubsetKind::use_fine_grained_sorting
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":512,"record_offset":3308,"parent_record_offset":3268,"ancestor_record_offset":2076,"inlinee":29626,"source_line":366,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::buffers::NaniDrawListOffsetDefaults::from_params
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 302
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x740B
// 首条位置: {"module":514,"inlinee":29707}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::array::impl$16::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":514,"record_offset":17724,"parent_record_offset":17684,"ancestor_record_offset":17684,"inlinee":11113,"source_line":401,"archive":"_pdb/C/modules/00514.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":514,"record_offset":17856,"parent_record_offset":17684,"ancestor_record_offset":17684,"inlinee":9699,"source_line":4320,"archive":"_pdb/C/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":514,"record_offset":17904,"parent_record_offset":17880,"ancestor_record_offset":17684,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::copy_from_slice_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":514,"record_offset":17880,"parent_record_offset":17856,"ancestor_record_offset":17684,"inlinee":9706,"source_line":5556,"archive":"_pdb/C/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":514,"record_offset":17752,"parent_record_offset":17724,"ancestor_record_offset":17684,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":514,"record_offset":17808,"parent_record_offset":17780,"ancestor_record_offset":17684,"inlinee":8427,"source_line":448,"archive":"_pdb/C/modules/00514.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":514,"record_offset":17780,"parent_record_offset":17752,"ancestor_record_offset":17684,"inlinee":11639,"source_line":533,"archive":"_pdb/C/modules/00514.jsonl.gz"}
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
// 完整结构及原始类型记录: buffers.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::buffers::GpuInstance
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::shadowed_buffer::ShadowedBuffer<country_core::render::nani::buffers::GpuInstance>", "constructor": "country_core::render::shadowed_buffer::ShadowedBuffer", "arguments": ["country_core::render::nani::buffers::GpuInstance"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::render::shadowed_buffer::ShadowedBuffer<country_core::render::nani::buffers::GpuInstance> >", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::render::shadowed_buffer::ShadowedBuffer<country_core::render::nani::buffers::GpuInstance>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::render::nani::buffers::GpuInstance>", "constructor": "country_core::render::shadowed_buffer::ShadowedBuffer::new", "arguments": ["country_core::render::nani::buffers::GpuInstance"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::shadowed_buffer::ShadowedBuffer::grow<country_core::render::nani::buffers::GpuInstance>", "constructor": "country_core::render::shadowed_buffer::ShadowedBuffer::grow", "arguments": ["country_core::render::nani::buffers::GpuInstance"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::buffers::GpuInstance>", "constructor": "country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush", "arguments": ["country_core::render::nani::buffers::GpuInstance"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer::new<country_core::render::nani::buffers::GpuInstance>", "constructor": "country_core::render::ssbo::TypedStorageBuffer::new", "arguments": ["country_core::render::nani::buffers::GpuInstance"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::buffers::NaniDrawListLengthReadbackSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::write_rg_buffer<country_core::render::nani::buffers::NaniDrawListLengthReadbackSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::write_rg_buffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListLengthReadbackSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_write_rg_buffer<country_core::render::nani::buffers::NaniDrawListLengthReadbackSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_write_rg_buffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListLengthReadbackSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::render::nani::buffers::NaniDrawListLengthReadbackSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::render::nani::buffers::NaniDrawListLengthReadbackSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0<country_core::render::nani::buffers::NaniDrawListLengthReadbackSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0", "arguments": ["country_core::render::nani::buffers::NaniDrawListLengthReadbackSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$4::new_storage_buffer<country_core::render::nani::buffers::NaniDrawListLengthReadbackSsbo>", "constructor": "country_core::render::ssbo::impl$4::new_storage_buffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListLengthReadbackSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$1::as_any_mut<country_core::render::nani::buffers::NaniDrawListLengthReadbackSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$1::as_any_mut", "arguments": ["country_core::render::nani::buffers::NaniDrawListLengthReadbackSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$1::as_any<country_core::render::nani::buffers::NaniDrawListLengthReadbackSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$1::as_any", "arguments": ["country_core::render::nani::buffers::NaniDrawListLengthReadbackSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::MappedStorageBuffer<country_core::render::nani::buffers::NaniDrawListLengthReadbackSsbo>", "constructor": "country_core::render::ssbo::MappedStorageBuffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListLengthReadbackSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::MappedStorageBuffer<country_core::render::nani::buffers::NaniDrawListLengthReadbackSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::MappedStorageBuffer<country_core::render::nani::buffers::NaniDrawListLengthReadbackSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::buffers::NaniDrawListLengthSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 11
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<country_core::render::nani::buffers::NaniDrawListLengthSsbo>", "constructor": "ref_mut$", "arguments": ["country_core::render::nani::buffers::NaniDrawListLengthSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$29::fetch<ref_mut$<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>,ref_mut$<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>,ref_mut$<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>,ref_mut$<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>,ref_mut$<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>,ref$<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>,ref$<country_core::render::nani::buffers::NaniDrawListMergeWindows>,ref_mut$<country_core::render::nani::buffers::NaniDrawListLengthSsbo>,ref_mut$<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo>,ref_mut$<country_core::render::nani::buffers::NaniDrawListSsbo>,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::impl$29::fetch", "arguments": ["ref_mut$<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>", "ref_mut$<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>", "ref$<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>", "ref$<country_core::render::nani::buffers::NaniDrawListMergeWindows>", "ref_mut$<country_core::render::nani::buffers::NaniDrawListLengthSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniDrawListSsbo>", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::read_rg_buffer<country_core::render::nani::buffers::NaniDrawListLengthSsbo,rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::read_rg_buffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListLengthSsbo", "rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::write_rg_buffer<country_core::render::nani::buffers::NaniDrawListLengthSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::write_rg_buffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListLengthSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_read_rg_buffer<country_core::render::nani::buffers::NaniDrawListLengthSsbo,rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_read_rg_buffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListLengthSsbo", "rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_write_rg_buffer<country_core::render::nani::buffers::NaniDrawListLengthSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_write_rg_buffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListLengthSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::render::nani::buffers::NaniDrawListLengthSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::render::nani::buffers::NaniDrawListLengthSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0<country_core::render::nani::buffers::NaniDrawListLengthSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0", "arguments": ["country_core::render::nani::buffers::NaniDrawListLengthSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$0::new_storage_buffer<country_core::render::nani::buffers::NaniDrawListLengthSsbo>", "constructor": "country_core::render::ssbo::impl$0::new_storage_buffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListLengthSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut<country_core::render::nani::buffers::NaniDrawListLengthSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut", "arguments": ["country_core::render::nani::buffers::NaniDrawListLengthSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any<country_core::render::nani::buffers::NaniDrawListLengthSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any", "arguments": ["country_core::render::nani::buffers::NaniDrawListLengthSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniDrawListLengthSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListLengthSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniDrawListLengthSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniDrawListLengthSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::buffers::NaniDrawListMergeWindows
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "ref$<country_core::render::nani::buffers::NaniDrawListMergeWindows>", "constructor": "ref$", "arguments": ["country_core::render::nani::buffers::NaniDrawListMergeWindows"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$29::fetch<ref_mut$<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>,ref_mut$<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>,ref_mut$<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>,ref_mut$<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>,ref_mut$<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>,ref$<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>,ref$<country_core::render::nani::buffers::NaniDrawListMergeWindows>,ref_mut$<country_core::render::nani::buffers::NaniDrawListLengthSsbo>,ref_mut$<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo>,ref_mut$<country_core::render::nani::buffers::NaniDrawListSsbo>,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::impl$29::fetch", "arguments": ["ref_mut$<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>", "ref_mut$<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>", "ref$<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>", "ref$<country_core::render::nani::buffers::NaniDrawListMergeWindows>", "ref_mut$<country_core::render::nani::buffers::NaniDrawListLengthSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniDrawListSsbo>", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::read_rg_buffer<country_core::render::nani::buffers::NaniDrawListMergeWindows,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::read_rg_buffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListMergeWindows", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_read_rg_buffer<country_core::render::nani::buffers::NaniDrawListMergeWindows,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_read_rg_buffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListMergeWindows", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::render::nani::buffers::NaniDrawListMergeWindows>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::render::nani::buffers::NaniDrawListMergeWindows"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0<country_core::render::nani::buffers::NaniDrawListMergeWindows>", "constructor": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0", "arguments": ["country_core::render::nani::buffers::NaniDrawListMergeWindows"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$7::new_storage_buffer<country_core::render::nani::buffers::NaniDrawListMergeWindows>", "constructor": "country_core::render::ssbo::impl$7::new_storage_buffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListMergeWindows"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::as_any_mut<country_core::render::nani::buffers::NaniDrawListMergeWindows>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::as_any_mut", "arguments": ["country_core::render::nani::buffers::NaniDrawListMergeWindows"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::as_any<country_core::render::nani::buffers::NaniDrawListMergeWindows>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::as_any", "arguments": ["country_core::render::nani::buffers::NaniDrawListMergeWindows"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::InstanceBuffer<country_core::render::nani::buffers::NaniDrawListMergeWindows>", "constructor": "country_core::render::ssbo::InstanceBuffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListMergeWindows"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::InstanceBuffer<country_core::render::nani::buffers::NaniDrawListMergeWindows> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::InstanceBuffer<country_core::render::nani::buffers::NaniDrawListMergeWindows>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::buffers::NaniDrawListOffsetDefaults
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "ref$<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>", "constructor": "ref$", "arguments": ["country_core::render::nani::buffers::NaniDrawListOffsetDefaults"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$29::fetch<ref_mut$<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>,ref_mut$<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>,ref_mut$<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>,ref_mut$<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>,ref_mut$<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>,ref$<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>,ref$<country_core::render::nani::buffers::NaniDrawListMergeWindows>,ref_mut$<country_core::render::nani::buffers::NaniDrawListLengthSsbo>,ref_mut$<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo>,ref_mut$<country_core::render::nani::buffers::NaniDrawListSsbo>,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::impl$29::fetch", "arguments": ["ref_mut$<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>", "ref_mut$<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>", "ref$<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>", "ref$<country_core::render::nani::buffers::NaniDrawListMergeWindows>", "ref_mut$<country_core::render::nani::buffers::NaniDrawListLengthSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniDrawListSsbo>", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::read_rg_buffer<country_core::render::nani::buffers::NaniDrawListOffsetDefaults,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::read_rg_buffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListOffsetDefaults", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::replace_with_data<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::replace_with_data", "arguments": ["country_core::render::nani::buffers::NaniDrawListOffsetDefaults"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_read_rg_buffer<country_core::render::nani::buffers::NaniDrawListOffsetDefaults,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_read_rg_buffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListOffsetDefaults", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>", "constructor": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0", "arguments": ["country_core::render::nani::buffers::NaniDrawListOffsetDefaults"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$7::new_storage_buffer<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>", "constructor": "country_core::render::ssbo::impl$7::new_storage_buffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListOffsetDefaults"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::as_any_mut<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::as_any_mut", "arguments": ["country_core::render::nani::buffers::NaniDrawListOffsetDefaults"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::as_any<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::as_any", "arguments": ["country_core::render::nani::buffers::NaniDrawListOffsetDefaults"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::InstanceBuffer<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>", "constructor": "country_core::render::ssbo::InstanceBuffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListOffsetDefaults"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::InstanceBuffer<country_core::render::nani::buffers::NaniDrawListOffsetDefaults> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::InstanceBuffer<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::buffers::NaniDrawListParams
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::ArcInner<country_core::render::nani::buffers::NaniDrawListParams>", "constructor": "alloc::sync::ArcInner", "arguments": ["country_core::render::nani::buffers::NaniDrawListParams"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::sync::ArcInner<country_core::render::nani::buffers::NaniDrawListParams> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::sync::ArcInner<country_core::render::nani::buffers::NaniDrawListParams>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::render::nani::buffers::NaniDrawListParams>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::render::nani::buffers::NaniDrawListParams"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<country_core::render::nani::buffers::NaniDrawListParams,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["country_core::render::nani::buffers::NaniDrawListParams", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::buffers::NaniDrawListPhase
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 1
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::buffers::NaniDrawListSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 11
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<country_core::render::nani::buffers::NaniDrawListSsbo>", "constructor": "ref_mut$", "arguments": ["country_core::render::nani::buffers::NaniDrawListSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$29::fetch<ref_mut$<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>,ref_mut$<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>,ref_mut$<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>,ref_mut$<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>,ref_mut$<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>,ref$<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>,ref$<country_core::render::nani::buffers::NaniDrawListMergeWindows>,ref_mut$<country_core::render::nani::buffers::NaniDrawListLengthSsbo>,ref_mut$<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo>,ref_mut$<country_core::render::nani::buffers::NaniDrawListSsbo>,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::impl$29::fetch", "arguments": ["ref_mut$<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>", "ref_mut$<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>", "ref$<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>", "ref$<country_core::render::nani::buffers::NaniDrawListMergeWindows>", "ref_mut$<country_core::render::nani::buffers::NaniDrawListLengthSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniDrawListSsbo>", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::read_rg_buffer<country_core::render::nani::buffers::NaniDrawListSsbo,rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::read_rg_buffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListSsbo", "rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::write_rg_buffer<country_core::render::nani::buffers::NaniDrawListSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::write_rg_buffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::replace_with_data<country_core::render::nani::buffers::NaniDrawListSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::replace_with_data", "arguments": ["country_core::render::nani::buffers::NaniDrawListSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_read_rg_buffer<country_core::render::nani::buffers::NaniDrawListSsbo,rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_read_rg_buffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListSsbo", "rhapsody::rg::node_builder::draw_stream::RasterStreamRecorderWithPipeline"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_write_rg_buffer<country_core::render::nani::buffers::NaniDrawListSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_write_rg_buffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0<country_core::render::nani::buffers::NaniDrawListSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0", "arguments": ["country_core::render::nani::buffers::NaniDrawListSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$0::new_storage_buffer<country_core::render::nani::buffers::NaniDrawListSsbo>", "constructor": "country_core::render::ssbo::impl$0::new_storage_buffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut<country_core::render::nani::buffers::NaniDrawListSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut", "arguments": ["country_core::render::nani::buffers::NaniDrawListSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any<country_core::render::nani::buffers::NaniDrawListSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any", "arguments": ["country_core::render::nani::buffers::NaniDrawListSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniDrawListSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniDrawListSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniDrawListSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>", "constructor": "ref_mut$", "arguments": ["country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$29::fetch<ref_mut$<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>,ref_mut$<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>,ref_mut$<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>,ref_mut$<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>,ref_mut$<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>,ref$<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>,ref$<country_core::render::nani::buffers::NaniDrawListMergeWindows>,ref_mut$<country_core::render::nani::buffers::NaniDrawListLengthSsbo>,ref_mut$<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo>,ref_mut$<country_core::render::nani::buffers::NaniDrawListSsbo>,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::impl$29::fetch", "arguments": ["ref_mut$<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>", "ref_mut$<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>", "ref$<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>", "ref$<country_core::render::nani::buffers::NaniDrawListMergeWindows>", "ref_mut$<country_core::render::nani::buffers::NaniDrawListLengthSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniDrawListSsbo>", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::write_rg_buffer<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::write_rg_buffer", "arguments": ["country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_write_rg_buffer<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_write_rg_buffer", "arguments": ["country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0", "arguments": ["country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$0::new_storage_buffer<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>", "constructor": "country_core::render::ssbo::impl$0::new_storage_buffer", "arguments": ["country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut", "arguments": ["country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any", "arguments": ["country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo>", "constructor": "ref_mut$", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$29::fetch<ref_mut$<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>,ref_mut$<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>,ref_mut$<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>,ref_mut$<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>,ref_mut$<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>,ref$<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>,ref$<country_core::render::nani::buffers::NaniDrawListMergeWindows>,ref_mut$<country_core::render::nani::buffers::NaniDrawListLengthSsbo>,ref_mut$<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo>,ref_mut$<country_core::render::nani::buffers::NaniDrawListSsbo>,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::impl$29::fetch", "arguments": ["ref_mut$<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>", "ref_mut$<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>", "ref$<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>", "ref$<country_core::render::nani::buffers::NaniDrawListMergeWindows>", "ref_mut$<country_core::render::nani::buffers::NaniDrawListLengthSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniDrawListSsbo>", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::write_rg_buffer<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::write_rg_buffer", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_write_rg_buffer<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_write_rg_buffer", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$0::new_storage_buffer<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo>", "constructor": "country_core::render::ssbo::impl$0::new_storage_buffer", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::buffers::NaniInstanceIndirectionSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 12
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::read_rg_buffer<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::read_rg_buffer", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_read_rg_buffer<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_read_rg_buffer", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>", "constructor": "ref_mut$", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$29::fetch<ref_mut$<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>,ref_mut$<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>,ref_mut$<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>,ref_mut$<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>,ref_mut$<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>,ref$<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>,ref$<country_core::render::nani::buffers::NaniDrawListMergeWindows>,ref_mut$<country_core::render::nani::buffers::NaniDrawListLengthSsbo>,ref_mut$<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo>,ref_mut$<country_core::render::nani::buffers::NaniDrawListSsbo>,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::impl$29::fetch", "arguments": ["ref_mut$<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>", "ref_mut$<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>", "ref$<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>", "ref$<country_core::render::nani::buffers::NaniDrawListMergeWindows>", "ref_mut$<country_core::render::nani::buffers::NaniDrawListLengthSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniDrawListSsbo>", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::write_rg_buffer<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::write_rg_buffer", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::replace_with_data<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::replace_with_data", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_write_rg_buffer<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_write_rg_buffer", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$0::new_storage_buffer<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>", "constructor": "country_core::render::ssbo::impl$0::new_storage_buffer", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>", "constructor": "ref_mut$", "arguments": ["country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$29::fetch<ref_mut$<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>,ref_mut$<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>,ref_mut$<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>,ref_mut$<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>,ref_mut$<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>,ref$<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>,ref$<country_core::render::nani::buffers::NaniDrawListMergeWindows>,ref_mut$<country_core::render::nani::buffers::NaniDrawListLengthSsbo>,ref_mut$<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo>,ref_mut$<country_core::render::nani::buffers::NaniDrawListSsbo>,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::impl$29::fetch", "arguments": ["ref_mut$<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>", "ref_mut$<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>", "ref$<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>", "ref$<country_core::render::nani::buffers::NaniDrawListMergeWindows>", "ref_mut$<country_core::render::nani::buffers::NaniDrawListLengthSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniDrawListSsbo>", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::write_rg_buffer<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::write_rg_buffer", "arguments": ["country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::replace_with_data<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::replace_with_data", "arguments": ["country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_write_rg_buffer<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_write_rg_buffer", "arguments": ["country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0", "arguments": ["country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$0::new_storage_buffer<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>", "constructor": "country_core::render::ssbo::impl$0::new_storage_buffer", "arguments": ["country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut", "arguments": ["country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any", "arguments": ["country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>", "constructor": "ref_mut$", "arguments": ["country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$29::fetch<ref_mut$<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>,ref_mut$<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>,ref_mut$<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>,ref_mut$<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>,ref_mut$<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>,ref$<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>,ref$<country_core::render::nani::buffers::NaniDrawListMergeWindows>,ref_mut$<country_core::render::nani::buffers::NaniDrawListLengthSsbo>,ref_mut$<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo>,ref_mut$<country_core::render::nani::buffers::NaniDrawListSsbo>,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::impl$29::fetch", "arguments": ["ref_mut$<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>", "ref_mut$<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>", "ref$<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>", "ref$<country_core::render::nani::buffers::NaniDrawListMergeWindows>", "ref_mut$<country_core::render::nani::buffers::NaniDrawListLengthSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo>", "ref_mut$<country_core::render::nani::buffers::NaniDrawListSsbo>", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::write_rg_buffer<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::write_rg_buffer", "arguments": ["country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::replace_with_data<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::replace_with_data", "arguments": ["country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_write_rg_buffer<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo,rhapsody::rg::api::GraphApi>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::finish_write_rg_buffer", "arguments": ["country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo", "rhapsody::rg::api::GraphApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$7::rg_state::closure$0", "arguments": ["country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$0::new_storage_buffer<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>", "constructor": "country_core::render::ssbo::impl$0::new_storage_buffer", "arguments": ["country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut", "arguments": ["country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any", "arguments": ["country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: buffers.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: buffers.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::audio::forest_audio::ForestAudioState>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::tree_storage::InteractableForestTree>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::audio::forest_audio::ForestAudioState>", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::tree_storage::InteractableForestTree>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::forest_audio::ForestAudioState>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::tree_storage::InteractableForestTree>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> > (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] tuple$<i64,std::time::Instant> (*)()
// 候选参数类型: []

// ------------------------------------------------------------------
// 函数: country_core::geometry::instanced_wall::impl$70::clone_behavior
// 信息 ID: a23fe90b457e00e7500a30b2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 230 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::drawable::NaniActiveInstance> @ 0x2db39a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::WoodenGate> @ 0x2dbafa -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::FollowMouse> @ 0x2dc94a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::systems::prefab::PrefabInstanceData> @ 0x2dd0aa -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::resources::critters::sheep::SheepAccessory> @ 0x2dd80a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::Ui2DFollowMouse> @ 0x2ddf6a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::Ui2dExtent> @ 0x2de6ba -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::AutoClutter> @ 0x2dee1a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::systems::look_at_camera::LookAtCamera> @ 0x2df57a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::SnappingToSegmentPreview> @ 0x2dfcca -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::SemiTransparent> @ 0x2e041a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> @ 0x2e0b6a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::resources::clutter_meshes::ClutterMarkedForDeletion> @ 0x2e1a1a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier> @ 0x2e216a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::TestCube> @ 0x2e28ca -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::DashedLineLength> @ 0x2e301a -> 0x104100 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::buffers::GpuInstance::new
// 信息 ID: 3c1763d732f02b85fd3869a2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::prefab::create_prefab_gpu_instances @ 0x91069d -> 0x94d500 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::systems::nani_maintain_instances @ 0x25b1c55 -> 0x94d500 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::buffers::GpuInstance::set_mesh_atlas_part
// 信息 ID: 9d65038ff2726f14e756d058
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::update_instance_mesh; module=522, inline_record=25184; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::buffers::NaniDrawListOffsetDefaults::from_params
// 信息 ID: 37516f16385f5bdcca3e42d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::nani::maintenance::nani_upload_derived_buffers; module=514, inline_record=17684; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::buffers::NaniDrawListParams::list_draw_merge_window_range
// 信息 ID: 302c86e8bc3374bab5a5032e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::group_draw_cmd_prefix_sum::closure_env$0<country_core::render::nani::gpu_world::RenderSublistIter>,tuple$<rhapsody::rg::node_builder::compute_pass_builder::ComputeDispatchIndirectApi> >; module=1657, inline_record=22792; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::buffers::NaniDrawListParams::list_draw_merge_window_subtotal_range
// 信息 ID: d988acf61876fa331c4c766b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::group_draw_cmd_prefix_sum::closure_env$0<country_core::render::nani::gpu_world::RenderSublistIter>,tuple$<rhapsody::rg::node_builder::compute_pass_builder::ComputeDispatchIndirectApi> >; module=1657, inline_record=23048; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::group_draw_cmd_prefix_sum::closure_env$1<country_core::render::nani::gpu_world::RenderSublistIter>,tuple$<rhapsody::rg::node_builder::compute_pass_builder::ComputeDispatchIndirectApi> >; module=1657, inline_record=29892; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::buffers::NaniDrawListPhase::from_pass
// 信息 ID: c5003c5c17de3af24e573d20
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists; module=518, inline_record=34832; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::buffers::NaniDrawListPhase::name
// 信息 ID: 4d7e36a60ccdfea3f222ac7e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::nani_capacity_debug_ui::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x54be9e -> 0x94d660 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::buffers::_::impl$0::first
// 信息 ID: 7785be4a22da6275fe4a1c41
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 230 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::drawable::NaniActiveInstance> @ 0x2db39a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::WoodenGate> @ 0x2dbafa -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::FollowMouse> @ 0x2dc94a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::systems::prefab::PrefabInstanceData> @ 0x2dd0aa -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::resources::critters::sheep::SheepAccessory> @ 0x2dd80a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::Ui2DFollowMouse> @ 0x2ddf6a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::Ui2dExtent> @ 0x2de6ba -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::AutoClutter> @ 0x2dee1a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::systems::look_at_camera::LookAtCamera> @ 0x2df57a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::SnappingToSegmentPreview> @ 0x2dfcca -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::SemiTransparent> @ 0x2e041a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> @ 0x2e0b6a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::resources::clutter_meshes::ClutterMarkedForDeletion> @ 0x2e1a1a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier> @ 0x2e216a -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::TestCube> @ 0x2e28ca -> 0x104100 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::register_component<country_core::components::DashedLineLength> @ 0x2e301a -> 0x104100 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::buffers::_::impl$0::next
// 信息 ID: 250e6548a8c091177e30411f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 22 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x224d4a -> 0x94d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x225025 -> 0x94d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::render::render_config::UpscalingMode> > @ 0x2252da -> 0x94d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::render::render_config::UpscalingMode> > @ 0x2255b5 -> 0x94d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::render::render_config::UpscalingPreset> > @ 0x225e5a -> 0x94d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::render::render_config::UpscalingPreset> > @ 0x226135 -> 0x94d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x227a2a -> 0x94d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x227d05 -> 0x94d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::systems::main_camera::CameraRotationPivot> > @ 0x22905a -> 0x94d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::systems::main_camera::CameraRotationPivot> > @ 0x229335 -> 0x94d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<alloc::string::String,country_core::resources::clutter_meshes::ClutterMeshTy>,core::iter::adapters::filter_map::FilterMap<enum_iterator::All<country_core::resources::clutter_meshes::ClutterMeshTy>,tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure$0::closure$4::closure_env$0> > @ 0x2fe6bf -> 0x94d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<alloc::string::String,country_core::resources::clutter_meshes::ClutterMeshTy>,core::iter::adapters::filter_map::FilterMap<enum_iterator::All<country_core::resources::clutter_meshes::ClutterMeshTy>,tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure$0::closure$4::closure_env$0> > @ 0x2fe7a3 -> 0x94d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::nani_capacity_debug_ui::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x54b791 -> 0x94d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::nani_capacity_debug_ui::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x54bbf9 -> 0x94d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::nani_capacity_debug_ui::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x54be94 -> 0x94d680 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure$0::closure$17::closure$1::closure_env$0,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x58d2a0 -> 0x94d680 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::buffers::nani_calculate_draw_list_params
// 信息 ID: d1e1a28526ae5c6993fc66cf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::nani::gpu_world::GpuWorld::new @ 0x9e123d -> 0x94d690 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::world::World::resource_scope<country_core::render::nani::gpu_world::GpuWorld,tuple$<>,country_core::render::nani::maintenance::nani_frame_boundary_maintenance::closure_env$1> @ 0x19ec6ce -> 0x94d690 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
