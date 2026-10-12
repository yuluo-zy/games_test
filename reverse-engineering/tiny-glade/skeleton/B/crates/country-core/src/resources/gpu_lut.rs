// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/resources/gpu_lut.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 5d9cd8fc7148d10607ffa596103314f77d35c8632c30a73b7c290618faee103f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: gpu_lut.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 504, 553
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_app; // namespace candidate
//   package identities: [{"name":"bevy_app","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::gpu_lut::impl$0::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":504,"record_offset":4480,"rva":9494656,"va":5378203776}
// 编译实例 RVAs: 0x90e080
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14088c000
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gpu_lut.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-533055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378203776,"raw_hex":"e8c1ddf7ff","size":5,"soft_fail":false,"target_va":5377671168,"va":5378204218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140896610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gpu_lut.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-490504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378203776,"raw_hex":"e8f883f8ff","size":5,"soft_fail":false,"target_va":5377713680,"va":5378204179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-3847939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378203776,"raw_hex":"e8fd48c5ff","size":5,"soft_fail":false,"target_va":5374355952,"va":5378203886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12840399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378203776,"raw_hex":"e8cfedc300","size":5,"soft_fail":false,"target_va":5391044528,"va":5378204124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":504,"record_offset":4620,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":504,"record_offset":4676,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::gpu_lut::GpuLutDesc::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gpu_lut.rs
//     evidence={"module":504,"record_offset":4648,"parent_record_offset":4480,"ancestor_record_offset":4480,"inlinee":28498,"source_line":80,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":504,"record_offset":4740,"parent_record_offset":4716,"ancestor_record_offset":4480,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":504,"record_offset":4788,"parent_record_offset":4764,"ancestor_record_offset":4480,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":504,"record_offset":4764,"parent_record_offset":4740,"ancestor_record_offset":4480,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":504,"record_offset":4716,"parent_record_offset":4676,"ancestor_record_offset":4480,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":504,"record_offset":4828,"parent_record_offset":4676,"ancestor_record_offset":4480,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure_env$0,tuple$<ref_mut$<dyn$<country_core::resources::gpu_lut::GpuLutApi> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":553,"record_offset":28720,"rva":10950336,"va":5379659456}
// 编译实例 RVAs: 0xa716c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5379659456,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5379659518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":553,"record_offset":28980,"parent_record_offset":28720,"ancestor_record_offset":28720,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::gpu_lut::impl$0::build::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gpu_lut.rs
//     evidence={"module":553,"record_offset":29000,"parent_record_offset":28980,"ancestor_record_offset":28720,"inlinee":31598,"source_line":28,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::gpu_lut::impl$1::compute
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gpu_lut.rs
//     evidence={"module":553,"record_offset":29024,"parent_record_offset":29000,"ancestor_record_offset":28720,"inlinee":31600,"source_line":63,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::gpu_lut::impl$0::build::closure$0
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7B6E
// 首条位置: {"module":553,"inlinee":31598}
// 编译实例 RVAs: 0xa716c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5379659456,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5379659518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::resources::gpu_lut::impl$1::compute
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gpu_lut.rs
//     evidence={"module":553,"record_offset":29024,"parent_record_offset":29000,"ancestor_record_offset":29000,"inlinee":31600,"source_line":63,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::gpu_lut::impl$0::build::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7B72
// 首条位置: {"module":553,"inlinee":31602}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":553,"record_offset":29840,"parent_record_offset":29764,"ancestor_record_offset":29764,"inlinee":26886,"source_line":452,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":553,"record_offset":29792,"parent_record_offset":29764,"ancestor_record_offset":29764,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":553,"record_offset":29936,"parent_record_offset":29868,"ancestor_record_offset":29764,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":553,"record_offset":29868,"parent_record_offset":29840,"ancestor_record_offset":29764,"inlinee":26676,"source_line":485,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure_env$1,tuple$<ref_mut$<dyn$<country_core::resources::gpu_lut::GpuLutApi> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":553,"record_offset":30176,"rva":10951040,"va":5379660160}
// 编译实例 RVAs: 0xa71980
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5379660160,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5379660222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":553,"record_offset":30436,"parent_record_offset":30176,"ancestor_record_offset":30176,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::gpu_lut::impl$0::build::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gpu_lut.rs
//     evidence={"module":553,"record_offset":30456,"parent_record_offset":30436,"ancestor_record_offset":30176,"inlinee":31603,"source_line":36,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::gpu_lut::impl$1::compute
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gpu_lut.rs
//     evidence={"module":553,"record_offset":30480,"parent_record_offset":30456,"ancestor_record_offset":30176,"inlinee":31600,"source_line":63,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::gpu_lut::impl$0::build::closure$1
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7B73
// 首条位置: {"module":553,"inlinee":31603}
// 编译实例 RVAs: 0xa71980
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5379660160,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5379660222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::resources::gpu_lut::impl$1::compute
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gpu_lut.rs
//     evidence={"module":553,"record_offset":30480,"parent_record_offset":30456,"ancestor_record_offset":30456,"inlinee":31600,"source_line":63,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::gpu_lut::impl$0::build::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7B6C
// 首条位置: {"module":553,"inlinee":31596}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":553,"record_offset":27956,"parent_record_offset":27832,"ancestor_record_offset":27832,"inlinee":26886,"source_line":452,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":553,"record_offset":27860,"parent_record_offset":27832,"ancestor_record_offset":27832,"inlinee":12552,"source_line":171,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":553,"record_offset":27908,"parent_record_offset":27832,"ancestor_record_offset":27832,"inlinee":12551,"source_line":704,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":553,"record_offset":28056,"parent_record_offset":27984,"ancestor_record_offset":27832,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":553,"record_offset":27984,"parent_record_offset":27956,"ancestor_record_offset":27832,"inlinee":26676,"source_line":485,"archive":"_pdb/B/modules/00553.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::gpu_lut::impl$1::compute
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7B70
// 首条位置: {"module":553,"inlinee":31600}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::gpu_lut::GpuLutDesc::new
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 80
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6F52
// 首条位置: {"module":504,"inlinee":28498}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::gpu_lut::GpuLutTracker::render_luts
// 记录形式: compiled_procedure; instances=1
// 源码行: 95
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":504,"record_offset":5080,"rva":9495168,"va":5378204288}
// 编译实例 RVAs: 0x90e280
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*40(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5378204288,"raw_hex":"ff5028","size":3,"soft_fail":false,"va":5378204406,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$30::call
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":504,"record_offset":5416,"parent_record_offset":5080,"ancestor_record_offset":5080,"inlinee":8202,"source_line":1989,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":504,"record_offset":5388,"parent_record_offset":5080,"ancestor_record_offset":5080,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":504,"record_offset":5236,"parent_record_offset":5080,"ancestor_record_offset":5080,"inlinee":7678,"source_line":3482,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":504,"record_offset":5312,"parent_record_offset":5080,"ancestor_record_offset":5080,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":504,"record_offset":5276,"parent_record_offset":5256,"ancestor_record_offset":5080,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":504,"record_offset":5256,"parent_record_offset":5236,"ancestor_record_offset":5080,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":504,"record_offset":5352,"parent_record_offset":5312,"ancestor_record_offset":5080,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::gpu_lut::impl$3::render_luts::impl$0::compute_impl
// 记录形式: compiled_procedure; instances=1
// 源码行: 112
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":504,"record_offset":5448,"rva":9495312,"va":5378204432}
// 编译实例 RVAs: 0x90e310
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14002e900
//     owner=unresolved_namespace; count=1
//     target_aliases=130 candidates; complete list: gpu_lut.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9305077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378204432,"raw_hex":"e80b0472ff","size":5,"soft_fail":false,"target_va":5368899840,"va":5378204912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: gpu_lut.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3123313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378204432,"raw_hex":"e88f57d0ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5378204812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141600570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gpu_lut.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13574560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378204432,"raw_hex":"e8a021cf00","size":5,"soft_fail":false,"target_va":5391779184,"va":5378204619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: gpu_lut.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33642519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378204432,"raw_hex":"e817580102","size":5,"soft_fail":false,"target_va":5411847168,"va":5378204644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-3848984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378204432,"raw_hex":"e8e844c5ff","size":5,"soft_fail":false,"target_va":5374355568,"va":5378204547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: gpu_lut.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-3123214","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5378204432,"raw_hex":"e9f257d0ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5378204713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5378204432,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5378204516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$28::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":504,"record_offset":5792,"parent_record_offset":5448,"ancestor_record_offset":5448,"inlinee":7605,"source_line":1975,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":504,"record_offset":5612,"parent_record_offset":5448,"ancestor_record_offset":5448,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":504,"record_offset":5928,"parent_record_offset":5904,"ancestor_record_offset":5448,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":504,"record_offset":5904,"parent_record_offset":5820,"ancestor_record_offset":5448,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":504,"record_offset":5820,"parent_record_offset":5792,"ancestor_record_offset":5448,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":504,"record_offset":5672,"parent_record_offset":5648,"ancestor_record_offset":5448,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":504,"record_offset":5720,"parent_record_offset":5696,"ancestor_record_offset":5448,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":504,"record_offset":5696,"parent_record_offset":5672,"ancestor_record_offset":5448,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":504,"record_offset":5648,"parent_record_offset":5612,"ancestor_record_offset":5448,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":504,"record_offset":5848,"parent_record_offset":5820,"ancestor_record_offset":5448,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":504,"record_offset":5872,"parent_record_offset":5848,"ancestor_record_offset":5448,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":504,"record_offset":5760,"parent_record_offset":5612,"ancestor_record_offset":5448,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00504.jsonl.gz"}
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
// 完整结构及原始类型记录: gpu_lut.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::gpu_lut::GpuLutApi
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "dyn$<country_core::resources::gpu_lut::GpuLutApi>", "constructor": "dyn$", "arguments": ["country_core::resources::gpu_lut::GpuLutApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<dyn$<country_core::resources::gpu_lut::GpuLutApi> >", "constructor": "ref_mut$", "arguments": ["dyn$<country_core::resources::gpu_lut::GpuLutApi>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref_mut$<dyn$<country_core::resources::gpu_lut::GpuLutApi> > >", "constructor": "tuple$", "arguments": ["ref_mut$<dyn$<country_core::resources::gpu_lut::GpuLutApi> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure_env$0,tuple$<ref_mut$<dyn$<country_core::resources::gpu_lut::GpuLutApi> > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::resources::gpu_lut::impl$0::build::closure_env$0", "tuple$<ref_mut$<dyn$<country_core::resources::gpu_lut::GpuLutApi> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure_env$1,tuple$<ref_mut$<dyn$<country_core::resources::gpu_lut::GpuLutApi> > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::resources::gpu_lut::impl$0::build::closure_env$1", "tuple$<ref_mut$<dyn$<country_core::resources::gpu_lut::GpuLutApi> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::gpu_lut::GpuLutDesc
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::gpu_lut::GpuLutDesc>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::gpu_lut::GpuLutDesc"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::resources::gpu_lut::GpuLutDesc,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::resources::gpu_lut::GpuLutDesc", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<country_core::resources::gpu_lut::GpuLutDesc,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<country_core::resources::gpu_lut::GpuLutDesc,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<country_core::resources::gpu_lut::GpuLutDesc,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["country_core::resources::gpu_lut::GpuLutDesc", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::gpu_lut::GpuLutPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$12::add_to_app<bevy_app::plugin::sealed::PluginMarker,tiny_glade::decorator_plugin::DecoratorPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::backend::icc::IccProfilePlugin,bevy_app::plugin::sealed::PluginMarker,system_critters::CritterPlugin,bevy_app::plugin::sealed::PluginMarker,system_critters::sheep::SheepPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::gpu_lut::GpuLutPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::components::drawable::PrevTransformPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::render::fsr_plugin::plugin_impl::FsrPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::dlss_plugin::plugin_impl::DlssPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::rollercoaster::RollercoasterPlugin>", "constructor": "bevy_app::plugin::sealed::impl$12::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "tiny_glade::decorator_plugin::DecoratorPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::backend::icc::IccProfilePlugin", "bevy_app::plugin::sealed::PluginMarker", "system_critters::CritterPlugin", "bevy_app::plugin::sealed::PluginMarker", "system_critters::sheep::SheepPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::gpu_lut::GpuLutPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::components::drawable::PrevTransformPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::render::fsr_plugin::plugin_impl::FsrPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::dlss_plugin::plugin_impl::DlssPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::rollercoaster::RollercoasterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::resources::gpu_lut::GpuLutPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::resources::gpu_lut::GpuLutPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<country_core::resources::gpu_lut::GpuLutPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["country_core::resources::gpu_lut::GpuLutPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<country_core::resources::gpu_lut::GpuLutPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["country_core::resources::gpu_lut::GpuLutPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<country_core::resources::gpu_lut::GpuLutPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["country_core::resources::gpu_lut::GpuLutPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<country_core::resources::gpu_lut::GpuLutPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["country_core::resources::gpu_lut::GpuLutPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::ready<country_core::resources::gpu_lut::GpuLutPlugin>", "constructor": "bevy_app::plugin::Plugin::ready", "arguments": ["country_core::resources::gpu_lut::GpuLutPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::finish<country_core::resources::gpu_lut::GpuLutPlugin>", "constructor": "bevy_app::plugin::Plugin::finish", "arguments": ["country_core::resources::gpu_lut::GpuLutPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::is_unique<country_core::resources::gpu_lut::GpuLutPlugin>", "constructor": "bevy_app::plugin::Plugin::is_unique", "arguments": ["country_core::resources::gpu_lut::GpuLutPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::gpu_lut::GpuLutTracker
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::resources::gpu_lut::GpuLutTracker>", "constructor": "bevy_ecs::component::ComponentDescriptor::drop_ptr", "arguments": ["country_core::resources::gpu_lut::GpuLutTracker"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::app::App::insert_resource<country_core::resources::gpu_lut::GpuLutTracker>", "constructor": "bevy_app::app::App::insert_resource", "arguments": ["country_core::resources::gpu_lut::GpuLutTracker"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: gpu_lut.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: gpu_lut.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure_env$0,tuple$<ref_mut$<dyn$<country_core::resources::gpu_lut::GpuLutApi> > > >
// 信息 ID: 58e5927bfe71e4c02eec1cbf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] dyn$ -> ["country_core::resources::gpu_lut::GpuLutApi"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["dyn$<country_core::resources::gpu_lut::GpuLutApi>"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["ref_mut$<dyn$<country_core::resources::gpu_lut::GpuLutApi> >"]; 字段布局 = unknown。
// [类型关系] core::ops::function::FnOnce::call_once -> ["country_core::resources::gpu_lut::impl$0::build::closure_env$0", "tuple$<ref_mut$<dyn$<country_core::resources::gpu_lut::GpuLutApi> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure_env$1,tuple$<ref_mut$<dyn$<country_core::resources::gpu_lut::GpuLutApi> > > >
// 信息 ID: ebf3cfae8b0e5634e7ad58be
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] dyn$ -> ["country_core::resources::gpu_lut::GpuLutApi"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["dyn$<country_core::resources::gpu_lut::GpuLutApi>"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["ref_mut$<dyn$<country_core::resources::gpu_lut::GpuLutApi> >"]; 字段布局 = unknown。
// [类型关系] core::ops::function::FnOnce::call_once -> ["country_core::resources::gpu_lut::impl$0::build::closure_env$1", "tuple$<ref_mut$<dyn$<country_core::resources::gpu_lut::GpuLutApi> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::gpu_lut::GpuLutDesc::new
// 信息 ID: 347919999f490e755fff6d35
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::gpu_lut::impl$0::build; module=504, inline_record=4648; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::gpu_lut::impl$0::build; module=504, inline_record=4860; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::gpu_lut::GpuLutTracker::render_luts
// 信息 ID: 3692bc32527c468eededcffe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x1622e7 -> 0x90e280 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::gpu_lut::impl$0::build
// 信息 ID: 38d5cdcfbdf41868bd80a381
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::gpu_lut::impl$0::build::closure$0
// 信息 ID: 9dc16b5f2c5e753fb7c40dc4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::gpu_lut::impl$0::build", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::gpu_lut::impl$0::build::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure_env$0,tuple$<ref_mut$<dyn$<country_core::resources::gpu_lut::GpuLutApi> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure_env$0,tuple$<ref_mut$<dyn$<country_core::resources::gpu_lut::GpuLutApi> > > >; module=553, inline_record=29000; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::gpu_lut::impl$0::build::closure$0::closure$0
// 信息 ID: 5ea46a12702b97eacefc06f7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::gpu_lut::impl$0::build", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::gpu_lut::impl$0::build::closure$0::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure$0::closure_env$0,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputePass> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure$0::closure_env$0,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputePass> >; module=553, inline_record=29764; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::gpu_lut::impl$0::build::closure$1
// 信息 ID: e5f3d2e7158f6e06e77161c5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::resources::gpu_lut::impl$0::build", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::gpu_lut::impl$0::build::closure_env$1 <- core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure_env$1,tuple$<ref_mut$<dyn$<country_core::resources::gpu_lut::GpuLutApi> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure_env$1,tuple$<ref_mut$<dyn$<country_core::resources::gpu_lut::GpuLutApi> > > >; module=553, inline_record=30456; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::gpu_lut::impl$0::build::closure$1::closure$0
// 信息 ID: 34612f14b368658de274a339
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::resources::gpu_lut::impl$0::build", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::gpu_lut::impl$0::build::closure$1::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure$1::closure_env$0,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputePass> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure$1::closure_env$0,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputePass> >; module=553, inline_record=27832; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::gpu_lut::impl$1::compute
// 信息 ID: 8a0dfde75eaf8608f8d6d6bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure_env$0,tuple$<ref_mut$<dyn$<country_core::resources::gpu_lut::GpuLutApi> > > >; module=553, inline_record=29024; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure_env$1,tuple$<ref_mut$<dyn$<country_core::resources::gpu_lut::GpuLutApi> > > >; module=553, inline_record=30480; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::gpu_lut::impl$0::build::closure$0; module=553, inline_record=34416; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::gpu_lut::impl$0::build::closure$1; module=553, inline_record=34592; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::gpu_lut::impl$3::render_luts::impl$0::compute_impl
// 信息 ID: c4a959263285d847f679e2af
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
