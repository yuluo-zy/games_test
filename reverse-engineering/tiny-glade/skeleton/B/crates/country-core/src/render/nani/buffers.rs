// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/render/nani/buffers.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 035bbf5b4dca85d46668257585dc67e2c6c5c4c6f5f9ca78b4888a6b3aa0bfd0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: buffers.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 477, 493, 523, 1632, 3098
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
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::buffers::GpuInstance::new
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x66A8
// 首条位置: {"module":477,"inlinee":26280}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::buffers::GpuInstance::set_mesh_atlas_part
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6AF5
// 首条位置: {"module":523,"inlinee":27381}
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
// 源码行: 111
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6AF7
// 首条位置: {"module":1632,"inlinee":27383}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1632,"record_offset":6928,"parent_record_offset":6888,"ancestor_record_offset":6888,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1632,"record_offset":7096,"parent_record_offset":6888,"ancestor_record_offset":6888,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1632,"record_offset":7024,"parent_record_offset":6888,"ancestor_record_offset":6888,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1632,"record_offset":6956,"parent_record_offset":6928,"ancestor_record_offset":6888,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1632,"record_offset":7056,"parent_record_offset":7024,"ancestor_record_offset":6888,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1632,"record_offset":6984,"parent_record_offset":6956,"ancestor_record_offset":6888,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::buffers::NaniDrawListParams::list_draw_merge_window_subtotal_range
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 121
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6AF8
// 首条位置: {"module":1632,"inlinee":27384}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1632,"record_offset":7176,"parent_record_offset":7136,"ancestor_record_offset":7136,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1632,"record_offset":7348,"parent_record_offset":7136,"ancestor_record_offset":7136,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1632,"record_offset":7284,"parent_record_offset":7136,"ancestor_record_offset":7136,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1632,"record_offset":7208,"parent_record_offset":7176,"ancestor_record_offset":7136,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1632,"record_offset":7312,"parent_record_offset":7284,"ancestor_record_offset":7136,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1632,"record_offset":7240,"parent_record_offset":7208,"ancestor_record_offset":7136,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::buffers::nani_calculate_draw_list_params
// 记录形式: compiled_procedure; instances=1
// 源码行: 133
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":493,"record_offset":15996,"rva":9154256,"va":5377863376}
// 编译实例 RVAs: 0x8baed0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: buffers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2783688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377863376,"raw_hex":"e83886d5ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5377865187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=16
//     evidence={"assembly":"\tcallq\t14602980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377863376,"raw_hex":"e8e4d2de00","size":5,"soft_fail":false,"target_va":5392466848,"va":5377863863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":493,"record_offset":16176,"parent_record_offset":15996,"ancestor_record_offset":15996,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":493,"record_offset":16436,"parent_record_offset":15996,"ancestor_record_offset":15996,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::div_ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":493,"record_offset":17644,"parent_record_offset":15996,"ancestor_record_offset":15996,"inlinee":8810,"source_line":3197,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":493,"record_offset":18032,"parent_record_offset":15996,"ancestor_record_offset":15996,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorldSubsetKind::max_draws_per_list
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":493,"record_offset":16752,"parent_record_offset":15996,"ancestor_record_offset":15996,"inlinee":27333,"source_line":257,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorldSubsetKind::max_instances
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":493,"record_offset":16392,"parent_record_offset":15996,"ancestor_record_offset":15996,"inlinee":27332,"source_line":208,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::GpuWorldSubsetKind::use_fine_grained_sorting
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":493,"record_offset":17268,"parent_record_offset":15996,"ancestor_record_offset":15996,"inlinee":27334,"source_line":241,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] enum_iterator::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":493,"record_offset":16288,"parent_record_offset":15996,"ancestor_record_offset":15996,"inlinee":8819,"source_line":221,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":493,"record_offset":18220,"parent_record_offset":18196,"ancestor_record_offset":15996,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":493,"record_offset":18196,"parent_record_offset":18104,"ancestor_record_offset":15996,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":16568,"parent_record_offset":16528,"ancestor_record_offset":15996,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":18132,"parent_record_offset":18104,"ancestor_record_offset":15996,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":18104,"parent_record_offset":18080,"ancestor_record_offset":15996,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":16648,"parent_record_offset":16608,"ancestor_record_offset":15996,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":16608,"parent_record_offset":16568,"ancestor_record_offset":15996,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":18080,"parent_record_offset":18056,"ancestor_record_offset":15996,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":493,"record_offset":16528,"parent_record_offset":16436,"ancestor_record_offset":15996,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":493,"record_offset":18164,"parent_record_offset":18132,"ancestor_record_offset":15996,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":493,"record_offset":16328,"parent_record_offset":16288,"ancestor_record_offset":15996,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":493,"record_offset":18056,"parent_record_offset":18032,"ancestor_record_offset":15996,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":493,"record_offset":16704,"parent_record_offset":16436,"ancestor_record_offset":15996,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::GpuWorldSubsetKind::max_instances
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":493,"record_offset":16864,"parent_record_offset":16752,"ancestor_record_offset":15996,"inlinee":27332,"source_line":208,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::GpuWorldSubsetKind::use_fine_grained_sorting
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":493,"record_offset":16820,"parent_record_offset":16752,"ancestor_record_offset":15996,"inlinee":27334,"source_line":241,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::_::impl$0::next
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":493,"record_offset":16360,"parent_record_offset":16288,"ancestor_record_offset":15996,"inlinee":27330,"source_line":43,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::buffers::impl$9::default_value
// 记录形式: compiled_procedure; instances=1
// 源码行: 258
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":493,"record_offset":19036,"rva":9156176,"va":5377865296}
// 编译实例 RVAs: 0x8bb650
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: buffers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33980499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377865296,"raw_hex":"e853800602","size":5,"soft_fail":false,"target_va":5411847168,"va":5377866664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: buffers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33981872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377865296,"raw_hex":"e8b0850602","size":5,"soft_fail":false,"target_va":5411847200,"va":5377865323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] enum_iterator::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"enum-iterator","version":"1.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/enum-iterator-1.2.0/src/lib.rs
//     evidence={"module":493,"record_offset":19180,"parent_record_offset":19036,"ancestor_record_offset":19036,"inlinee":8819,"source_line":221,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":493,"record_offset":19304,"parent_record_offset":19180,"ancestor_record_offset":19036,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::_::impl$0::next
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":493,"record_offset":19412,"parent_record_offset":19180,"ancestor_record_offset":19036,"inlinee":27330,"source_line":43,"archive":"_pdb/B/modules/00493.jsonl.gz"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::shadowed_buffer::ShadowedBuffer<country_core::render::nani::buffers::GpuInstance>", "constructor": "country_core::render::shadowed_buffer::ShadowedBuffer", "arguments": ["country_core::render::nani::buffers::GpuInstance"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::render::shadowed_buffer::ShadowedBuffer<country_core::render::nani::buffers::GpuInstance> >", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::render::shadowed_buffer::ShadowedBuffer<country_core::render::nani::buffers::GpuInstance>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer::new<country_core::render::nani::buffers::GpuInstance>", "constructor": "country_core::render::ssbo::TypedStorageBuffer::new", "arguments": ["country_core::render::nani::buffers::GpuInstance"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::render::nani::buffers::GpuInstance>", "constructor": "country_core::render::shadowed_buffer::ShadowedBuffer::new", "arguments": ["country_core::render::nani::buffers::GpuInstance"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::buffers::GpuInstance>", "constructor": "country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush", "arguments": ["country_core::render::nani::buffers::GpuInstance"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::buffers::NaniDrawListLengthSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::write_access_buffer<country_core::render::nani::buffers::NaniDrawListLengthSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::write_access_buffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListLengthSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::render::nani::buffers::NaniDrawListLengthSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::render::nani::buffers::NaniDrawListLengthSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::render::nani::buffers::NaniDrawListLengthSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListLengthSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::render::nani::buffers::NaniDrawListLengthSsbo>", "constructor": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListLengthSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniDrawListLengthSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListLengthSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniDrawListLengthSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniDrawListLengthSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any<country_core::render::nani::buffers::NaniDrawListLengthSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any", "arguments": ["country_core::render::nani::buffers::NaniDrawListLengthSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut<country_core::render::nani::buffers::NaniDrawListLengthSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut", "arguments": ["country_core::render::nani::buffers::NaniDrawListLengthSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$0::new_storage_buffer<country_core::render::nani::buffers::NaniDrawListLengthSsbo>", "constructor": "country_core::render::ssbo::impl$0::new_storage_buffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListLengthSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::buffers::NaniDrawListOffsetDefaults
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>", "constructor": "country_core::render::pass_builder::pass_resource_access::read_access_buffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListOffsetDefaults"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::render::nani::buffers::NaniDrawListOffsetDefaults"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListOffsetDefaults"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>", "constructor": "country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListOffsetDefaults"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::InstanceBuffer<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>", "constructor": "country_core::render::ssbo::InstanceBuffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListOffsetDefaults"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::InstanceBuffer<country_core::render::nani::buffers::NaniDrawListOffsetDefaults> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::InstanceBuffer<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::as_any<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::as_any", "arguments": ["country_core::render::nani::buffers::NaniDrawListOffsetDefaults"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$2::as_any_mut<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>", "constructor": "country_core::asset_libraries::ssbo_library::impl$2::as_any_mut", "arguments": ["country_core::render::nani::buffers::NaniDrawListOffsetDefaults"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer::new<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>", "constructor": "country_core::render::ssbo::TypedStorageBuffer::new", "arguments": ["country_core::render::nani::buffers::NaniDrawListOffsetDefaults"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$7::new_storage_buffer<country_core::render::nani::buffers::NaniDrawListOffsetDefaults>", "constructor": "country_core::render::ssbo::impl$7::new_storage_buffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListOffsetDefaults"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::buffers::NaniDrawListParams
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::render::nani::buffers::NaniDrawListParams>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::render::nani::buffers::NaniDrawListParams"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::buffers::NaniDrawListSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::write_access_buffer<country_core::render::nani::buffers::NaniDrawListSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::write_access_buffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::write_access_buffer_concurrent<country_core::render::nani::buffers::NaniDrawListSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::write_access_buffer_concurrent", "arguments": ["country_core::render::nani::buffers::NaniDrawListSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new_with_data<country_core::render::nani::buffers::NaniDrawListSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new_with_data", "arguments": ["country_core::render::nani::buffers::NaniDrawListSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::render::nani::buffers::NaniDrawListSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::render::nani::buffers::NaniDrawListSsbo>", "constructor": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer_concurrent<country_core::render::nani::buffers::NaniDrawListSsbo>", "constructor": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer_concurrent", "arguments": ["country_core::render::nani::buffers::NaniDrawListSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniDrawListSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniDrawListSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniDrawListSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any<country_core::render::nani::buffers::NaniDrawListSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any", "arguments": ["country_core::render::nani::buffers::NaniDrawListSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut<country_core::render::nani::buffers::NaniDrawListSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut", "arguments": ["country_core::render::nani::buffers::NaniDrawListSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$0::new_storage_buffer<country_core::render::nani::buffers::NaniDrawListSsbo>", "constructor": "country_core::render::ssbo::impl$0::new_storage_buffer", "arguments": ["country_core::render::nani::buffers::NaniDrawListSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 12
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::read_access_buffer", "arguments": ["country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::write_access_buffer<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::write_access_buffer", "arguments": ["country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::write_access_buffer_concurrent<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::write_access_buffer_concurrent", "arguments": ["country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer", "arguments": ["country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>", "constructor": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer", "arguments": ["country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer_concurrent<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>", "constructor": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer_concurrent", "arguments": ["country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>", "constructor": "country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer", "arguments": ["country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any", "arguments": ["country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut", "arguments": ["country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$0::new_storage_buffer<country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>", "constructor": "country_core::render::ssbo::impl$0::new_storage_buffer", "arguments": ["country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::write_access_buffer<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::write_access_buffer", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::write_access_buffer_concurrent<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::write_access_buffer_concurrent", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo>", "constructor": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer_concurrent<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo>", "constructor": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer_concurrent", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$0::new_storage_buffer<country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo>", "constructor": "country_core::render::ssbo::impl$0::new_storage_buffer", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::buffers::NaniInstanceIndirectionSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 11
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::read_access_buffer", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::write_access_buffer_concurrent<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::write_access_buffer_concurrent", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new_with_data<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new_with_data", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer_concurrent<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>", "constructor": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer_concurrent", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>", "constructor": "country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$0::new_storage_buffer<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>", "constructor": "country_core::render::ssbo::impl$0::new_storage_buffer", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>", "constructor": "country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer", "arguments": ["country_core::render::nani::buffers::NaniInstanceIndirectionSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::buffers::NaniListDrawCmdOffsets
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new_with_data<country_core::render::nani::buffers::NaniListDrawCmdOffsets>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new_with_data", "arguments": ["country_core::render::nani::buffers::NaniListDrawCmdOffsets"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniListDrawCmdOffsets>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::render::nani::buffers::NaniListDrawCmdOffsets"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniListDrawCmdOffsets> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniListDrawCmdOffsets>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any<country_core::render::nani::buffers::NaniListDrawCmdOffsets>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any", "arguments": ["country_core::render::nani::buffers::NaniListDrawCmdOffsets"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut<country_core::render::nani::buffers::NaniListDrawCmdOffsets>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut", "arguments": ["country_core::render::nani::buffers::NaniListDrawCmdOffsets"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$0::new_storage_buffer<country_core::render::nani::buffers::NaniListDrawCmdOffsets>", "constructor": "country_core::render::ssbo::impl$0::new_storage_buffer", "arguments": ["country_core::render::nani::buffers::NaniListDrawCmdOffsets"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::read_access_buffer", "arguments": ["country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::write_access_buffer_concurrent<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::write_access_buffer_concurrent", "arguments": ["country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new_with_data<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new_with_data", "arguments": ["country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer", "arguments": ["country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer_concurrent<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>", "constructor": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer_concurrent", "arguments": ["country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>", "constructor": "country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer", "arguments": ["country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any", "arguments": ["country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut", "arguments": ["country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$0::new_storage_buffer<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo>", "constructor": "country_core::render::ssbo::impl$0::new_storage_buffer", "arguments": ["country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::read_access_buffer", "arguments": ["country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::pass_resource_access::write_access_buffer_concurrent<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>", "constructor": "country_core::render::pass_builder::pass_resource_access::write_access_buffer_concurrent", "arguments": ["country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new_with_data<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new_with_data", "arguments": ["country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer", "arguments": ["country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer_concurrent<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>", "constructor": "country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer_concurrent", "arguments": ["country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>", "constructor": "country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer", "arguments": ["country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer", "arguments": ["country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::ssbo::TypedStorageBuffer<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any", "arguments": ["country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>", "constructor": "country_core::asset_libraries::ssbo_library::impl$0::as_any_mut", "arguments": ["country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::impl$0::new_storage_buffer<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo>", "constructor": "country_core::render::ssbo::impl$0::new_storage_buffer", "arguments": ["country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: buffers.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-camera/B-decompile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo", "status": "evidence_association; original field ownership unknown"}, {"kind": "exact_PDB_named_entity_mention", "name": "country_core::render::nani::buffers::NaniDrawListOffsetDefaults", "status": "evidence_association; original field ownership unknown"}, {"kind": "exact_PDB_named_entity_mention", "name": "country_core::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo", "status": "evidence_association; original field ownership unknown"}, {"kind": "exact_PDB_named_entity_mention", "name": "country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo", "status": "evidence_association; original field ownership unknown"}, {"kind": "exact_PDB_named_entity_mention", "name": "country_core::render::nani::buffers::NaniInstanceIndirectionSsbo", "status": "evidence_association; original field ownership unknown"}, {"kind": "exact_PDB_named_entity_mention", "name": "country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo", "status": "evidence_association; original field ownership unknown"}, {"kind": "exact_PDB_named_entity_mention", "name": "country_core::render::nani::buffers::NaniDrawListSsbo", "status": "evidence_association; original field ownership unknown"}, {"kind": "exact_PDB_named_entity_mention", "name": "country_core::render::nani::buffers::NaniDrawListLengthSsbo", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: buffers.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::render::nani::buffers::GpuInstance::new
// 信息 ID: 944d287cf285b2857bcd64b6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::prefab::create_prefab_gpu_instances; module=477, inline_record=18928; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::systems::nani_maintain_instances; module=3098, inline_record=14632; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::buffers::GpuInstance::set_mesh_atlas_part
// 信息 ID: e772b73436e6a8939807f171
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::update_instance_mesh; module=523, inline_record=9360; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::buffers::NaniDrawListParams::list_draw_merge_window_range
// 信息 ID: 814e3958f7a84c489c556d51
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::group_draw_cmd_prefix_sum::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> >; module=1632, inline_record=6888; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::buffers::NaniDrawListParams::list_draw_merge_window_subtotal_range
// 信息 ID: 2c1de704f74b8277f129142d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::group_draw_cmd_prefix_sum::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> >; module=1632, inline_record=7136; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::group_draw_cmd_prefix_sum::closure_env$1<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> >; module=1632, inline_record=16356; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::buffers::impl$9::default_value
// 信息 ID: b4301f0e85bd298b29bcf28d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::ssbo::TypedStorageBuffer::new<country_core::render::nani::buffers::NaniDrawListOffsetDefaults> @ 0x161550f -> 0x8bb650 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$7::new_storage_buffer<country_core::render::nani::buffers::NaniDrawListOffsetDefaults> @ 0x162088c -> 0x8bb650 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::buffers::nani_calculate_draw_list_params
// 信息 ID: 7ec3e2e8d633d28c0d1d61ff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::init_populate_ssbo_library @ 0x90a2f0 -> 0x8baed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::gpu_world::GpuWorld::new @ 0x991805 -> 0x8baed0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
