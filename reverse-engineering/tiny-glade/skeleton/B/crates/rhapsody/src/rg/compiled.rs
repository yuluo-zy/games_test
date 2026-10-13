// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/rg/compiled.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 1dbc5a24ab8fb8eeb18d47c350aabe85e64aea1c4c6c87945b5d10dee8d70dc0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: compiled.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 43, 104, 105, 175, 184, 698
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_tasks; // namespace candidate
//   package identities: [{"name":"bevy_tasks","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use gpu_profiler; // namespace candidate
//   package identities: [{"name":"gpu-profiler","source":"local_or_path","version":"0.1.0"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use vk_sync_fork; // namespace candidate
//   package identities: [{"name":"vk-sync-fork","source":"local_or_path","version":"0.4.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::compiled::CompiledRenderGraph::add_pre_pass_barriers<country_core::render::pass_builder::pass_resources::PassResources>
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":105,"record_offset":19004,"rva":2899200,"va":5371608320}
// 编译实例 RVAs: 0x2c3d00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd66a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9513175","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371608320,"raw_hex":"e8d7289100","size":5,"soft_fail":false,"target_va":5381121696,"va":5371608516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6810
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9513108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371608320,"raw_hex":"e894289100","size":5,"soft_fail":false,"target_va":5381122064,"va":5371608951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a21b8
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39706833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371608320,"raw_hex":"e8d1e05d02","size":5,"soft_fail":false,"target_va":5411316152,"va":5371609314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t39932552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371608320,"raw_hex":"e888526102","size":5,"soft_fail":false,"target_va":5411541776,"va":5371609219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t39872351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371608320,"raw_hex":"e85f676002","size":5,"soft_fail":false,"target_va":5411481622,"va":5371609266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39932440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371608320,"raw_hex":"e818526102","size":5,"soft_fail":false,"target_va":5411542064,"va":5371609619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t39932696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371608320,"raw_hex":"e818536102","size":5,"soft_fail":false,"target_va":5411542296,"va":5371609595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39933524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371608320,"raw_hex":"e854566102","size":5,"soft_fail":false,"target_va":5411543104,"va":5371609575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39933560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371608320,"raw_hex":"e878566102","size":5,"soft_fail":false,"target_va":5411543120,"va":5371609555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::barrier::BufferBarrier::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9512609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371608320,"raw_hex":"e8a1269100","size":5,"soft_fail":false,"target_va":5381121552,"va":5371608938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::barrier::ImageBarrier::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9512996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371608320,"raw_hex":"e824289100","size":5,"soft_fail":false,"target_va":5381121504,"va":5371608503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::barrier::image_aspect_mask_from_format
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9513102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371608320,"raw_hex":"e88e289100","size":5,"soft_fail":false,"target_va":5381121584,"va":5371608477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::compiled::pass_access_range_indices
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9591125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371608320,"raw_hex":"e855599200","size":5,"soft_fail":false,"target_va":5381199504,"va":5371608374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::recorder::PassResourceAccessType::can_be_concurrent_with
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9605107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371608320,"raw_hex":"e8f38f9200","size":5,"soft_fail":false,"target_va":5381213808,"va":5371608696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource::RuntimeResource::borrow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9514592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371608320,"raw_hex":"e8602e9100","size":5,"soft_fail":false,"target_va":5381123200,"va":5371608603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource_registry::ResourceRegistry::resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9583286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371608320,"raw_hex":"e8b63a9200","size":5,"soft_fail":false,"target_va":5381191856,"va":5371608565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":19240,"parent_record_offset":19004,"ancestor_record_offset":19004,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":105,"record_offset":19780,"parent_record_offset":19004,"ancestor_record_offset":19004,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":105,"record_offset":19476,"parent_record_offset":19004,"ancestor_record_offset":19004,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":105,"record_offset":19652,"parent_record_offset":19004,"ancestor_record_offset":19004,"inlinee":11960,"source_line":315,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] vk_sync_fork::impl$5::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"vk-sync-fork","version":"0.4.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/vk-sync-rs/src/lib.rs
//     evidence={"module":105,"record_offset":19816,"parent_record_offset":19004,"ancestor_record_offset":19004,"inlinee":11962,"source_line":21,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":105,"record_offset":19436,"parent_record_offset":19400,"ancestor_record_offset":19004,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":105,"record_offset":19520,"parent_record_offset":19476,"ancestor_record_offset":19004,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":105,"record_offset":19268,"parent_record_offset":19240,"ancestor_record_offset":19004,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":105,"record_offset":19296,"parent_record_offset":19268,"ancestor_record_offset":19004,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":105,"record_offset":19400,"parent_record_offset":19368,"ancestor_record_offset":19004,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":105,"record_offset":19692,"parent_record_offset":19652,"ancestor_record_offset":19004,"inlinee":11948,"source_line":14,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":105,"record_offset":19748,"parent_record_offset":19652,"ancestor_record_offset":19004,"inlinee":11952,"source_line":23,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":105,"record_offset":19720,"parent_record_offset":19652,"ancestor_record_offset":19004,"inlinee":11950,"source_line":32,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::compiled::BatchCompiler::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 57
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":698,"record_offset":256,"rva":12490208,"va":5381199328}
// 编译实例 RVAs: 0xbe95e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::compiled::BatchCompiler::flush_batch
// 记录形式: compiled_procedure; instances=1
// 源码行: 67
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":698,"record_offset":376,"rva":12490240,"va":5381199360}
// 编译实例 RVAs: 0xbe9600
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6842526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381199360,"raw_hex":"e8629797ff","size":5,"soft_fail":false,"target_va":5374356912,"va":5381199433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":698,"record_offset":632,"parent_record_offset":376,"ancestor_record_offset":376,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00698.jsonl.gz"}
//   [pdb_inline_direct] core::mem::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":698,"record_offset":516,"parent_record_offset":376,"ancestor_record_offset":376,"inlinee":5313,"source_line":793,"archive":"_pdb/B/modules/00698.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":698,"record_offset":700,"parent_record_offset":668,"ancestor_record_offset":376,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00698.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":698,"record_offset":748,"parent_record_offset":724,"ancestor_record_offset":376,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00698.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":698,"record_offset":724,"parent_record_offset":700,"ancestor_record_offset":376,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00698.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":698,"record_offset":668,"parent_record_offset":632,"ancestor_record_offset":376,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00698.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":698,"record_offset":540,"parent_record_offset":516,"ancestor_record_offset":376,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00698.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":698,"record_offset":568,"parent_record_offset":540,"ancestor_record_offset":376,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00698.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":698,"record_offset":596,"parent_record_offset":540,"ancestor_record_offset":376,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00698.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::compiled::BatchCompiler::compile
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 75
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x224F
// 首条位置: {"module":43,"inlinee":8783}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":43,"record_offset":24020,"parent_record_offset":23980,"ancestor_record_offset":23980,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":43,"record_offset":24176,"parent_record_offset":23980,"ancestor_record_offset":23980,"inlinee":7317,"source_line":302,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":43,"record_offset":24080,"parent_record_offset":24048,"ancestor_record_offset":23980,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":43,"record_offset":24112,"parent_record_offset":24048,"ancestor_record_offset":23980,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":43,"record_offset":24048,"parent_record_offset":24020,"ancestor_record_offset":23980,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":43,"record_offset":24136,"parent_record_offset":24112,"ancestor_record_offset":23980,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::compiled::CompiledRenderGraph::new<country_core::render::pass_builder::pass_resources::PassResources>
// 记录形式: compiled_procedure; instances=1
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":105,"record_offset":20528,"rva":2900608,"va":5371609728}
// 编译实例 RVAs: 0x2c4280
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402bf880
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371609728,"raw_hex":"e887b5ffff","size":5,"soft_fail":false,"target_va":5371590784,"va":5371609844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405da820
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3236840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371609728,"raw_hex":"e8e8633100","size":5,"soft_fail":false,"target_va":5374847008,"va":5371610163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ef210
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3321295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371609728,"raw_hex":"e8cfad3200","size":5,"soft_fail":false,"target_va":5374931472,"va":5371610172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fd1d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3378596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371609728,"raw_hex":"e8a48d3300","size":5,"soft_fail":false,"target_va":5374988752,"va":5371610151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3471293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371609728,"raw_hex":"e8bdf73400","size":5,"soft_fail":false,"target_va":5375081504,"va":5371610206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6650
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9511683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371609728,"raw_hex":"e803239100","size":5,"soft_fail":false,"target_va":5381121616,"va":5371609928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40237297","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371609728,"raw_hex":"e8f1f86502","size":5,"soft_fail":false,"target_va":5411847168,"va":5371609866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":105,"record_offset":20820,"parent_record_offset":20528,"ancestor_record_offset":20528,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":105,"record_offset":20904,"parent_record_offset":20528,"ancestor_record_offset":20528,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":105,"record_offset":20736,"parent_record_offset":20528,"ancestor_record_offset":20528,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":105,"record_offset":21092,"parent_record_offset":21068,"ancestor_record_offset":20528,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":105,"record_offset":21068,"parent_record_offset":20976,"ancestor_record_offset":20528,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":105,"record_offset":21004,"parent_record_offset":20976,"ancestor_record_offset":20528,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":105,"record_offset":20976,"parent_record_offset":20952,"ancestor_record_offset":20528,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":105,"record_offset":20952,"parent_record_offset":20928,"ancestor_record_offset":20528,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":20844,"parent_record_offset":20820,"ancestor_record_offset":20528,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":105,"record_offset":20868,"parent_record_offset":20844,"ancestor_record_offset":20528,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":105,"record_offset":21036,"parent_record_offset":21004,"ancestor_record_offset":20528,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":105,"record_offset":20928,"parent_record_offset":20904,"ancestor_record_offset":20528,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":105,"record_offset":20784,"parent_record_offset":20760,"ancestor_record_offset":20528,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":105,"record_offset":20760,"parent_record_offset":20736,"ancestor_record_offset":20528,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::compiled::impl$1::new::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 107
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2E85
// 首条位置: {"module":104,"inlinee":11909}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::resource_info::ResourceAccessIndex::try_index
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":104,"record_offset":161192,"parent_record_offset":161168,"ancestor_record_offset":161168,"inlinee":11911,"source_line":219,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":104,"record_offset":161240,"parent_record_offset":161216,"ancestor_record_offset":161168,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource_info::ResourceAccessIndex::valid
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":104,"record_offset":161216,"parent_record_offset":161192,"ancestor_record_offset":161168,"inlinee":11912,"source_line":228,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource_info::impl$17::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":104,"record_offset":161264,"parent_record_offset":161240,"ancestor_record_offset":161168,"inlinee":11914,"source_line":252,"archive":"_pdb/B/modules/00104.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::compiled::CompiledRenderGraph::compute_batches
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 123
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x224D
// 首条位置: {"module":43,"inlinee":8781}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":43,"record_offset":24208,"parent_record_offset":23940,"ancestor_record_offset":23940,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::compiled::BatchCompiler::compile
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/compiled.rs
//     evidence={"module":43,"record_offset":23980,"parent_record_offset":23940,"ancestor_record_offset":23940,"inlinee":8783,"source_line":75,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":43,"record_offset":24420,"parent_record_offset":24396,"ancestor_record_offset":23940,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":43,"record_offset":24396,"parent_record_offset":24304,"ancestor_record_offset":23940,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":43,"record_offset":24332,"parent_record_offset":24304,"ancestor_record_offset":23940,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":43,"record_offset":24304,"parent_record_offset":24280,"ancestor_record_offset":23940,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":43,"record_offset":24280,"parent_record_offset":24256,"ancestor_record_offset":23940,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":43,"record_offset":24080,"parent_record_offset":24048,"ancestor_record_offset":23940,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":43,"record_offset":24112,"parent_record_offset":24048,"ancestor_record_offset":23940,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":43,"record_offset":24048,"parent_record_offset":24020,"ancestor_record_offset":23940,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":43,"record_offset":24020,"parent_record_offset":23980,"ancestor_record_offset":23940,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":43,"record_offset":24136,"parent_record_offset":24112,"ancestor_record_offset":23940,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":43,"record_offset":24364,"parent_record_offset":24332,"ancestor_record_offset":23940,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":43,"record_offset":24232,"parent_record_offset":24208,"ancestor_record_offset":23940,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":43,"record_offset":24176,"parent_record_offset":23980,"ancestor_record_offset":23940,"inlinee":7317,"source_line":302,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources>
// 记录形式: compiled_procedure; instances=1
// 源码行: 130
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":105,"record_offset":3068,"rva":2891072,"va":5371600192}
// 编译实例 RVAs: 0x2c1d40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1401058a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1821847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e86933e4ff","size":5,"soft_fail":false,"target_va":5369780384,"va":5371602226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b54b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-53299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e8cd2fffff","size":5,"soft_fail":false,"target_va":5371548848,"va":5371602142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ca150
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e88e7d0000","size":5,"soft_fail":false,"target_va":5371634000,"va":5371601853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d12d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3206278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e886ec3000","size":5,"soft_fail":false,"target_va":5374808784,"va":5371602501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d4610
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3216427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e82b143100","size":5,"soft_fail":false,"target_va":5374821904,"va":5371605472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140609cb0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3435355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e85b6b3400","size":5,"soft_fail":false,"target_va":5375040688,"va":5371605328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3480422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e8661b3500","size":5,"soft_fail":false,"target_va":5375081488,"va":5371601061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=9
//     target_aliases=1 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3479581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e81d183500","size":5,"soft_fail":false,"target_va":5375081504,"va":5371601918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3914433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e8c1ba3b00","size":5,"soft_fail":false,"target_va":5375516032,"va":5371601594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd66a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9520857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e8d9469100","size":5,"soft_fail":false,"target_va":5381121696,"va":5371600834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6810
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9521375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e8df489100","size":5,"soft_fail":false,"target_va":5381122064,"va":5371600684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6950
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9521481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e849499100","size":5,"soft_fail":false,"target_va":5381122384,"va":5371600898,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6980
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9521563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e89b499100","size":5,"soft_fail":false,"target_va":5381122432,"va":5371600864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bee030
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9617457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e831c09200","size":5,"soft_fail":false,"target_va":5381218352,"va":5371600890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bf09b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9626321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e8d1e29200","size":5,"soft_fail":false,"target_va":5381228976,"va":5371602650,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bf0b40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9626043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e8bbe19200","size":5,"soft_fail":false,"target_va":5381229376,"va":5371603328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40244688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e8d0156602","size":5,"soft_fail":false,"target_va":5411847168,"va":5371602475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39936215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e8d7606102","size":5,"soft_fail":false,"target_va":5411540992,"va":5371604772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2753309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e81d032a00","size":5,"soft_fail":false,"target_va":5374354992,"va":5371601678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39713842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e832fc5d02","size":5,"soft_fail":false,"target_va":5411318240,"va":5371604393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39936203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e8cb606102","size":5,"soft_fail":false,"target_va":5411540963,"va":5371604755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::in_place_collect::from_iter_in_place
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2752233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e8e9fe2900","size":5,"soft_fail":false,"target_va":5374354560,"va":5371602322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_tasks::usages::ComputeTaskPool::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_tasks","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11022990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e88e32a800","size":5,"soft_fail":false,"target_va":5382625072,"va":5371602077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t39937175","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e897646102","size":5,"soft_fail":false,"target_va":5411541776,"va":5371604596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t39877673","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e8297c6002","size":5,"soft_fail":false,"target_va":5411481622,"va":5371603944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t39938197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e895686102","size":5,"soft_fail":false,"target_va":5411542064,"va":5371603862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t39937769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e8e9666102","size":5,"soft_fail":false,"target_va":5411542296,"va":5371604522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39938066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e812686102","size":5,"soft_fail":false,"target_va":5411541904,"va":5371603833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t39938744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e8b86a6102","size":5,"soft_fail":false,"target_va":5411543104,"va":5371604355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t39938780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e8dc6a6102","size":5,"soft_fail":false,"target_va":5411543120,"va":5371604335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] gpu_profiler::backend::ash::ActiveProfiler::create_marker
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"gpu-profiler","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9760785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e811f09400","size":5,"soft_fail":false,"target_va":5381361904,"va":5371601114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::barrier::BufferBarrier::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9520882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e8f2469100","size":5,"soft_fail":false,"target_va":5381121552,"va":5371600665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::barrier::ImageBarrier::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9520684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e82c469100","size":5,"soft_fail":false,"target_va":5381121504,"va":5371600815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::barrier::image_aspect_mask_from_format
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9520789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e895469100","size":5,"soft_fail":false,"target_va":5381121584,"va":5371600790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::command_buffer::CommandBuffer::end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9612603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e83bad9200","size":5,"soft_fail":false,"target_va":5381215184,"va":5371602576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::command_buffer::CommandBufferSeqTracker::track
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9613516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e8ccb09200","size":5,"soft_fail":false,"target_va":5381214160,"va":5371600639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::compiled::impl$1::record_batch::closure$1<country_core::render::pass_builder::pass_resources::PassResources>
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e8150f0000","size":5,"soft_fail":false,"target_va":5371605568,"va":5371601702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::executor::PassPgoStats::duration_ns
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9600786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e8127f9200","size":5,"soft_fail":false,"target_va":5381202400,"va":5371601609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource::RuntimeResource::borrow
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9522648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e8d84d9100","size":5,"soft_fail":false,"target_va":5381123200,"va":5371600547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource_registry::ResourceRegistry::resource
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9591326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e81e5a9200","size":5,"soft_fail":false,"target_va":5381191856,"va":5371600525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39932299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371600192,"raw_hex":"e88b516102","size":5,"soft_fail":false,"target_va":5411537104,"va":5371604800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":105,"record_offset":7928,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":105,"record_offset":8080,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":5972,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":6580,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":6236,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":4736,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":6000,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":5305,"source_line":3369,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":8392,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":8848,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] bevy_tasks::slice::ParallelSliceMut::par_chunk_map_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_tasks","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_tasks-0.16.0/src/slice.rs
//     evidence={"module":105,"record_offset":8164,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":11923,"source_line":140,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":105,"record_offset":6484,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":11953,"source_line":1063,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":105,"record_offset":6884,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":105,"record_offset":5020,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":105,"record_offset":4708,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":105,"record_offset":3408,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":105,"record_offset":6204,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4666,"source_line":998,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":105,"record_offset":8588,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":105,"record_offset":6772,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4614,"source_line":134,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":105,"record_offset":4936,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":105,"record_offset":3380,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":7317,"source_line":302,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::split_at_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":105,"record_offset":6972,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":5492,"source_line":1950,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":105,"record_offset":9712,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$2::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":105,"record_offset":9600,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":11955,"source_line":35,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::issue_and_reset_pipeline_barrier
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/barrier.rs
//     evidence={"module":105,"record_offset":4672,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":8711,"source_line":419,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::compiled::CompiledRenderGraph::gather_batch_preamble_barriers
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/compiled.rs
//     evidence={"module":105,"record_offset":3448,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":11944,"source_line":352,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::compiled::CompiledRenderGraph::record_postamble
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/compiled.rs
//     evidence={"module":105,"record_offset":9928,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":11956,"source_line":394,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::executor::PassPgoStats::extend
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/executor.rs
//     evidence={"module":105,"record_offset":7064,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":8793,"source_line":162,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":105,"record_offset":5384,"parent_record_offset":5360,"ancestor_record_offset":3068,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":105,"record_offset":5408,"parent_record_offset":5384,"ancestor_record_offset":3068,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":105,"record_offset":5908,"parent_record_offset":5884,"ancestor_record_offset":3068,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":105,"record_offset":5360,"parent_record_offset":5324,"ancestor_record_offset":3068,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":105,"record_offset":5884,"parent_record_offset":5860,"ancestor_record_offset":3068,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":105,"record_offset":3708,"parent_record_offset":3684,"ancestor_record_offset":3068,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":105,"record_offset":9060,"parent_record_offset":9032,"ancestor_record_offset":3068,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":105,"record_offset":5256,"parent_record_offset":5224,"ancestor_record_offset":3068,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":105,"record_offset":9144,"parent_record_offset":9120,"ancestor_record_offset":3068,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":105,"record_offset":7464,"parent_record_offset":7436,"ancestor_record_offset":3068,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":105,"record_offset":5860,"parent_record_offset":5836,"ancestor_record_offset":3068,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":105,"record_offset":9120,"parent_record_offset":9088,"ancestor_record_offset":3068,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":105,"record_offset":3756,"parent_record_offset":3732,"ancestor_record_offset":3068,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":105,"record_offset":3732,"parent_record_offset":3708,"ancestor_record_offset":3068,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":105,"record_offset":9088,"parent_record_offset":9060,"ancestor_record_offset":3068,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":105,"record_offset":5324,"parent_record_offset":5288,"ancestor_record_offset":3068,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":105,"record_offset":5288,"parent_record_offset":5256,"ancestor_record_offset":3068,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":105,"record_offset":5836,"parent_record_offset":5812,"ancestor_record_offset":3068,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":105,"record_offset":8104,"parent_record_offset":8080,"ancestor_record_offset":3068,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":105,"record_offset":8344,"parent_record_offset":8320,"ancestor_record_offset":3068,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":8988,"parent_record_offset":8944,"ancestor_record_offset":3068,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":6620,"parent_record_offset":6580,"ancestor_record_offset":3068,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":3684,"parent_record_offset":3624,"ancestor_record_offset":3068,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":10140,"parent_record_offset":10116,"ancestor_record_offset":3068,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":3624,"parent_record_offset":3448,"ancestor_record_offset":3068,"inlinee":5295,"source_line":2606,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":5488,"parent_record_offset":5464,"ancestor_record_offset":3068,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":8128,"parent_record_offset":8104,"ancestor_record_offset":3068,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":3596,"parent_record_offset":3448,"ancestor_record_offset":3068,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":9328,"parent_record_offset":8988,"ancestor_record_offset":3068,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":9032,"parent_record_offset":8988,"ancestor_record_offset":3068,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":3656,"parent_record_offset":3624,"ancestor_record_offset":3068,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":5192,"parent_record_offset":5140,"ancestor_record_offset":3068,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":5224,"parent_record_offset":5192,"ancestor_record_offset":3068,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":105,"record_offset":3884,"parent_record_offset":3448,"ancestor_record_offset":3068,"inlinee":8249,"source_line":156,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":105,"record_offset":4084,"parent_record_offset":4060,"ancestor_record_offset":3068,"inlinee":8251,"source_line":157,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":4148,"parent_record_offset":3448,"ancestor_record_offset":3068,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":5060,"parent_record_offset":5020,"ancestor_record_offset":3068,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":8884,"parent_record_offset":8848,"ancestor_record_offset":3068,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":10092,"parent_record_offset":9928,"ancestor_record_offset":3068,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":5672,"parent_record_offset":5648,"ancestor_record_offset":3068,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":10116,"parent_record_offset":10092,"ancestor_record_offset":3068,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::in_place_collect::from_iter_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\in_place_collect.rs
//     evidence={"module":105,"record_offset":8548,"parent_record_offset":8524,"ancestor_record_offset":3068,"inlinee":11705,"source_line":251,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::in_place_collect::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\in_place_collect.rs
//     evidence={"module":105,"record_offset":8524,"parent_record_offset":8472,"ancestor_record_offset":3068,"inlinee":11928,"source_line":233,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::forget_remaining_elements
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":105,"record_offset":9360,"parent_record_offset":8944,"ancestor_record_offset":3068,"inlinee":10155,"source_line":157,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":105,"record_offset":7316,"parent_record_offset":7292,"ancestor_record_offset":3068,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":105,"record_offset":7364,"parent_record_offset":7340,"ancestor_record_offset":3068,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":105,"record_offset":7108,"parent_record_offset":7064,"ancestor_record_offset":3068,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":105,"record_offset":5728,"parent_record_offset":5672,"ancestor_record_offset":3068,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":105,"record_offset":5464,"parent_record_offset":5140,"ancestor_record_offset":3068,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$2::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":105,"record_offset":8944,"parent_record_offset":8848,"ancestor_record_offset":3068,"inlinee":8579,"source_line":35,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":105,"record_offset":5100,"parent_record_offset":5060,"ancestor_record_offset":3068,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":105,"record_offset":5140,"parent_record_offset":5100,"ancestor_record_offset":3068,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_tasks::task_pool::TaskPool::scope
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_tasks","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_tasks-0.16.0/src/task_pool.rs
//     evidence={"module":105,"record_offset":8200,"parent_record_offset":8164,"ancestor_record_offset":3068,"inlinee":11683,"source_line":310,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] bevy_tasks::task_pool::impl$2::scope::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_tasks","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_tasks-0.16.0/src/task_pool.rs
//     evidence={"module":105,"record_offset":8320,"parent_record_offset":8264,"ancestor_record_offset":3068,"inlinee":7369,"source_line":315,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":105,"record_offset":7264,"parent_record_offset":7064,"ancestor_record_offset":3068,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$66::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":105,"record_offset":6520,"parent_record_offset":6484,"ancestor_record_offset":3068,"inlinee":9736,"source_line":1826,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$66::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":105,"record_offset":6548,"parent_record_offset":6484,"ancestor_record_offset":3068,"inlinee":7225,"source_line":1824,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":105,"record_offset":7032,"parent_record_offset":6972,"ancestor_record_offset":3068,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":105,"record_offset":5536,"parent_record_offset":5512,"ancestor_record_offset":3068,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":105,"record_offset":5588,"parent_record_offset":5560,"ancestor_record_offset":3068,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":105,"record_offset":8496,"parent_record_offset":8472,"ancestor_record_offset":3068,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":105,"record_offset":5512,"parent_record_offset":5488,"ancestor_record_offset":3068,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":105,"record_offset":5648,"parent_record_offset":5588,"ancestor_record_offset":3068,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":105,"record_offset":4840,"parent_record_offset":4800,"ancestor_record_offset":3068,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":105,"record_offset":7492,"parent_record_offset":7464,"ancestor_record_offset":3068,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":105,"record_offset":4060,"parent_record_offset":3884,"ancestor_record_offset":3068,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":105,"record_offset":4612,"parent_record_offset":3448,"ancestor_record_offset":3068,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":105,"record_offset":8292,"parent_record_offset":8264,"ancestor_record_offset":3068,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":105,"record_offset":5788,"parent_record_offset":5140,"ancestor_record_offset":3068,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":105,"record_offset":3844,"parent_record_offset":3820,"ancestor_record_offset":3068,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":105,"record_offset":4020,"parent_record_offset":3996,"ancestor_record_offset":3068,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":105,"record_offset":7176,"parent_record_offset":7108,"ancestor_record_offset":3068,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":105,"record_offset":3944,"parent_record_offset":3908,"ancestor_record_offset":3068,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":105,"record_offset":4108,"parent_record_offset":4084,"ancestor_record_offset":3068,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":105,"record_offset":8036,"parent_record_offset":8008,"ancestor_record_offset":3068,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":105,"record_offset":5700,"parent_record_offset":5672,"ancestor_record_offset":3068,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":105,"record_offset":3796,"parent_record_offset":3624,"ancestor_record_offset":3068,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":105,"record_offset":9624,"parent_record_offset":9600,"ancestor_record_offset":3068,"inlinee":8637,"source_line":1069,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_mut_checked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":105,"record_offset":7004,"parent_record_offset":6972,"ancestor_record_offset":3068,"inlinee":5493,"source_line":2154,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":105,"record_offset":6160,"parent_record_offset":6136,"ancestor_record_offset":3068,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":105,"record_offset":4892,"parent_record_offset":4868,"ancestor_record_offset":3068,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_mut_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":105,"record_offset":6136,"parent_record_offset":6064,"ancestor_record_offset":3068,"inlinee":5242,"source_line":112,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":105,"record_offset":4868,"parent_record_offset":4800,"ancestor_record_offset":3068,"inlinee":4612,"source_line":101,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":105,"record_offset":4176,"parent_record_offset":4148,"ancestor_record_offset":3068,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":105,"record_offset":6032,"parent_record_offset":6000,"ancestor_record_offset":3068,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":105,"record_offset":4204,"parent_record_offset":4176,"ancestor_record_offset":3068,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":105,"record_offset":11404,"parent_record_offset":11376,"ancestor_record_offset":3068,"inlinee":5488,"source_line":278,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":105,"record_offset":4800,"parent_record_offset":4768,"ancestor_record_offset":3068,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":105,"record_offset":6064,"parent_record_offset":6032,"ancestor_record_offset":3068,"inlinee":5308,"source_line":444,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":105,"record_offset":3820,"parent_record_offset":3796,"ancestor_record_offset":3068,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":105,"record_offset":3996,"parent_record_offset":3972,"ancestor_record_offset":3068,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":105,"record_offset":9648,"parent_record_offset":9624,"ancestor_record_offset":3068,"inlinee":8638,"source_line":221,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":105,"record_offset":9800,"parent_record_offset":9776,"ancestor_record_offset":3068,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":105,"record_offset":5560,"parent_record_offset":5536,"ancestor_record_offset":3068,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":105,"record_offset":3908,"parent_record_offset":3884,"ancestor_record_offset":3068,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":105,"record_offset":3972,"parent_record_offset":3908,"ancestor_record_offset":3068,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":105,"record_offset":9776,"parent_record_offset":9712,"ancestor_record_offset":3068,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::device::GraphicsDevice::issue_and_reset_pipeline_barrier
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/barrier.rs
//     evidence={"module":105,"record_offset":11016,"parent_record_offset":9928,"ancestor_record_offset":3068,"inlinee":8711,"source_line":419,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":105,"record_offset":4388,"parent_record_offset":4340,"ancestor_record_offset":3068,"inlinee":11948,"source_line":14,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":105,"record_offset":4444,"parent_record_offset":4340,"ancestor_record_offset":3068,"inlinee":11952,"source_line":23,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":105,"record_offset":4416,"parent_record_offset":4340,"ancestor_record_offset":3068,"inlinee":11950,"source_line":32,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::impl$4::get_tracked
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":105,"record_offset":4340,"parent_record_offset":3448,"ancestor_record_offset":3068,"inlinee":11946,"source_line":329,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::compiled::impl$1::record_batch::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/compiled.rs
//     evidence={"module":105,"record_offset":5616,"parent_record_offset":5588,"ancestor_record_offset":3068,"inlinee":11872,"source_line":154,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::graph::ExportableResource::index
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/graph.rs
//     evidence={"module":105,"record_offset":10248,"parent_record_offset":9928,"ancestor_record_offset":3068,"inlinee":11958,"source_line":33,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::graph::ExportableResource::raw
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/graph.rs
//     evidence={"module":105,"record_offset":10272,"parent_record_offset":10248,"ancestor_record_offset":3068,"inlinee":11959,"source_line":25,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":105,"record_offset":7236,"parent_record_offset":7064,"ancestor_record_offset":3068,"inlinee":4947,"source_line":1189,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":105,"record_offset":8264,"parent_record_offset":8228,"ancestor_record_offset":3068,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":105,"record_offset":8228,"parent_record_offset":8200,"ancestor_record_offset":3068,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::compiled::impl$1::record_batch::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 154
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2E60
// 首条位置: {"module":104,"inlinee":11872}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::compiled::impl$1::record_batch::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 162
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2201
// 首条位置: {"module":43,"inlinee":8705}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":43,"record_offset":5760,"parent_record_offset":1096,"ancestor_record_offset":1096,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":43,"record_offset":1324,"parent_record_offset":1096,"ancestor_record_offset":1096,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":43,"record_offset":2332,"parent_record_offset":1096,"ancestor_record_offset":1096,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":43,"record_offset":1920,"parent_record_offset":1096,"ancestor_record_offset":1096,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":43,"record_offset":1892,"parent_record_offset":1096,"ancestor_record_offset":1096,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":43,"record_offset":1804,"parent_record_offset":1096,"ancestor_record_offset":1096,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":43,"record_offset":4424,"parent_record_offset":1096,"ancestor_record_offset":1096,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":43,"record_offset":4540,"parent_record_offset":1096,"ancestor_record_offset":1096,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":43,"record_offset":4644,"parent_record_offset":1096,"ancestor_record_offset":1096,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":43,"record_offset":4604,"parent_record_offset":1096,"ancestor_record_offset":1096,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":43,"record_offset":1720,"parent_record_offset":1096,"ancestor_record_offset":1096,"inlinee":8637,"source_line":1069,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":43,"record_offset":2208,"parent_record_offset":1096,"ancestor_record_offset":1096,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_direct] core::time::Duration::as_nanos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":43,"record_offset":5732,"parent_record_offset":1096,"ancestor_record_offset":1096,"inlinee":8712,"source_line":596,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":43,"record_offset":3412,"parent_record_offset":1096,"ancestor_record_offset":1096,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":43,"record_offset":2428,"parent_record_offset":1096,"ancestor_record_offset":1096,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::issue_and_reset_pipeline_barrier
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/barrier.rs
//     evidence={"module":43,"record_offset":2296,"parent_record_offset":1096,"ancestor_record_offset":1096,"inlinee":8711,"source_line":419,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$17::track
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":43,"record_offset":2180,"parent_record_offset":1096,"ancestor_record_offset":1096,"inlinee":8709,"source_line":250,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":43,"record_offset":1640,"parent_record_offset":1616,"ancestor_record_offset":1096,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":43,"record_offset":1664,"parent_record_offset":1640,"ancestor_record_offset":1096,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":43,"record_offset":6160,"parent_record_offset":6136,"ancestor_record_offset":1096,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":43,"record_offset":1616,"parent_record_offset":1440,"ancestor_record_offset":1096,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":43,"record_offset":6136,"parent_record_offset":6044,"ancestor_record_offset":1096,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":43,"record_offset":2024,"parent_record_offset":2000,"ancestor_record_offset":1096,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":43,"record_offset":1380,"parent_record_offset":1352,"ancestor_record_offset":1096,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":43,"record_offset":6072,"parent_record_offset":6044,"ancestor_record_offset":1096,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":43,"record_offset":6044,"parent_record_offset":6020,"ancestor_record_offset":1096,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":43,"record_offset":2072,"parent_record_offset":2048,"ancestor_record_offset":1096,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":43,"record_offset":2048,"parent_record_offset":2024,"ancestor_record_offset":1096,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":43,"record_offset":1440,"parent_record_offset":1408,"ancestor_record_offset":1096,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":43,"record_offset":1408,"parent_record_offset":1380,"ancestor_record_offset":1096,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":43,"record_offset":6020,"parent_record_offset":5996,"ancestor_record_offset":1096,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":43,"record_offset":1476,"parent_record_offset":1440,"ancestor_record_offset":1096,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":43,"record_offset":5796,"parent_record_offset":5760,"ancestor_record_offset":1096,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":43,"record_offset":2000,"parent_record_offset":1976,"ancestor_record_offset":1096,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":43,"record_offset":1976,"parent_record_offset":1944,"ancestor_record_offset":1096,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":43,"record_offset":1352,"parent_record_offset":1324,"ancestor_record_offset":1096,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":43,"record_offset":1944,"parent_record_offset":1920,"ancestor_record_offset":1096,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":43,"record_offset":1500,"parent_record_offset":1476,"ancestor_record_offset":1096,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":43,"record_offset":1524,"parent_record_offset":1500,"ancestor_record_offset":1096,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":43,"record_offset":3804,"parent_record_offset":3776,"ancestor_record_offset":1096,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":43,"record_offset":3856,"parent_record_offset":3832,"ancestor_record_offset":1096,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":43,"record_offset":3832,"parent_record_offset":3804,"ancestor_record_offset":1096,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":43,"record_offset":3744,"parent_record_offset":3712,"ancestor_record_offset":1096,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":43,"record_offset":3776,"parent_record_offset":3744,"ancestor_record_offset":1096,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":43,"record_offset":3948,"parent_record_offset":3776,"ancestor_record_offset":1096,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":43,"record_offset":4060,"parent_record_offset":4036,"ancestor_record_offset":1096,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":43,"record_offset":3880,"parent_record_offset":3856,"ancestor_record_offset":1096,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":43,"record_offset":1548,"parent_record_offset":1524,"ancestor_record_offset":1096,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":43,"record_offset":1572,"parent_record_offset":1548,"ancestor_record_offset":1096,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":43,"record_offset":6104,"parent_record_offset":6072,"ancestor_record_offset":1096,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":43,"record_offset":3276,"parent_record_offset":3244,"ancestor_record_offset":1096,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":43,"record_offset":3576,"parent_record_offset":3536,"ancestor_record_offset":1096,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":43,"record_offset":3360,"parent_record_offset":3332,"ancestor_record_offset":1096,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":43,"record_offset":3680,"parent_record_offset":3536,"ancestor_record_offset":1096,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":43,"record_offset":4012,"parent_record_offset":3712,"ancestor_record_offset":1096,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":43,"record_offset":1768,"parent_record_offset":1744,"ancestor_record_offset":1096,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":43,"record_offset":2848,"parent_record_offset":2824,"ancestor_record_offset":1096,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":43,"record_offset":2824,"parent_record_offset":2800,"ancestor_record_offset":1096,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":43,"record_offset":1856,"parent_record_offset":1828,"ancestor_record_offset":1096,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":43,"record_offset":4476,"parent_record_offset":4448,"ancestor_record_offset":1096,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":43,"record_offset":3904,"parent_record_offset":3880,"ancestor_record_offset":1096,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":43,"record_offset":2360,"parent_record_offset":2332,"ancestor_record_offset":1096,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":43,"record_offset":2388,"parent_record_offset":2360,"ancestor_record_offset":1096,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":43,"record_offset":2116,"parent_record_offset":1944,"ancestor_record_offset":1096,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":43,"record_offset":2776,"parent_record_offset":2752,"ancestor_record_offset":1096,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":43,"record_offset":2800,"parent_record_offset":2776,"ancestor_record_offset":1096,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":43,"record_offset":1744,"parent_record_offset":1720,"ancestor_record_offset":1096,"inlinee":8638,"source_line":221,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":43,"record_offset":3104,"parent_record_offset":3076,"ancestor_record_offset":1096,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":43,"record_offset":2752,"parent_record_offset":2636,"ancestor_record_offset":1096,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":43,"record_offset":1828,"parent_record_offset":1804,"ancestor_record_offset":1096,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":43,"record_offset":3244,"parent_record_offset":3216,"ancestor_record_offset":1096,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":43,"record_offset":2464,"parent_record_offset":2428,"ancestor_record_offset":1096,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":43,"record_offset":2604,"parent_record_offset":2572,"ancestor_record_offset":1096,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":43,"record_offset":2572,"parent_record_offset":2536,"ancestor_record_offset":1096,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":43,"record_offset":2512,"parent_record_offset":2488,"ancestor_record_offset":1096,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":43,"record_offset":2536,"parent_record_offset":2512,"ancestor_record_offset":1096,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":43,"record_offset":2488,"parent_record_offset":2464,"ancestor_record_offset":1096,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":43,"record_offset":3076,"parent_record_offset":2536,"ancestor_record_offset":1096,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":43,"record_offset":3216,"parent_record_offset":3184,"ancestor_record_offset":1096,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":43,"record_offset":3308,"parent_record_offset":3216,"ancestor_record_offset":1096,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":43,"record_offset":3184,"parent_record_offset":3156,"ancestor_record_offset":1096,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":43,"record_offset":3156,"parent_record_offset":2428,"ancestor_record_offset":1096,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":43,"record_offset":3332,"parent_record_offset":3308,"ancestor_record_offset":1096,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":43,"record_offset":2636,"parent_record_offset":2604,"ancestor_record_offset":1096,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":43,"record_offset":2920,"parent_record_offset":2636,"ancestor_record_offset":1096,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":43,"record_offset":2892,"parent_record_offset":2636,"ancestor_record_offset":1096,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":43,"record_offset":2696,"parent_record_offset":2636,"ancestor_record_offset":1096,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":43,"record_offset":3452,"parent_record_offset":3412,"ancestor_record_offset":1096,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":43,"record_offset":4716,"parent_record_offset":4680,"ancestor_record_offset":1096,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":43,"record_offset":5284,"parent_record_offset":5012,"ancestor_record_offset":1096,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":43,"record_offset":3604,"parent_record_offset":3576,"ancestor_record_offset":1096,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":43,"record_offset":3712,"parent_record_offset":3536,"ancestor_record_offset":1096,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":43,"record_offset":3984,"parent_record_offset":3712,"ancestor_record_offset":1096,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":43,"record_offset":3632,"parent_record_offset":3604,"ancestor_record_offset":1096,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":43,"record_offset":3536,"parent_record_offset":3492,"ancestor_record_offset":1096,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":43,"record_offset":3492,"parent_record_offset":3452,"ancestor_record_offset":1096,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::compiled::impl$1::record_batch::closure$1<country_core::render::pass_builder::pass_resources::PassResources>
// 记录形式: compiled_procedure; instances=1
// 源码行: 162
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":105,"record_offset":13020,"rva":2896448,"va":5371605568}
// 编译实例 RVAs: 0x2c3240
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14002e900
//     owner=unresolved_namespace; count=1
//     target_aliases=130 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2708248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e8e8acd6ff","size":5,"soft_fail":false,"target_va":5368899840,"va":5371608083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a2580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2233860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e8fce9ddff","size":5,"soft_fail":false,"target_va":5369374080,"va":5371607935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dcef0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1994229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e80b92e1ff","size":5,"soft_fail":false,"target_va":5369614064,"va":5371608288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402c3d00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2119","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e847080000","size":5,"soft_fail":false,"target_va":5371608320,"va":5371606196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d4610
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3213739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e8ab093100","size":5,"soft_fail":false,"target_va":5374821904,"va":5371608160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f9ee0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3367480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e838623300","size":5,"soft_fail":false,"target_va":5374975712,"va":5371608227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3475726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e80e093500","size":5,"soft_fail":false,"target_va":5375081488,"va":5371605757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3474251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e84b033500","size":5,"soft_fail":false,"target_va":5375081504,"va":5371607248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd4b40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9507823","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e8ef139100","size":5,"soft_fail":false,"target_va":5381114688,"va":5371606860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6650
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9515444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e8b4319100","size":5,"soft_fail":false,"target_va":5381121616,"va":5371606167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6950
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9516133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e865349100","size":5,"soft_fail":false,"target_va":5381122384,"va":5371606246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6980
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9516219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e8bb349100","size":5,"soft_fail":false,"target_va":5381122432,"va":5371606208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6a70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9516581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e825369100","size":5,"soft_fail":false,"target_va":5381122672,"va":5371606086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be2880
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9565302","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e876f49100","size":5,"soft_fail":false,"target_va":5381171328,"va":5371606021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bee030
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9612113","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e851ab9200","size":5,"soft_fail":false,"target_va":5381218352,"va":5371606234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bee090
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9612162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e882ab9200","size":5,"soft_fail":false,"target_va":5381218448,"va":5371606281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bef4a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9617917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e8fdc19200","size":5,"soft_fail":false,"target_va":5381223584,"va":5371605662,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140befb00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9619209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e809c79200","size":5,"soft_fail":false,"target_va":5381225216,"va":5371606002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bf09b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9622667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e88bd49200","size":5,"soft_fail":false,"target_va":5381228976,"va":5371606304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bf0b40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9622284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e80cd39200","size":5,"soft_fail":false,"target_va":5381229376,"va":5371607087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bfc750
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9671885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e8cd949300","size":5,"soft_fail":false,"target_va":5381277520,"va":5371605630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a1640
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39706521","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e899df5d02","size":5,"soft_fail":false,"target_va":5411313216,"va":5371606690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2747810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e8a2ed2900","size":5,"soft_fail":false,"target_va":5374354992,"va":5371607177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39933259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e84b556102","size":5,"soft_fail":false,"target_va":5411540963,"va":5371607699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39934130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e8b2586102","size":5,"soft_fail":false,"target_va":5411541584,"va":5371607449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39934177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e8e1586102","size":5,"soft_fail":false,"target_va":5411541776,"va":5371607594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39934564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e8645a6102","size":5,"soft_fail":false,"target_va":5411542064,"va":5371607495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39934680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e8d85a6102","size":5,"soft_fail":false,"target_va":5411542296,"va":5371607611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39935295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e83f5d6102","size":5,"soft_fail":false,"target_va":5411542880,"va":5371607580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39935676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e8bc5e6102","size":5,"soft_fail":false,"target_va":5411543360,"va":5371607679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] gpu_profiler::backend::ash::ActiveProfiler::write_marker
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"gpu-profiler","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9755629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e8eddb9400","size":5,"soft_fail":false,"target_va":5381362000,"va":5371606366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9778758","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e846369500","size":5,"soft_fail":false,"target_va":5381385536,"va":5371606773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9783987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e8b34a9500","size":5,"soft_fail":false,"target_va":5381390640,"va":5371606648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9779413","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e8d5389500","size":5,"soft_fail":false,"target_va":5381386480,"va":5371607062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9778328","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e898349500","size":5,"soft_fail":false,"target_va":5381384704,"va":5371606371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::leek::begin_rg_pass
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9565883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e8bbf69100","size":5,"soft_fail":false,"target_va":5381171920,"va":5371606032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::leek::end_rg_pass
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9564887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e8d7f29100","size":5,"soft_fail":false,"target_va":5381171984,"va":5371607092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::command_buffer::CommandBuffer::begin
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9609077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e8759f9200","size":5,"soft_fail":false,"target_va":5381214768,"va":5371605686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::executor::PassPgoDuration::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9593867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e80b649200","size":5,"soft_fail":false,"target_va":5381201008,"va":5371607136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource::RuntimeResource::borrow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9517073","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e811389100","size":5,"soft_fail":false,"target_va":5381123200,"va":5371606122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource_registry::ResourceRegistry::resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9585745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e851449200","size":5,"soft_fail":false,"target_va":5381191856,"va":5371606106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39929474","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e882466102","size":5,"soft_fail":false,"target_va":5411537104,"va":5371607625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::elapsed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11141315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e8c300aa00","size":5,"soft_fail":false,"target_va":5382748432,"va":5371607112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::now
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11141680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371605568,"raw_hex":"e83002aa00","size":5,"soft_fail":false,"target_va":5382747616,"va":5371605931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5371605568,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5371606942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":17728,"parent_record_offset":13020,"ancestor_record_offset":13020,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":13292,"parent_record_offset":13020,"ancestor_record_offset":13020,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":14300,"parent_record_offset":13020,"ancestor_record_offset":13020,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":13888,"parent_record_offset":13020,"ancestor_record_offset":13020,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":105,"record_offset":13860,"parent_record_offset":13020,"ancestor_record_offset":13020,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":105,"record_offset":13772,"parent_record_offset":13020,"ancestor_record_offset":13020,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":105,"record_offset":16388,"parent_record_offset":13020,"ancestor_record_offset":13020,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":105,"record_offset":16504,"parent_record_offset":13020,"ancestor_record_offset":13020,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":105,"record_offset":16608,"parent_record_offset":13020,"ancestor_record_offset":13020,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":105,"record_offset":16568,"parent_record_offset":13020,"ancestor_record_offset":13020,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":105,"record_offset":13688,"parent_record_offset":13020,"ancestor_record_offset":13020,"inlinee":8637,"source_line":1069,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":105,"record_offset":14176,"parent_record_offset":13020,"ancestor_record_offset":13020,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] core::time::Duration::as_nanos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":105,"record_offset":17700,"parent_record_offset":13020,"ancestor_record_offset":13020,"inlinee":8712,"source_line":596,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":105,"record_offset":15380,"parent_record_offset":13020,"ancestor_record_offset":13020,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":105,"record_offset":14396,"parent_record_offset":13020,"ancestor_record_offset":13020,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::issue_and_reset_pipeline_barrier
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/barrier.rs
//     evidence={"module":105,"record_offset":14264,"parent_record_offset":13020,"ancestor_record_offset":13020,"inlinee":8711,"source_line":419,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$17::track
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":105,"record_offset":14148,"parent_record_offset":13020,"ancestor_record_offset":13020,"inlinee":8709,"source_line":250,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":105,"record_offset":13608,"parent_record_offset":13584,"ancestor_record_offset":13020,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":105,"record_offset":13632,"parent_record_offset":13608,"ancestor_record_offset":13020,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":105,"record_offset":18128,"parent_record_offset":18104,"ancestor_record_offset":13020,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":105,"record_offset":13584,"parent_record_offset":13408,"ancestor_record_offset":13020,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":105,"record_offset":18104,"parent_record_offset":18012,"ancestor_record_offset":13020,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":105,"record_offset":13992,"parent_record_offset":13968,"ancestor_record_offset":13020,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":105,"record_offset":13348,"parent_record_offset":13320,"ancestor_record_offset":13020,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":105,"record_offset":18040,"parent_record_offset":18012,"ancestor_record_offset":13020,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":105,"record_offset":18012,"parent_record_offset":17988,"ancestor_record_offset":13020,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":105,"record_offset":14040,"parent_record_offset":14016,"ancestor_record_offset":13020,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":105,"record_offset":14016,"parent_record_offset":13992,"ancestor_record_offset":13020,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":105,"record_offset":13408,"parent_record_offset":13376,"ancestor_record_offset":13020,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":105,"record_offset":13376,"parent_record_offset":13348,"ancestor_record_offset":13020,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":105,"record_offset":17988,"parent_record_offset":17964,"ancestor_record_offset":13020,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":105,"record_offset":13444,"parent_record_offset":13408,"ancestor_record_offset":13020,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":17764,"parent_record_offset":17728,"ancestor_record_offset":13020,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":13968,"parent_record_offset":13944,"ancestor_record_offset":13020,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":13944,"parent_record_offset":13912,"ancestor_record_offset":13020,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":13320,"parent_record_offset":13292,"ancestor_record_offset":13020,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":13912,"parent_record_offset":13888,"ancestor_record_offset":13020,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":105,"record_offset":13468,"parent_record_offset":13444,"ancestor_record_offset":13020,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":105,"record_offset":13492,"parent_record_offset":13468,"ancestor_record_offset":13020,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":105,"record_offset":15768,"parent_record_offset":15740,"ancestor_record_offset":13020,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":105,"record_offset":15820,"parent_record_offset":15796,"ancestor_record_offset":13020,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":105,"record_offset":15796,"parent_record_offset":15768,"ancestor_record_offset":13020,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":105,"record_offset":15708,"parent_record_offset":15676,"ancestor_record_offset":13020,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":105,"record_offset":15740,"parent_record_offset":15708,"ancestor_record_offset":13020,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":105,"record_offset":15912,"parent_record_offset":15740,"ancestor_record_offset":13020,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":105,"record_offset":16024,"parent_record_offset":16000,"ancestor_record_offset":13020,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":105,"record_offset":15844,"parent_record_offset":15820,"ancestor_record_offset":13020,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":105,"record_offset":13516,"parent_record_offset":13492,"ancestor_record_offset":13020,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":105,"record_offset":13540,"parent_record_offset":13516,"ancestor_record_offset":13020,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":105,"record_offset":18072,"parent_record_offset":18040,"ancestor_record_offset":13020,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":105,"record_offset":15244,"parent_record_offset":15212,"ancestor_record_offset":13020,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":105,"record_offset":15544,"parent_record_offset":15504,"ancestor_record_offset":13020,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":105,"record_offset":15328,"parent_record_offset":15300,"ancestor_record_offset":13020,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":105,"record_offset":15644,"parent_record_offset":15504,"ancestor_record_offset":13020,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":105,"record_offset":15976,"parent_record_offset":15676,"ancestor_record_offset":13020,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":105,"record_offset":13736,"parent_record_offset":13712,"ancestor_record_offset":13020,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":105,"record_offset":14816,"parent_record_offset":14792,"ancestor_record_offset":13020,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":105,"record_offset":14792,"parent_record_offset":14768,"ancestor_record_offset":13020,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":105,"record_offset":13824,"parent_record_offset":13796,"ancestor_record_offset":13020,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":105,"record_offset":16440,"parent_record_offset":16412,"ancestor_record_offset":13020,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":105,"record_offset":15868,"parent_record_offset":15844,"ancestor_record_offset":13020,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":105,"record_offset":14328,"parent_record_offset":14300,"ancestor_record_offset":13020,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":105,"record_offset":14356,"parent_record_offset":14328,"ancestor_record_offset":13020,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":105,"record_offset":14084,"parent_record_offset":13912,"ancestor_record_offset":13020,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":105,"record_offset":14744,"parent_record_offset":14720,"ancestor_record_offset":13020,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":105,"record_offset":14768,"parent_record_offset":14744,"ancestor_record_offset":13020,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":105,"record_offset":13712,"parent_record_offset":13688,"ancestor_record_offset":13020,"inlinee":8638,"source_line":221,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":105,"record_offset":15072,"parent_record_offset":15044,"ancestor_record_offset":13020,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":105,"record_offset":14720,"parent_record_offset":14604,"ancestor_record_offset":13020,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":105,"record_offset":13796,"parent_record_offset":13772,"ancestor_record_offset":13020,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":105,"record_offset":15212,"parent_record_offset":15184,"ancestor_record_offset":13020,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":105,"record_offset":14432,"parent_record_offset":14396,"ancestor_record_offset":13020,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":105,"record_offset":14572,"parent_record_offset":14540,"ancestor_record_offset":13020,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":105,"record_offset":14540,"parent_record_offset":14504,"ancestor_record_offset":13020,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":105,"record_offset":14480,"parent_record_offset":14456,"ancestor_record_offset":13020,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":105,"record_offset":14504,"parent_record_offset":14480,"ancestor_record_offset":13020,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":105,"record_offset":14456,"parent_record_offset":14432,"ancestor_record_offset":13020,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":105,"record_offset":15044,"parent_record_offset":14504,"ancestor_record_offset":13020,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":105,"record_offset":15184,"parent_record_offset":15152,"ancestor_record_offset":13020,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":105,"record_offset":15276,"parent_record_offset":15184,"ancestor_record_offset":13020,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":105,"record_offset":15152,"parent_record_offset":15124,"ancestor_record_offset":13020,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":105,"record_offset":15124,"parent_record_offset":14396,"ancestor_record_offset":13020,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":105,"record_offset":15300,"parent_record_offset":15276,"ancestor_record_offset":13020,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":105,"record_offset":14604,"parent_record_offset":14572,"ancestor_record_offset":13020,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":105,"record_offset":14888,"parent_record_offset":14604,"ancestor_record_offset":13020,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":105,"record_offset":14860,"parent_record_offset":14604,"ancestor_record_offset":13020,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":105,"record_offset":14664,"parent_record_offset":14604,"ancestor_record_offset":13020,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":105,"record_offset":15420,"parent_record_offset":15380,"ancestor_record_offset":13020,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":105,"record_offset":16680,"parent_record_offset":16644,"ancestor_record_offset":13020,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":105,"record_offset":17252,"parent_record_offset":16980,"ancestor_record_offset":13020,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":105,"record_offset":15572,"parent_record_offset":15544,"ancestor_record_offset":13020,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":105,"record_offset":15676,"parent_record_offset":15504,"ancestor_record_offset":13020,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":105,"record_offset":15948,"parent_record_offset":15676,"ancestor_record_offset":13020,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":105,"record_offset":15600,"parent_record_offset":15572,"ancestor_record_offset":13020,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":105,"record_offset":15504,"parent_record_offset":15460,"ancestor_record_offset":13020,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":105,"record_offset":15460,"parent_record_offset":15420,"ancestor_record_offset":13020,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::compiled::impl$1::record_batch::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 300
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2200
// 首条位置: {"module":43,"inlinee":8704}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::compiled::impl$1::record_batch::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/compiled.rs
//     evidence={"module":43,"record_offset":1096,"parent_record_offset":1036,"ancestor_record_offset":1036,"inlinee":8705,"source_line":162,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":43,"record_offset":1640,"parent_record_offset":1616,"ancestor_record_offset":1036,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":43,"record_offset":1664,"parent_record_offset":1640,"ancestor_record_offset":1036,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":43,"record_offset":6160,"parent_record_offset":6136,"ancestor_record_offset":1036,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":43,"record_offset":1616,"parent_record_offset":1440,"ancestor_record_offset":1036,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":43,"record_offset":6136,"parent_record_offset":6044,"ancestor_record_offset":1036,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":43,"record_offset":2024,"parent_record_offset":2000,"ancestor_record_offset":1036,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":43,"record_offset":1380,"parent_record_offset":1352,"ancestor_record_offset":1036,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":43,"record_offset":6072,"parent_record_offset":6044,"ancestor_record_offset":1036,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":43,"record_offset":6044,"parent_record_offset":6020,"ancestor_record_offset":1036,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":43,"record_offset":2072,"parent_record_offset":2048,"ancestor_record_offset":1036,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":43,"record_offset":2048,"parent_record_offset":2024,"ancestor_record_offset":1036,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":43,"record_offset":1440,"parent_record_offset":1408,"ancestor_record_offset":1036,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":43,"record_offset":1408,"parent_record_offset":1380,"ancestor_record_offset":1036,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":43,"record_offset":6020,"parent_record_offset":5996,"ancestor_record_offset":1036,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":43,"record_offset":1476,"parent_record_offset":1440,"ancestor_record_offset":1036,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":43,"record_offset":5796,"parent_record_offset":5760,"ancestor_record_offset":1036,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":43,"record_offset":2000,"parent_record_offset":1976,"ancestor_record_offset":1036,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":43,"record_offset":1976,"parent_record_offset":1944,"ancestor_record_offset":1036,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":43,"record_offset":5760,"parent_record_offset":1096,"ancestor_record_offset":1036,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":43,"record_offset":1324,"parent_record_offset":1096,"ancestor_record_offset":1036,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":43,"record_offset":1352,"parent_record_offset":1324,"ancestor_record_offset":1036,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":43,"record_offset":2332,"parent_record_offset":1096,"ancestor_record_offset":1036,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":43,"record_offset":1920,"parent_record_offset":1096,"ancestor_record_offset":1036,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":43,"record_offset":1944,"parent_record_offset":1920,"ancestor_record_offset":1036,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":43,"record_offset":1500,"parent_record_offset":1476,"ancestor_record_offset":1036,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":43,"record_offset":1524,"parent_record_offset":1500,"ancestor_record_offset":1036,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":43,"record_offset":3804,"parent_record_offset":3776,"ancestor_record_offset":1036,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":43,"record_offset":3856,"parent_record_offset":3832,"ancestor_record_offset":1036,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":43,"record_offset":3832,"parent_record_offset":3804,"ancestor_record_offset":1036,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":43,"record_offset":3744,"parent_record_offset":3712,"ancestor_record_offset":1036,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":43,"record_offset":3776,"parent_record_offset":3744,"ancestor_record_offset":1036,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":43,"record_offset":3948,"parent_record_offset":3776,"ancestor_record_offset":1036,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":43,"record_offset":4060,"parent_record_offset":4036,"ancestor_record_offset":1036,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":43,"record_offset":1892,"parent_record_offset":1096,"ancestor_record_offset":1036,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":43,"record_offset":1804,"parent_record_offset":1096,"ancestor_record_offset":1036,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":43,"record_offset":3880,"parent_record_offset":3856,"ancestor_record_offset":1036,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":43,"record_offset":1548,"parent_record_offset":1524,"ancestor_record_offset":1036,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":43,"record_offset":1572,"parent_record_offset":1548,"ancestor_record_offset":1036,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":43,"record_offset":6104,"parent_record_offset":6072,"ancestor_record_offset":1036,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":43,"record_offset":3276,"parent_record_offset":3244,"ancestor_record_offset":1036,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":43,"record_offset":3576,"parent_record_offset":3536,"ancestor_record_offset":1036,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":43,"record_offset":4424,"parent_record_offset":1096,"ancestor_record_offset":1036,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":43,"record_offset":4540,"parent_record_offset":1096,"ancestor_record_offset":1036,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":43,"record_offset":3360,"parent_record_offset":3332,"ancestor_record_offset":1036,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":43,"record_offset":3680,"parent_record_offset":3536,"ancestor_record_offset":1036,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":43,"record_offset":4012,"parent_record_offset":3712,"ancestor_record_offset":1036,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":43,"record_offset":1768,"parent_record_offset":1744,"ancestor_record_offset":1036,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":43,"record_offset":2848,"parent_record_offset":2824,"ancestor_record_offset":1036,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":43,"record_offset":2824,"parent_record_offset":2800,"ancestor_record_offset":1036,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":43,"record_offset":1856,"parent_record_offset":1828,"ancestor_record_offset":1036,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":43,"record_offset":4476,"parent_record_offset":4448,"ancestor_record_offset":1036,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":43,"record_offset":3904,"parent_record_offset":3880,"ancestor_record_offset":1036,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":43,"record_offset":4604,"parent_record_offset":1096,"ancestor_record_offset":1036,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":43,"record_offset":1720,"parent_record_offset":1096,"ancestor_record_offset":1036,"inlinee":8637,"source_line":1069,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":43,"record_offset":2360,"parent_record_offset":2332,"ancestor_record_offset":1036,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":43,"record_offset":2388,"parent_record_offset":2360,"ancestor_record_offset":1036,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":43,"record_offset":2116,"parent_record_offset":1944,"ancestor_record_offset":1036,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":43,"record_offset":2776,"parent_record_offset":2752,"ancestor_record_offset":1036,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":43,"record_offset":2800,"parent_record_offset":2776,"ancestor_record_offset":1036,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":43,"record_offset":1744,"parent_record_offset":1720,"ancestor_record_offset":1036,"inlinee":8638,"source_line":221,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":43,"record_offset":3104,"parent_record_offset":3076,"ancestor_record_offset":1036,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":43,"record_offset":2208,"parent_record_offset":1096,"ancestor_record_offset":1036,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":43,"record_offset":2752,"parent_record_offset":2636,"ancestor_record_offset":1036,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":43,"record_offset":1828,"parent_record_offset":1804,"ancestor_record_offset":1036,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":43,"record_offset":3244,"parent_record_offset":3216,"ancestor_record_offset":1036,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":43,"record_offset":2464,"parent_record_offset":2428,"ancestor_record_offset":1036,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":43,"record_offset":2604,"parent_record_offset":2572,"ancestor_record_offset":1036,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":43,"record_offset":2572,"parent_record_offset":2536,"ancestor_record_offset":1036,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":43,"record_offset":2512,"parent_record_offset":2488,"ancestor_record_offset":1036,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":43,"record_offset":2536,"parent_record_offset":2512,"ancestor_record_offset":1036,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":43,"record_offset":2488,"parent_record_offset":2464,"ancestor_record_offset":1036,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":43,"record_offset":3076,"parent_record_offset":2536,"ancestor_record_offset":1036,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":43,"record_offset":3216,"parent_record_offset":3184,"ancestor_record_offset":1036,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":43,"record_offset":3308,"parent_record_offset":3216,"ancestor_record_offset":1036,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":43,"record_offset":3184,"parent_record_offset":3156,"ancestor_record_offset":1036,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":43,"record_offset":3156,"parent_record_offset":2428,"ancestor_record_offset":1036,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":43,"record_offset":3332,"parent_record_offset":3308,"ancestor_record_offset":1036,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":43,"record_offset":2636,"parent_record_offset":2604,"ancestor_record_offset":1036,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":43,"record_offset":2920,"parent_record_offset":2636,"ancestor_record_offset":1036,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":43,"record_offset":2892,"parent_record_offset":2636,"ancestor_record_offset":1036,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":43,"record_offset":2696,"parent_record_offset":2636,"ancestor_record_offset":1036,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::as_nanos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":43,"record_offset":5732,"parent_record_offset":1096,"ancestor_record_offset":1036,"inlinee":8712,"source_line":596,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":43,"record_offset":3412,"parent_record_offset":1096,"ancestor_record_offset":1036,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":43,"record_offset":3452,"parent_record_offset":3412,"ancestor_record_offset":1036,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":43,"record_offset":4716,"parent_record_offset":4680,"ancestor_record_offset":1036,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":43,"record_offset":5284,"parent_record_offset":5012,"ancestor_record_offset":1036,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":43,"record_offset":3604,"parent_record_offset":3576,"ancestor_record_offset":1036,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":43,"record_offset":3712,"parent_record_offset":3536,"ancestor_record_offset":1036,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":43,"record_offset":3984,"parent_record_offset":3712,"ancestor_record_offset":1036,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":43,"record_offset":2428,"parent_record_offset":1096,"ancestor_record_offset":1036,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::device::GraphicsDevice::issue_and_reset_pipeline_barrier
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/barrier.rs
//     evidence={"module":43,"record_offset":2296,"parent_record_offset":1096,"ancestor_record_offset":1036,"inlinee":8711,"source_line":419,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$17::track
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":43,"record_offset":2180,"parent_record_offset":1096,"ancestor_record_offset":1036,"inlinee":8709,"source_line":250,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":43,"record_offset":3632,"parent_record_offset":3604,"ancestor_record_offset":1036,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":43,"record_offset":3536,"parent_record_offset":3492,"ancestor_record_offset":1036,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":43,"record_offset":3492,"parent_record_offset":3452,"ancestor_record_offset":1036,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00043.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::compiled::impl$1::record_batch::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 315
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x36E3
// 首条位置: {"module":175,"inlinee":14051}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::executor::PassPgoStats::extend
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/executor.rs
//     evidence={"module":175,"record_offset":12440,"parent_record_offset":12396,"ancestor_record_offset":12396,"inlinee":8793,"source_line":162,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":175,"record_offset":12924,"parent_record_offset":12900,"ancestor_record_offset":12396,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":175,"record_offset":12900,"parent_record_offset":12812,"ancestor_record_offset":12396,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":175,"record_offset":12840,"parent_record_offset":12812,"ancestor_record_offset":12396,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":175,"record_offset":12812,"parent_record_offset":12788,"ancestor_record_offset":12396,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":175,"record_offset":12788,"parent_record_offset":12764,"ancestor_record_offset":12396,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":175,"record_offset":12692,"parent_record_offset":12668,"ancestor_record_offset":12396,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":175,"record_offset":12740,"parent_record_offset":12716,"ancestor_record_offset":12396,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":175,"record_offset":12484,"parent_record_offset":12440,"ancestor_record_offset":12396,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":175,"record_offset":12640,"parent_record_offset":12440,"ancestor_record_offset":12396,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":175,"record_offset":12868,"parent_record_offset":12840,"ancestor_record_offset":12396,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":175,"record_offset":12668,"parent_record_offset":12440,"ancestor_record_offset":12396,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":175,"record_offset":12552,"parent_record_offset":12484,"ancestor_record_offset":12396,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":175,"record_offset":12520,"parent_record_offset":12484,"ancestor_record_offset":12396,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":175,"record_offset":12576,"parent_record_offset":12552,"ancestor_record_offset":12396,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":175,"record_offset":12612,"parent_record_offset":12440,"ancestor_record_offset":12396,"inlinee":4947,"source_line":1189,"archive":"_pdb/B/modules/00175.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::compiled::CompiledRenderGraph::gather_batch_preamble_barriers
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 352
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2EA8
// 首条位置: {"module":105,"inlinee":11944}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":3624,"parent_record_offset":3448,"ancestor_record_offset":3448,"inlinee":5295,"source_line":2606,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":3596,"parent_record_offset":3448,"ancestor_record_offset":3448,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":105,"record_offset":3884,"parent_record_offset":3448,"ancestor_record_offset":3448,"inlinee":8249,"source_line":156,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":4148,"parent_record_offset":3448,"ancestor_record_offset":3448,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":105,"record_offset":4612,"parent_record_offset":3448,"ancestor_record_offset":3448,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get_tracked
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":105,"record_offset":4340,"parent_record_offset":3448,"ancestor_record_offset":3448,"inlinee":11946,"source_line":329,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":105,"record_offset":3708,"parent_record_offset":3684,"ancestor_record_offset":3448,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":105,"record_offset":3756,"parent_record_offset":3732,"ancestor_record_offset":3448,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":105,"record_offset":3732,"parent_record_offset":3708,"ancestor_record_offset":3448,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":3684,"parent_record_offset":3624,"ancestor_record_offset":3448,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":3656,"parent_record_offset":3624,"ancestor_record_offset":3448,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":105,"record_offset":4084,"parent_record_offset":4060,"ancestor_record_offset":3448,"inlinee":8251,"source_line":157,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":105,"record_offset":4060,"parent_record_offset":3884,"ancestor_record_offset":3448,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":105,"record_offset":3844,"parent_record_offset":3820,"ancestor_record_offset":3448,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":105,"record_offset":4020,"parent_record_offset":3996,"ancestor_record_offset":3448,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":105,"record_offset":3944,"parent_record_offset":3908,"ancestor_record_offset":3448,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":105,"record_offset":4108,"parent_record_offset":4084,"ancestor_record_offset":3448,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":105,"record_offset":3796,"parent_record_offset":3624,"ancestor_record_offset":3448,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":105,"record_offset":4176,"parent_record_offset":4148,"ancestor_record_offset":3448,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":105,"record_offset":4204,"parent_record_offset":4176,"ancestor_record_offset":3448,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":105,"record_offset":3820,"parent_record_offset":3796,"ancestor_record_offset":3448,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":105,"record_offset":3996,"parent_record_offset":3972,"ancestor_record_offset":3448,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":105,"record_offset":3908,"parent_record_offset":3884,"ancestor_record_offset":3448,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":105,"record_offset":3972,"parent_record_offset":3908,"ancestor_record_offset":3448,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":105,"record_offset":4388,"parent_record_offset":4340,"ancestor_record_offset":3448,"inlinee":11948,"source_line":14,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":105,"record_offset":4444,"parent_record_offset":4340,"ancestor_record_offset":3448,"inlinee":11952,"source_line":23,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":105,"record_offset":4416,"parent_record_offset":4340,"ancestor_record_offset":3448,"inlinee":11950,"source_line":32,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::compiled::CompiledRenderGraph::record_postamble
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 394
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2EB4
// 首条位置: {"module":105,"inlinee":11956}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":10304,"parent_record_offset":9928,"ancestor_record_offset":9928,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":10092,"parent_record_offset":9928,"ancestor_record_offset":9928,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":105,"record_offset":11052,"parent_record_offset":9928,"ancestor_record_offset":9928,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":105,"record_offset":10176,"parent_record_offset":9928,"ancestor_record_offset":9928,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::issue_and_reset_pipeline_barrier
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/barrier.rs
//     evidence={"module":105,"record_offset":11016,"parent_record_offset":9928,"ancestor_record_offset":9928,"inlinee":8711,"source_line":419,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get_tracked
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":105,"record_offset":10744,"parent_record_offset":9928,"ancestor_record_offset":9928,"inlinee":11946,"source_line":329,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::graph::ExportableResource::index
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/graph.rs
//     evidence={"module":105,"record_offset":10248,"parent_record_offset":9928,"ancestor_record_offset":9928,"inlinee":11958,"source_line":33,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":10140,"parent_record_offset":10116,"ancestor_record_offset":9928,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":105,"record_offset":10116,"parent_record_offset":10092,"ancestor_record_offset":9928,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":105,"record_offset":10212,"parent_record_offset":10176,"ancestor_record_offset":9928,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":105,"record_offset":10332,"parent_record_offset":10304,"ancestor_record_offset":9928,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":105,"record_offset":10360,"parent_record_offset":10332,"ancestor_record_offset":9928,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":105,"record_offset":10792,"parent_record_offset":10744,"ancestor_record_offset":9928,"inlinee":11948,"source_line":14,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":105,"record_offset":10848,"parent_record_offset":10744,"ancestor_record_offset":9928,"inlinee":11952,"source_line":23,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":105,"record_offset":10820,"parent_record_offset":10744,"ancestor_record_offset":9928,"inlinee":11950,"source_line":32,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::graph::ExportableResource::raw
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/graph.rs
//     evidence={"module":105,"record_offset":10272,"parent_record_offset":10248,"ancestor_record_offset":9928,"inlinee":11959,"source_line":25,"archive":"_pdb/B/modules/00105.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::compiled::pass_access_range_indices
// 记录形式: compiled_procedure; instances=1
// 源码行: 535
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":698,"record_offset":824,"rva":12490384,"va":5381199504}
// 编译实例 RVAs: 0xbe9690
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::resource_info::ResourceAccessIndex::index
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":698,"record_offset":960,"parent_record_offset":824,"ancestor_record_offset":824,"inlinee":39478,"source_line":214,"archive":"_pdb/B/modules/00698.jsonl.gz"}
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
// 完整结构及原始类型记录: compiled.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::compiled::BatchCompiler
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 2
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::compiled::CompiledRenderGraph
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 7; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::compiled::CompiledRenderGraph::record_batch", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::compiled::CompiledRenderGraph::add_pre_pass_barriers<country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::compiled::CompiledRenderGraph::add_pre_pass_barriers", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::compiled::CompiledRenderGraph::new<country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::compiled::CompiledRenderGraph::new", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::compiled::CompiledRenderGraph<country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::compiled::CompiledRenderGraph", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::rg::compiled::CompiledRenderGraph<country_core::render::pass_builder::pass_resources::PassResources> >", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::rg::compiled::CompiledRenderGraph<country_core::render::pass_builder::pass_resources::PassResources>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: compiled.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: compiled.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 8; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)
// 候选参数类型: ["ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >"]
// [文件级接口候选，具体函数归属 unknown] country_core::render::render_config::VideoMode (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] std::ffi::os_str::OsString (*)(ref$<alloc::string::String>)
// 候选参数类型: ["ref$<alloc::string::String>"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<i64,winit::monitor::VideoMode> >,ref$<tuple$<i64,winit::monitor::VideoMode> >)
// 候选参数类型: ["ref$<tuple$<i64,winit::monitor::VideoMode> >", "ref$<tuple$<i64,winit::monitor::VideoMode> >"]
// [文件级接口候选，具体函数归属 unknown] alloc::string::String (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] serde::__private::de::content::ContentDeserializer<serde_json::error::Error> (*)(enum2$<serde::__private::de::content::Content>)
// 候选参数类型: ["enum2$<serde::__private::de::content::Content>"]
// [文件级接口候选，具体函数归属 unknown] sentry_types::protocol::v7::Breadcrumb (*)(ref$<sentry_types::protocol::v7::Breadcrumb>)
// 候选参数类型: ["ref$<sentry_types::protocol::v7::Breadcrumb>"]

// ------------------------------------------------------------------
// 函数: rhapsody::rg::compiled::BatchCompiler::compile
// 信息 ID: 02264efb57d90a36a821ab7a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::executor::GraphExecutionRecorder::execute_compiled<country_core::render::pass_builder::pass_resources::PassResources>; module=43, inline_record=23980; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::compiled::BatchCompiler::flush_batch
// 信息 ID: 141b6940f0ca7612fe7b72ba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::execute_compiled<country_core::render::pass_builder::pass_resources::PassResources> @ 0xfa406 -> 0xbe9600 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::execute_compiled<country_core::render::pass_builder::pass_resources::PassResources> @ 0xfa41d -> 0xbe9600 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::compiled::BatchCompiler::new
// 信息 ID: 9357789f9f898ed8f3a653d8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::execute_compiled<country_core::render::pass_builder::pass_resources::PassResources> @ 0xfa3c6 -> 0xbe95e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::compiled::CompiledRenderGraph::add_pre_pass_barriers<country_core::render::pass_builder::pass_resources::PassResources>
// 信息 ID: 7e646eff37708a5dc88f3367
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::compiled::CompiledRenderGraph::add_pre_pass_barriers -> ["country_core::render::pass_builder::pass_resources::PassResources"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSliceMut::par_chunk_map_mut::closure$0::async_block_env$0<alloc::vec::Vec<ref_mut$<slice2$<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > >,alloc::alloc::Global>,ref_mut$<slice2$<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > >,rhapsody::rg::compiled::impl$1::record_batch::closure_env$2<country_core::render::pass_builder::pass_resources::PassResources>,tuple$<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global> > > > > @ 0xf874e -> 0x2c3d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::impl$1::record_batch::closure$1<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2c34b4 -> 0x2c3d00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::compiled::CompiledRenderGraph::compute_batches
// 信息 ID: 859e6f1f9f32cc3bbfdc219e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::executor::GraphExecutionRecorder::execute_compiled<country_core::render::pass_builder::pass_resources::PassResources>; module=43, inline_record=23940; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::compiled::CompiledRenderGraph::gather_batch_preamble_barriers
// 信息 ID: 2e847696c7fcf4003a21ddc6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources>; module=105, inline_record=3448; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::compiled::CompiledRenderGraph::new<country_core::render::pass_builder::pass_resources::PassResources>
// 信息 ID: 947006b2d78ba2c7de8437be
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::compiled::CompiledRenderGraph::new -> ["country_core::render::pass_builder::pass_resources::PassResources"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x163c1f -> 0x2c4280 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_loading_screen_render_graph::closure_env$0> @ 0x165959 -> 0x2c4280 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<tuple$<>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::execute_presentation_render_graph::closure_env$0> @ 0x1670f1 -> 0x2c4280 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources>
// 信息 ID: c5ad73f133ca99805df14598
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::compiled::CompiledRenderGraph::record_batch -> ["country_core::render::pass_builder::pass_resources::PassResources"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::execute_compiled<country_core::render::pass_builder::pass_resources::PassResources> @ 0xfaa85 -> 0x2c1d40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::compiled::CompiledRenderGraph::record_postamble
// 信息 ID: acaf907d6590265c8a05a7cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources>; module=105, inline_record=9928; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::compiled::impl$1::new::closure$0
// 信息 ID: 5c601d60645b3943fc388815
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::rg::compiled::impl$1::new", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::rg::compiled::impl$1::new::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<rhapsody::rg::resource::ResourceHandle,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::rg::resource_info::ResourceLifetime> >,rhapsody::rg::compiled::impl$1::new::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<rhapsody::rg::resource::ResourceHandle,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::rg::resource_info::ResourceLifetime> >,rhapsody::rg::compiled::impl$1::new::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources> > >; module=104, inline_record=161168; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<rhapsody::rg::resource::ResourceHandle,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::rg::resource_info::ResourceLifetime> >,rhapsody::rg::compiled::impl$1::new::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources> > >; module=104, inline_record=162004; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::compiled::impl$1::record_batch::closure$0
// 信息 ID: b95f6188f5aaa5313e45a2f9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::rg::compiled::impl$1::record_batch", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources>; module=105, inline_record=5616; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::compiled::impl$1::record_batch::closure$1
// 信息 ID: 8622cddede914ac499c0d324
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::rg::compiled::impl$1::record_batch", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSliceMut::par_chunk_map_mut::closure$0::async_block_env$0<alloc::vec::Vec<ref_mut$<slice2$<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > >,alloc::alloc::Global>,ref_mut$<slice2$<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > >,rhapsody::rg::compiled::impl$1::record_batch::closure_env$2<country_core::render::pass_builder::pass_resources::PassResources>,tuple$<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global> > > > >; module=43, inline_record=1096; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::compiled::impl$1::record_batch::closure$1<country_core::render::pass_builder::pass_resources::PassResources>
// 信息 ID: 5a5412784fe23fcf80133891
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::compiled::impl$1::record_batch::closure$1 -> ["country_core::render::pass_builder::pass_resources::PassResources"]; 字段布局 = unknown。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::rg::compiled::impl$1::record_batch", "captures": "unknown"}
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2c29d8 -> 0x2c3240 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2c2326 -> 0x2c3240 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::compiled::impl$1::record_batch::closure$2
// 信息 ID: 4b878e396cbe98818a49cae1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "rhapsody::rg::compiled::impl$1::record_batch", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::rg::compiled::impl$1::record_batch::closure_env$2 <- std::thread::local::LocalKey::with<core::cell::RefCell<tuple$<parking::Parker,core::task::wake::Waker> >,futures_lite::future::block_on::closure_env$0<alloc::vec::Vec<tuple$<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSliceMut::par_chunk_map_mut::closure_env$0<alloc::vec::Vec<ref_mut$<slice2$<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > >,alloc::alloc::Global>,ref_mut$<slice2$<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > >,rhapsody::rg::compiled::impl$1::record_batch::closure_env$2<country_core::render::pass_builder::pass_resources::PassResources>,tuple$<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global> > >,tuple$<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global> > > > >,alloc::vec::Vec<tuple$<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global> >,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::rg::compiled::impl$1::record_batch::closure_env$2 <- core::ptr::drop_in_place<enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn_env$0<tuple$<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global> >,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSliceMut::par_chunk_map_mut::closure_env$0<alloc::vec::Vec<ref_mut$<slice2$<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > >,alloc::alloc::Global>,ref_mut$<slice2$<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > >,rhapsody::rg::compiled::impl$1::record_batch::closure_env$2<country_core::render::pass_builder::pass_resources::PassResources>,tuple$<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global> > >,tuple$<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global> > > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::rg::compiled::impl$1::record_batch::closure_env$2 <- core::ptr::drop_in_place<enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn_env$0<tuple$<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global> >,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSliceMut::par_chunk_map_mut::closure_env$0<alloc::vec::Vec<ref_mut$<slice2$<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > >,alloc::alloc::Global>,ref_mut$<slice2$<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > >,rhapsody::rg::compiled::impl$1::record_batch::closure_env$2<country_core::render::pass_builder::pass_resources::PassResources>,tuple$<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global> > >,tuple$<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global> > > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::rg::compiled::impl$1::record_batch::closure_env$2 <- core::ptr::drop_in_place<enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn_env$0<tuple$<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global> >,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSliceMut::par_chunk_map_mut::closure_env$0<alloc::vec::Vec<ref_mut$<slice2$<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > >,alloc::alloc::Global>,ref_mut$<slice2$<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > >,rhapsody::rg::compiled::impl$1::record_batch::closure_env$2<country_core::render::pass_builder::pass_resources::PassResources>,tuple$<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global> > >,tuple$<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global> > > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::rg::compiled::impl$1::record_batch::closure_env$2 <- core::ptr::drop_in_place<enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn_env$0<tuple$<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global> >,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSliceMut::par_chunk_map_mut::closure_env$0<alloc::vec::Vec<ref_mut$<slice2$<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > >,alloc::alloc::Global>,ref_mut$<slice2$<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > >,rhapsody::rg::compiled::impl$1::record_batch::closure_env$2<country_core::render::pass_builder::pass_resources::PassResources>,tuple$<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global> > >,tuple$<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global> > > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::rg::compiled::impl$1::record_batch::closure_env$2 <- core::ptr::drop_in_place<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSliceMut::par_chunk_map_mut::closure_env$0<alloc::vec::Vec<ref_mut$<slice2$<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > >,alloc::alloc::Global>,ref_mut$<slice2$<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > >,rhapsody::rg::compiled::impl$1::record_batch::closure_env$2<country_core::render::pass_builder::pass_resources::PassResources>,tuple$<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global> > >,tuple$<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::rg::compiled::impl$1::record_batch::closure_env$2 <- core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSliceMut::par_chunk_map_mut::closure$0::async_block_env$0<alloc::vec::Vec<ref_mut$<slice2$<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > >,alloc::alloc::Global>,ref_mut$<slice2$<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > >,rhapsody::rg::compiled::impl$1::record_batch::closure_env$2<country_core::render::pass_builder::pass_resources::PassResources>,tuple$<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::rg::compiled::impl$1::record_batch::closure_env$2 <- core::panic::unwind_safe::impl$28::poll<enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<tuple$<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global> >,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSliceMut::par_chunk_map_mut::closure_env$0<alloc::vec::Vec<ref_mut$<slice2$<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > >,alloc::alloc::Global>,ref_mut$<slice2$<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > >,rhapsody::rg::compiled::impl$1::record_batch::closure_env$2<country_core::render::pass_builder::pass_resources::PassResources>,tuple$<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global> > >,tuple$<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global> > > > > > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 61 条。
// [PDB 内联宿主] core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::slice::ParallelSliceMut::par_chunk_map_mut::closure$0::async_block_env$0<alloc::vec::Vec<ref_mut$<slice2$<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > >,alloc::alloc::Global>,ref_mut$<slice2$<rhapsody::rg::recorder::RecordedPass<country_core::render::pass_builder::pass_resources::PassResources> > >,rhapsody::rg::compiled::impl$1::record_batch::closure_env$2<country_core::render::pass_builder::pass_resources::PassResources>,tuple$<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global> > > > >; module=43, inline_record=1036; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::compiled::impl$1::record_batch::closure$3
// 信息 ID: 581b23a3c47d8e155a670011
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "rhapsody::rg::compiled::impl$1::record_batch", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::rg::compiled::impl$1::record_batch::closure_env$3 <- alloc::vec::into_iter::impl$5::try_fold<tuple$<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global> >,alloc::alloc::Global,alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::command_buffer::CommandBuffer>,core::iter::adapters::map::map_try_fold::closure_env$0<tuple$<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global> >,rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::command_buffer::CommandBuffer>,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::command_buffer::CommandBuffer>,never$> >,rhapsody::rg::compiled::impl$1::record_batch::closure_env$3<country_core::render::pass_builder::pass_resources::PassResources>,alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<rhapsody::raw::command_buffer::CommandBuffer> >,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::command_buffer::CommandBuffer>,never$> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::rg::compiled::impl$1::record_batch::closure_env$3 <- alloc::vec::in_place_collect::from_iter_in_place<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global> >,alloc::alloc::Global>,rhapsody::rg::compiled::impl$1::record_batch::closure_env$3<country_core::render::pass_builder::pass_resources::PassResources> >,rhapsody::raw::command_buffer::CommandBuffer>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::rg::compiled::impl$1::record_batch::closure_env$3 <- core::ptr::drop_in_place<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global> >,alloc::alloc::Global>,rhapsody::rg::compiled::impl$1::record_batch::closure_env$3<country_core::render::pass_builder::pass_resources::PassResources> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] alloc::vec::into_iter::impl$5::try_fold<tuple$<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global> >,alloc::alloc::Global,alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::command_buffer::CommandBuffer>,core::iter::adapters::map::map_try_fold::closure_env$0<tuple$<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global> >,rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::command_buffer::CommandBuffer>,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::command_buffer::CommandBuffer>,never$> >,rhapsody::rg::compiled::impl$1::record_batch::closure_env$3<country_core::render::pass_builder::pass_resources::PassResources>,alloc::vec::in_place_collect::write_in_place_with_drop::closure_env$0<rhapsody::raw::command_buffer::CommandBuffer> >,enum2$<core::result::Result<alloc::vec::in_place_drop::InPlaceDrop<rhapsody::raw::command_buffer::CommandBuffer>,never$> > >; module=175, inline_record=12396; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::compiled::pass_access_range_indices
// 信息 ID: 6b4efbef2d2685031e64eea5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::compiled::CompiledRenderGraph::add_pre_pass_barriers<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2c3d36 -> 0xbe9690 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
