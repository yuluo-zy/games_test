// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/resources/gpu_lut.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: a7026a44a1cf9bcb5f7a5cd4ce1fbdeb38aa36cecf0babc2be9d43bf92de3cca
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: gpu_lut.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 413, 1360
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::gpu_lut::impl$0::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 25
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":413,"record_offset":87308,"rva":5757200,"va":5374466320}
// 编译实例 RVAs: 0x57d910
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e77c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gpu_lut.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1482020","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374466320,"raw_hex":"e8249d1600","size":5,"soft_fail":false,"target_va":5375948736,"va":5374466711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e8530
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: gpu_lut.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1485499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374466320,"raw_hex":"e8bbaa1600","size":5,"soft_fail":false,"target_va":5375952176,"va":5374466672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-3158760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374466320,"raw_hex":"e818cdcfff","size":5,"soft_fail":false,"target_va":5371307664,"va":5374466419,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8607647","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374466320,"raw_hex":"e89f578300","size":5,"soft_fail":false,"target_va":5383074272,"va":5374466620,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":413,"record_offset":87448,"parent_record_offset":87308,"ancestor_record_offset":87308,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00413.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":413,"record_offset":87504,"parent_record_offset":87308,"ancestor_record_offset":87308,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00413.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::gpu_lut::GpuLutDesc::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gpu_lut.rs
//     evidence={"module":413,"record_offset":87476,"parent_record_offset":87308,"ancestor_record_offset":87308,"inlinee":23687,"source_line":80,"archive":"_pdb/A/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":413,"record_offset":87576,"parent_record_offset":87544,"ancestor_record_offset":87308,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":413,"record_offset":87544,"parent_record_offset":87504,"ancestor_record_offset":87308,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":413,"record_offset":87608,"parent_record_offset":87504,"ancestor_record_offset":87308,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00413.jsonl.gz"}
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
// 首条位置: {"module":1360,"record_offset":58672,"rva":15810960,"va":5384520080}
// 编译实例 RVAs: 0xf14190
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5384520080,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5384520142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1360,"record_offset":58932,"parent_record_offset":58672,"ancestor_record_offset":58672,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/01360.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::gpu_lut::impl$0::build::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gpu_lut.rs
//     evidence={"module":1360,"record_offset":58952,"parent_record_offset":58932,"ancestor_record_offset":58672,"inlinee":63304,"source_line":28,"archive":"_pdb/A/modules/01360.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::gpu_lut::impl$1::compute
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gpu_lut.rs
//     evidence={"module":1360,"record_offset":58976,"parent_record_offset":58952,"ancestor_record_offset":58672,"inlinee":63301,"source_line":63,"archive":"_pdb/A/modules/01360.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::gpu_lut::impl$0::build::closure$0
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF748
// 首条位置: {"module":1360,"inlinee":63304}
// 编译实例 RVAs: 0xf14190
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5384520080,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5384520142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::resources::gpu_lut::impl$1::compute
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gpu_lut.rs
//     evidence={"module":1360,"record_offset":58976,"parent_record_offset":58952,"ancestor_record_offset":58952,"inlinee":63301,"source_line":63,"archive":"_pdb/A/modules/01360.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::gpu_lut::impl$0::build::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF747
// 首条位置: {"module":1360,"inlinee":63303}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":1360,"record_offset":58336,"parent_record_offset":58260,"ancestor_record_offset":58260,"inlinee":22040,"source_line":444,"archive":"_pdb/A/modules/01360.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":1360,"record_offset":58288,"parent_record_offset":58260,"ancestor_record_offset":58260,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/01360.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1360,"record_offset":58432,"parent_record_offset":58364,"ancestor_record_offset":58260,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01360.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":1360,"record_offset":58364,"parent_record_offset":58336,"ancestor_record_offset":58260,"inlinee":21934,"source_line":477,"archive":"_pdb/A/modules/01360.jsonl.gz"}
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
// 首条位置: {"module":1360,"record_offset":57640,"rva":15810304,"va":5384519424}
// 编译实例 RVAs: 0xf13f00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5384519424,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5384519486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1360,"record_offset":57900,"parent_record_offset":57640,"ancestor_record_offset":57640,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/01360.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::gpu_lut::impl$0::build::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gpu_lut.rs
//     evidence={"module":1360,"record_offset":57920,"parent_record_offset":57900,"ancestor_record_offset":57640,"inlinee":63299,"source_line":36,"archive":"_pdb/A/modules/01360.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::gpu_lut::impl$1::compute
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gpu_lut.rs
//     evidence={"module":1360,"record_offset":57944,"parent_record_offset":57920,"ancestor_record_offset":57640,"inlinee":63301,"source_line":63,"archive":"_pdb/A/modules/01360.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::gpu_lut::impl$0::build::closure$1
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF743
// 首条位置: {"module":1360,"inlinee":63299}
// 编译实例 RVAs: 0xf13f00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5384519424,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5384519486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::resources::gpu_lut::impl$1::compute
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gpu_lut.rs
//     evidence={"module":1360,"record_offset":57944,"parent_record_offset":57920,"ancestor_record_offset":57920,"inlinee":63301,"source_line":63,"archive":"_pdb/A/modules/01360.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::gpu_lut::impl$0::build::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF74A
// 首条位置: {"module":1360,"inlinee":63306}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_2d
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":1360,"record_offset":59416,"parent_record_offset":59292,"ancestor_record_offset":59292,"inlinee":22040,"source_line":444,"archive":"_pdb/A/modules/01360.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::ComputePass::write_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":1360,"record_offset":59320,"parent_record_offset":59292,"ancestor_record_offset":59292,"inlinee":9729,"source_line":163,"archive":"_pdb/A/modules/01360.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::compute_pass_builder::impl$3::read_texture
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":1360,"record_offset":59368,"parent_record_offset":59292,"ancestor_record_offset":59292,"inlinee":9728,"source_line":688,"archive":"_pdb/A/modules/01360.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1360,"record_offset":59516,"parent_record_offset":59444,"ancestor_record_offset":59292,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01360.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::ComputePass::dispatch_impl
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":1360,"record_offset":59444,"parent_record_offset":59416,"ancestor_record_offset":59292,"inlinee":21934,"source_line":477,"archive":"_pdb/A/modules/01360.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::gpu_lut::impl$1::compute
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF745
// 首条位置: {"module":1360,"inlinee":63301}
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
// IPI IDs: 0x5C87
// 首条位置: {"module":413,"inlinee":23687}
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
// 首条位置: {"module":413,"record_offset":87816,"rva":5757664,"va":5374466784}
// 编译实例 RVAs: 0x57dae0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*40(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5374466784,"raw_hex":"ff5028","size":3,"soft_fail":false,"va":5374466902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$49::call
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":413,"record_offset":88104,"parent_record_offset":87816,"ancestor_record_offset":87816,"inlinee":8413,"source_line":2033,"archive":"_pdb/A/modules/00413.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":413,"record_offset":88076,"parent_record_offset":87816,"ancestor_record_offset":87816,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/00413.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":413,"record_offset":87972,"parent_record_offset":87816,"ancestor_record_offset":87816,"inlinee":6287,"source_line":2952,"archive":"_pdb/A/modules/00413.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":413,"record_offset":87996,"parent_record_offset":87816,"ancestor_record_offset":87816,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":413,"record_offset":88040,"parent_record_offset":87996,"ancestor_record_offset":87816,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00413.jsonl.gz"}
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
// 首条位置: {"module":413,"record_offset":88136,"rva":5757808,"va":5374466928}
// 编译实例 RVAs: 0x57db70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140005a60
//     owner=unresolved_namespace; count=1
//     target_aliases=118 candidates; complete list: gpu_lut.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5735137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374466928,"raw_hex":"e81f7da8ff","size":5,"soft_fail":false,"target_va":5368732256,"va":5374467388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: gpu_lut.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1717583","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374466928,"raw_hex":"e8b1cae5ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374467290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dcd7a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gpu_lut.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8715128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374466928,"raw_hex":"e878fb8400","size":5,"soft_fail":false,"target_va":5383182240,"va":5374467107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: gpu_lut.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29126223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374466928,"raw_hex":"e84f6ebc01","size":5,"soft_fail":false,"target_va":5403593360,"va":5374467132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-3161983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374466928,"raw_hex":"e881c0cfff","size":5,"soft_fail":false,"target_va":5371305056,"va":5374467034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: gpu_lut.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-1717492","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5374466928,"raw_hex":"e90ccbe5ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374467199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5374466928,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5374467010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$47::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":413,"record_offset":88420,"parent_record_offset":88136,"ancestor_record_offset":88136,"inlinee":5521,"source_line":2019,"archive":"_pdb/A/modules/00413.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":413,"record_offset":88300,"parent_record_offset":88136,"ancestor_record_offset":88136,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":413,"record_offset":88556,"parent_record_offset":88532,"ancestor_record_offset":88136,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":413,"record_offset":88532,"parent_record_offset":88448,"ancestor_record_offset":88136,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":413,"record_offset":88448,"parent_record_offset":88420,"ancestor_record_offset":88136,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":413,"record_offset":88356,"parent_record_offset":88332,"ancestor_record_offset":88136,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":413,"record_offset":88332,"parent_record_offset":88300,"ancestor_record_offset":88136,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":413,"record_offset":88476,"parent_record_offset":88448,"ancestor_record_offset":88136,"inlinee":4115,"source_line":199,"archive":"_pdb/A/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":413,"record_offset":88500,"parent_record_offset":88476,"ancestor_record_offset":88136,"inlinee":4117,"source_line":392,"archive":"_pdb/A/modules/00413.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":413,"record_offset":88388,"parent_record_offset":88300,"ancestor_record_offset":88136,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00413.jsonl.gz"}
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
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure_env$1,tuple$<ref_mut$<dyn$<country_core::resources::gpu_lut::GpuLutApi> > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::resources::gpu_lut::impl$0::build::closure_env$1", "tuple$<ref_mut$<dyn$<country_core::resources::gpu_lut::GpuLutApi> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure_env$0,tuple$<ref_mut$<dyn$<country_core::resources::gpu_lut::GpuLutApi> > > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::resources::gpu_lut::impl$0::build::closure_env$0", "tuple$<ref_mut$<dyn$<country_core::resources::gpu_lut::GpuLutApi> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::gpu_lut::GpuLutDesc
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::gpu_lut::GpuLutDesc>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::gpu_lut::GpuLutDesc"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::resources::gpu_lut::GpuLutDesc,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::resources::gpu_lut::GpuLutDesc", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<country_core::resources::gpu_lut::GpuLutDesc,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<country_core::resources::gpu_lut::GpuLutDesc,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<country_core::resources::gpu_lut::GpuLutDesc,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["country_core::resources::gpu_lut::GpuLutDesc", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::gpu_lut::GpuLutPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 7
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$7::add_to_app<bevy_app::plugin::sealed::PluginMarker,country_core::systems::backend::icc::IccProfilePlugin,bevy_app::plugin::sealed::PluginMarker,system_critters::CritterPlugin,bevy_app::plugin::sealed::PluginMarker,system_critters::sheep::SheepPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::gpu_lut::GpuLutPlugin>", "constructor": "bevy_app::plugin::sealed::impl$7::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "country_core::systems::backend::icc::IccProfilePlugin", "bevy_app::plugin::sealed::PluginMarker", "system_critters::CritterPlugin", "bevy_app::plugin::sealed::PluginMarker", "system_critters::sheep::SheepPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::gpu_lut::GpuLutPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::resources::gpu_lut::GpuLutPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::resources::gpu_lut::GpuLutPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<country_core::resources::gpu_lut::GpuLutPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["country_core::resources::gpu_lut::GpuLutPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<country_core::resources::gpu_lut::GpuLutPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["country_core::resources::gpu_lut::GpuLutPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<country_core::resources::gpu_lut::GpuLutPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["country_core::resources::gpu_lut::GpuLutPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<country_core::resources::gpu_lut::GpuLutPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["country_core::resources::gpu_lut::GpuLutPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::finish<country_core::resources::gpu_lut::GpuLutPlugin>", "constructor": "bevy_app::plugin::Plugin::finish", "arguments": ["country_core::resources::gpu_lut::GpuLutPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::gpu_lut::GpuLutTracker
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::gpu_lut::GpuLutTracker>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::gpu_lut::GpuLutTracker"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::app::App::insert_resource<country_core::resources::gpu_lut::GpuLutTracker>", "constructor": "bevy_app::app::App::insert_resource", "arguments": ["country_core::resources::gpu_lut::GpuLutTracker"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::resources::gpu_lut::GpuLutTracker>", "constructor": "bevy_ecs::component::ComponentDescriptor::drop_ptr", "arguments": ["country_core::resources::gpu_lut::GpuLutTracker"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 10; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>,bevy_ecs::event::EventReader<country_core::systems::save::load_game::StartupRevealStarted>,bevy_ecs::event::EventReader<country_core::systems::save::load_game::StartupRevealFinished>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>", "bevy_ecs::event::EventReader<country_core::systems::save::load_game::StartupRevealStarted>", "bevy_ecs::event::EventReader<country_core::systems::save::load_game::StartupRevealFinished>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::save::load_game::StartupRevealStarted>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::save::load_game::StartupRevealFinished>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeights>", "bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeights>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::inner::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure_env$0,tuple$<ref_mut$<dyn$<country_core::resources::gpu_lut::GpuLutApi> > > >
// 信息 ID: 2dd22317c23c7673f327db87
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
// 信息 ID: 8fa8e5c9b8ac3db0bfce9c9e
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
// 信息 ID: aecc80dedd95dd36d0881e97
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::gpu_lut::impl$0::build; module=413, inline_record=87476; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::gpu_lut::impl$0::build; module=413, inline_record=87640; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::gpu_lut::GpuLutTracker::render_luts
// 信息 ID: 656fec41d0d0199a23eb56ec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2fd783 -> 0x57dae0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::gpu_lut::impl$0::build
// 信息 ID: 21068c0cf3f089ba06a531cf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::gpu_lut::impl$0::build::closure$0
// 信息 ID: 1a1b29c6ffc69ccfe4143a5d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::gpu_lut::impl$0::build", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::gpu_lut::impl$0::build::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure_env$0,tuple$<ref_mut$<dyn$<country_core::resources::gpu_lut::GpuLutApi> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure_env$0,tuple$<ref_mut$<dyn$<country_core::resources::gpu_lut::GpuLutApi> > > >; module=1360, inline_record=58952; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::gpu_lut::impl$0::build::closure$0::closure$0
// 信息 ID: cd828661230fa201c8f3ae6d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::gpu_lut::impl$0::build", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::gpu_lut::impl$0::build::closure$0::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure$0::closure_env$0,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputePass> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure$0::closure_env$0,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputePass> >; module=1360, inline_record=58260; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::gpu_lut::impl$0::build::closure$1
// 信息 ID: df1d2a6c9e1839ac8ddf4756
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::resources::gpu_lut::impl$0::build", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::gpu_lut::impl$0::build::closure_env$1 <- core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure_env$1,tuple$<ref_mut$<dyn$<country_core::resources::gpu_lut::GpuLutApi> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::resources::gpu_lut::impl$0::build::closure_env$1 <- core::ptr::drop_in_place<country_core::resources::gpu_lut::impl$0::build::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure_env$1,tuple$<ref_mut$<dyn$<country_core::resources::gpu_lut::GpuLutApi> > > >; module=1360, inline_record=57920; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::gpu_lut::impl$0::build::closure$1::closure$0
// 信息 ID: 4658520f428c4a9581ad870e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::resources::gpu_lut::impl$0::build", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::gpu_lut::impl$0::build::closure$1::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure$1::closure_env$0,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputePass> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure$1::closure_env$0,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputePass> >; module=1360, inline_record=59292; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::gpu_lut::impl$1::compute
// 信息 ID: ec33e7c5479de79b10e84621
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure_env$1,tuple$<ref_mut$<dyn$<country_core::resources::gpu_lut::GpuLutApi> > > >; module=1360, inline_record=57944; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure_env$0,tuple$<ref_mut$<dyn$<country_core::resources::gpu_lut::GpuLutApi> > > >; module=1360, inline_record=58976; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::gpu_lut::impl$0::build::closure$0; module=1360, inline_record=62908; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::gpu_lut::impl$0::build::closure$1; module=1360, inline_record=63084; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::gpu_lut::impl$3::render_luts::impl$0::compute_impl
// 信息 ID: fd3c259d39803faef6cb688f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
