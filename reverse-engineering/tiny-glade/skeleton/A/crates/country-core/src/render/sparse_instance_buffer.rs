// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/render/sparse_instance_buffer.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 9a852ddcd0038c1c6d80bfb9e9fbbdc8db7cca843853b960385a85600811906f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: sparse_instance_buffer.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 234, 266, 360, 361, 363, 366, 368, 369, 370, 371, 373, 397, 445, 446, 485, 494, 500, 738, 751, 877, 886, 895, 1210, 1214, 1218, 1220, 1222, 1224, 1225, 1249, 1280, 1308, 1321, 1375, 1951, 1957, 2278, 2372, 2697, 2749, 2795, 3214, 3217, 3508, 3533, 3867
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bit_iter; // namespace candidate
//   package identities: [{"name":"bit-iter","source":"PDB_registry_path_only","version":"1.1.1"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"PDB_registry_path_only","version":"0.4.9"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"PDB_registry_path_only","version":"0.12.1"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::SparseInstanceBuffer::allocate
// 记录形式: inline_instance, inlinee_declaration; instances=22
// 源码行: 117
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3978
// 首条位置: {"module":266,"inlinee":14712}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":266,"record_offset":7452,"parent_record_offset":7412,"ancestor_record_offset":7412,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::downgrade
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":266,"record_offset":7636,"parent_record_offset":7412,"ancestor_record_offset":7412,"inlinee":11196,"source_line":1613,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":266,"record_offset":7536,"parent_record_offset":7512,"ancestor_record_offset":7412,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":266,"record_offset":7560,"parent_record_offset":7536,"ancestor_record_offset":7412,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":266,"record_offset":7480,"parent_record_offset":7452,"ancestor_record_offset":7412,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":266,"record_offset":7512,"parent_record_offset":7480,"ancestor_record_offset":7412,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":266,"record_offset":7688,"parent_record_offset":7636,"ancestor_record_offset":7412,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_pause
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":266,"record_offset":7768,"parent_record_offset":7744,"ancestor_record_offset":7412,"inlinee":8944,"source_line":21,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::spin_loop
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hint.rs
//     evidence={"module":266,"record_offset":7744,"parent_record_offset":7636,"ancestor_record_offset":7412,"inlinee":8943,"source_line":216,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":266,"record_offset":7712,"parent_record_offset":7688,"ancestor_record_offset":7412,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":266,"record_offset":7588,"parent_record_offset":7560,"ancestor_record_offset":7412,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":266,"record_offset":7856,"parent_record_offset":7636,"ancestor_record_offset":7412,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":266,"record_offset":7800,"parent_record_offset":7636,"ancestor_record_offset":7412,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":266,"record_offset":7880,"parent_record_offset":7856,"ancestor_record_offset":7412,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":266,"record_offset":7824,"parent_record_offset":7800,"ancestor_record_offset":7412,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::SparseInstanceBuffer::free_impl
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 140
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB371
// 首条位置: {"module":751,"inlinee":45937}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":1976,"parent_record_offset":1096,"ancestor_record_offset":1096,"inlinee":5990,"source_line":2129,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":1604,"parent_record_offset":1096,"ancestor_record_offset":1096,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":2268,"parent_record_offset":1096,"ancestor_record_offset":1096,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::rev
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":751,"record_offset":2212,"parent_record_offset":1096,"ancestor_record_offset":1096,"inlinee":4519,"source_line":3348,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":751,"record_offset":1568,"parent_record_offset":1096,"ancestor_record_offset":1096,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":751,"record_offset":1200,"parent_record_offset":1096,"ancestor_record_offset":1096,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":751,"record_offset":1632,"parent_record_offset":1096,"ancestor_record_offset":1096,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::bitmap_unset
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":751,"record_offset":1712,"parent_record_offset":1096,"ancestor_record_offset":1096,"inlinee":45939,"source_line":588,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":751,"record_offset":1804,"parent_record_offset":1096,"ancestor_record_offset":1096,"inlinee":5786,"source_line":479,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":751,"record_offset":1276,"parent_record_offset":1252,"ancestor_record_offset":1096,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":751,"record_offset":1252,"parent_record_offset":1228,"ancestor_record_offset":1096,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":751,"record_offset":1228,"parent_record_offset":1200,"ancestor_record_offset":1096,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":751,"record_offset":2460,"parent_record_offset":2432,"ancestor_record_offset":1096,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":751,"record_offset":1424,"parent_record_offset":1392,"ancestor_record_offset":1096,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":751,"record_offset":2432,"parent_record_offset":2400,"ancestor_record_offset":1096,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":751,"record_offset":2036,"parent_record_offset":2012,"ancestor_record_offset":1096,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":751,"record_offset":2400,"parent_record_offset":2376,"ancestor_record_offset":1096,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":751,"record_offset":1392,"parent_record_offset":1364,"ancestor_record_offset":1096,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":2540,"parent_record_offset":2332,"ancestor_record_offset":1096,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":2012,"parent_record_offset":1976,"ancestor_record_offset":1096,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":2332,"parent_record_offset":2300,"ancestor_record_offset":1096,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":2068,"parent_record_offset":1976,"ancestor_record_offset":1096,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":2376,"parent_record_offset":2332,"ancestor_record_offset":1096,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":2096,"parent_record_offset":1976,"ancestor_record_offset":1096,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$6::next_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":751,"record_offset":2688,"parent_record_offset":2644,"ancestor_record_offset":1096,"inlinee":45740,"source_line":168,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$6::next_back::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":751,"record_offset":2940,"parent_record_offset":2912,"ancestor_record_offset":1096,"inlinee":45742,"source_line":169,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":3040,"parent_record_offset":3012,"ancestor_record_offset":1096,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":751,"record_offset":3104,"parent_record_offset":3040,"ancestor_record_offset":1096,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":751,"record_offset":2512,"parent_record_offset":2332,"ancestor_record_offset":1096,"inlinee":5454,"source_line":13,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":751,"record_offset":3212,"parent_record_offset":3188,"ancestor_record_offset":1096,"inlinee":5437,"source_line":30,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":751,"record_offset":2300,"parent_record_offset":2268,"ancestor_record_offset":1096,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\rev.rs
//     evidence={"module":751,"record_offset":2236,"parent_record_offset":2212,"ancestor_record_offset":1096,"inlinee":4521,"source_line":20,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::impl$1::fold
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\rev.rs
//     evidence={"module":751,"record_offset":2620,"parent_record_offset":2596,"ancestor_record_offset":1096,"inlinee":21622,"source_line":60,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::double_ended::DoubleEndedIterator::rfold
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\double_ended.rs
//     evidence={"module":751,"record_offset":2644,"parent_record_offset":2620,"ancestor_record_offset":1096,"inlinee":21624,"source_line":300,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":751,"record_offset":2596,"parent_record_offset":2332,"ancestor_record_offset":1096,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":751,"record_offset":3012,"parent_record_offset":2644,"ancestor_record_offset":1096,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":751,"record_offset":1456,"parent_record_offset":1424,"ancestor_record_offset":1096,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":751,"record_offset":2912,"parent_record_offset":2688,"ancestor_record_offset":1096,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":751,"record_offset":1312,"parent_record_offset":1200,"ancestor_record_offset":1096,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":751,"record_offset":2172,"parent_record_offset":2148,"ancestor_record_offset":1096,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":751,"record_offset":2872,"parent_record_offset":2848,"ancestor_record_offset":1096,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":751,"record_offset":2848,"parent_record_offset":2820,"ancestor_record_offset":1096,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":751,"record_offset":1676,"parent_record_offset":1632,"ancestor_record_offset":1096,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":751,"record_offset":2968,"parent_record_offset":2940,"ancestor_record_offset":1096,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":751,"record_offset":3072,"parent_record_offset":3040,"ancestor_record_offset":1096,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":751,"record_offset":1912,"parent_record_offset":1828,"ancestor_record_offset":1096,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":751,"record_offset":1936,"parent_record_offset":1912,"ancestor_record_offset":1096,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":751,"record_offset":2124,"parent_record_offset":1976,"ancestor_record_offset":1096,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":751,"record_offset":2148,"parent_record_offset":2124,"ancestor_record_offset":1096,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":751,"record_offset":2820,"parent_record_offset":2720,"ancestor_record_offset":1096,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":751,"record_offset":2720,"parent_record_offset":2688,"ancestor_record_offset":1096,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":3356,"parent_record_offset":3324,"ancestor_record_offset":1096,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":1856,"parent_record_offset":1828,"ancestor_record_offset":1096,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_and
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":1744,"parent_record_offset":1712,"ancestor_record_offset":1096,"inlinee":19100,"source_line":2728,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_and
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":1768,"parent_record_offset":1744,"ancestor_record_offset":1096,"inlinee":19101,"source_line":3439,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":3380,"parent_record_offset":3356,"ancestor_record_offset":1096,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":1880,"parent_record_offset":1856,"ancestor_record_offset":1096,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":751,"record_offset":3300,"parent_record_offset":3276,"ancestor_record_offset":1096,"inlinee":9025,"source_line":1667,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":751,"record_offset":1828,"parent_record_offset":1804,"ancestor_record_offset":1096,"inlinee":5787,"source_line":67,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":751,"record_offset":3324,"parent_record_offset":3300,"ancestor_record_offset":1096,"inlinee":5795,"source_line":94,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
// 记录形式: inline_instance, inlinee_declaration; instances=62
// 源码行: 158
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3979
// 首条位置: {"module":266,"inlinee":14713}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":266,"record_offset":8400,"parent_record_offset":8124,"ancestor_record_offset":8124,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":266,"record_offset":8188,"parent_record_offset":8124,"ancestor_record_offset":8124,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":266,"record_offset":8432,"parent_record_offset":8124,"ancestor_record_offset":8124,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":266,"record_offset":8372,"parent_record_offset":8124,"ancestor_record_offset":8124,"inlinee":9339,"source_line":302,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_direct] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=32
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":266,"record_offset":8224,"parent_record_offset":8124,"ancestor_record_offset":8124,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":266,"record_offset":8488,"parent_record_offset":8460,"ancestor_record_offset":8124,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":266,"record_offset":8512,"parent_record_offset":8488,"ancestor_record_offset":8124,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":266,"record_offset":8544,"parent_record_offset":8460,"ancestor_record_offset":8124,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":266,"record_offset":8312,"parent_record_offset":8224,"ancestor_record_offset":8124,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":266,"record_offset":8336,"parent_record_offset":8312,"ancestor_record_offset":8124,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":266,"record_offset":8256,"parent_record_offset":8224,"ancestor_record_offset":8124,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":266,"record_offset":8280,"parent_record_offset":8256,"ancestor_record_offset":8124,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
//     owner=private_source_ownership_candidate; count=32
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":266,"record_offset":8460,"parent_record_offset":8432,"ancestor_record_offset":8124,"inlinee":14715,"source_line":175,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=68
// 源码行: 175
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x397B
// 首条位置: {"module":266,"inlinee":14715}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":266,"record_offset":8488,"parent_record_offset":8460,"ancestor_record_offset":8460,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":266,"record_offset":8544,"parent_record_offset":8460,"ancestor_record_offset":8460,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":266,"record_offset":8512,"parent_record_offset":8488,"ancestor_record_offset":8460,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush<country_core::geometry::instanced_wall::BrickTransformSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 188
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1375,"record_offset":6536,"rva":16101712,"va":5384810832}
// 编译实例 RVAs: 0xf5b150
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12062593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384810832,"raw_hex":"e87ff047ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5384812300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087d5d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7200930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384810832,"raw_hex":"e85e1f92ff","size":5,"soft_fail":false,"target_va":5377611216,"va":5384812141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f5bad0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384810832,"raw_hex":"e84d090000","size":5,"soft_fail":false,"target_va":5384813264,"va":5384810878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f864b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t175563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384810832,"raw_hex":"e8cbad0200","size":5,"soft_fail":false,"target_va":5384987824,"va":5384812256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-7281540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384810832,"raw_hex":"e87ce490ff","size":5,"soft_fail":false,"target_va":5377529888,"va":5384811423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-13506726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384810832,"raw_hex":"e85ae731ff","size":5,"soft_fail":false,"target_va":5371304880,"va":5384811601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-3138","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384810832,"raw_hex":"e8bef3ffff","size":5,"soft_fail":false,"target_va":5384809136,"va":5384812269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18167749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384810832,"raw_hex":"e8c5371501","size":5,"soft_fail":false,"target_va":5402980128,"va":5384812374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384810832,"raw_hex":"e8d5f9ffff","size":5,"soft_fail":false,"target_va":5384809328,"va":5384810902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t18169672","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384810832,"raw_hex":"e8483f1501","size":5,"soft_fail":false,"target_va":5402980656,"va":5384810979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ProfilerScope::new<alloc::string::String>
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-1002","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384810832,"raw_hex":"e816fcffff","size":5,"soft_fail":false,"target_va":5384810448,"va":5384811445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-8454744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384810832,"raw_hex":"e8a8fd7eff","size":5,"soft_fail":false,"target_va":5376356144,"va":5384810883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1375,"record_offset":8376,"parent_record_offset":6536,"ancestor_record_offset":6536,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1375,"record_offset":10660,"parent_record_offset":6536,"ancestor_record_offset":6536,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":8460,"parent_record_offset":6536,"ancestor_record_offset":6536,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":9388,"parent_record_offset":6536,"ancestor_record_offset":6536,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::flatten::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1375,"record_offset":8600,"parent_record_offset":6536,"ancestor_record_offset":6536,"inlinee":5070,"source_line":65,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1375,"record_offset":11008,"parent_record_offset":6536,"ancestor_record_offset":6536,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::drain_chunk_bitmap
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1375,"record_offset":8488,"parent_record_offset":6536,"ancestor_record_offset":6536,"inlinee":45940,"source_line":255,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1375,"record_offset":6792,"parent_record_offset":6536,"ancestor_record_offset":6536,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1375,"record_offset":7368,"parent_record_offset":6536,"ancestor_record_offset":6536,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1375,"record_offset":10956,"parent_record_offset":10932,"ancestor_record_offset":6536,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1375,"record_offset":10932,"parent_record_offset":10876,"ancestor_record_offset":6536,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1375,"record_offset":8424,"parent_record_offset":8400,"ancestor_record_offset":6536,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1375,"record_offset":10904,"parent_record_offset":10876,"ancestor_record_offset":6536,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1375,"record_offset":9456,"parent_record_offset":9424,"ancestor_record_offset":6536,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1375,"record_offset":10876,"parent_record_offset":10848,"ancestor_record_offset":6536,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1375,"record_offset":10708,"parent_record_offset":10660,"ancestor_record_offset":6536,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":9424,"parent_record_offset":9388,"ancestor_record_offset":6536,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] bit_iter::impl$34::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bit-iter","version":"1.1.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bit-iter-1.1.1/src/lib.rs
//     evidence={"module":1375,"record_offset":8776,"parent_record_offset":8748,"ancestor_record_offset":6536,"inlinee":45942,"source_line":98,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1375,"record_offset":10736,"parent_record_offset":10660,"ancestor_record_offset":6536,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1375,"record_offset":9252,"parent_record_offset":9016,"ancestor_record_offset":6536,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1375,"record_offset":8664,"parent_record_offset":8628,"ancestor_record_offset":6536,"inlinee":5074,"source_line":839,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$35::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1375,"record_offset":8628,"parent_record_offset":8600,"ancestor_record_offset":6536,"inlinee":5072,"source_line":605,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1375,"record_offset":8964,"parent_record_offset":8940,"ancestor_record_offset":6536,"inlinee":5090,"source_line":358,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1375,"record_offset":8940,"parent_record_offset":8628,"ancestor_record_offset":6536,"inlinee":5088,"source_line":49,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1375,"record_offset":8748,"parent_record_offset":8720,"ancestor_record_offset":6536,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1375,"record_offset":8808,"parent_record_offset":8776,"ancestor_record_offset":6536,"inlinee":6317,"source_line":169,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1375,"record_offset":11124,"parent_record_offset":11092,"ancestor_record_offset":6536,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":1375,"record_offset":7004,"parent_record_offset":6908,"ancestor_record_offset":6536,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1375,"record_offset":8720,"parent_record_offset":8664,"ancestor_record_offset":6536,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1375,"record_offset":8868,"parent_record_offset":8844,"ancestor_record_offset":6536,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1375,"record_offset":8692,"parent_record_offset":8664,"ancestor_record_offset":6536,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1375,"record_offset":7960,"parent_record_offset":7500,"ancestor_record_offset":6536,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1375,"record_offset":8400,"parent_record_offset":8376,"ancestor_record_offset":6536,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1375,"record_offset":7316,"parent_record_offset":7288,"ancestor_record_offset":6536,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1375,"record_offset":10800,"parent_record_offset":10660,"ancestor_record_offset":6536,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1375,"record_offset":8560,"parent_record_offset":8536,"ancestor_record_offset":6536,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1375,"record_offset":9328,"parent_record_offset":9304,"ancestor_record_offset":6536,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1375,"record_offset":7736,"parent_record_offset":7712,"ancestor_record_offset":6536,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1375,"record_offset":7712,"parent_record_offset":7688,"ancestor_record_offset":6536,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1375,"record_offset":7920,"parent_record_offset":7888,"ancestor_record_offset":6536,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1375,"record_offset":9488,"parent_record_offset":9388,"ancestor_record_offset":6536,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1375,"record_offset":10764,"parent_record_offset":10660,"ancestor_record_offset":6536,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1375,"record_offset":6940,"parent_record_offset":6908,"ancestor_record_offset":6536,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1375,"record_offset":8512,"parent_record_offset":8488,"ancestor_record_offset":6536,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1375,"record_offset":6968,"parent_record_offset":6940,"ancestor_record_offset":6536,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1375,"record_offset":8536,"parent_record_offset":8512,"ancestor_record_offset":6536,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1375,"record_offset":9304,"parent_record_offset":9280,"ancestor_record_offset":6536,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1375,"record_offset":7688,"parent_record_offset":7664,"ancestor_record_offset":6536,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1375,"record_offset":8016,"parent_record_offset":7992,"ancestor_record_offset":6536,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1375,"record_offset":9280,"parent_record_offset":9252,"ancestor_record_offset":6536,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1375,"record_offset":7664,"parent_record_offset":7556,"ancestor_record_offset":6536,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1375,"record_offset":6908,"parent_record_offset":6880,"ancestor_record_offset":6536,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1375,"record_offset":7396,"parent_record_offset":7368,"ancestor_record_offset":6536,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1375,"record_offset":7528,"parent_record_offset":7500,"ancestor_record_offset":6536,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1375,"record_offset":7500,"parent_record_offset":7468,"ancestor_record_offset":6536,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1375,"record_offset":7444,"parent_record_offset":7420,"ancestor_record_offset":6536,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1375,"record_offset":7468,"parent_record_offset":7444,"ancestor_record_offset":6536,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1375,"record_offset":7420,"parent_record_offset":7396,"ancestor_record_offset":6536,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1375,"record_offset":7992,"parent_record_offset":7468,"ancestor_record_offset":6536,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1375,"record_offset":7104,"parent_record_offset":7072,"ancestor_record_offset":6536,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1375,"record_offset":7264,"parent_record_offset":7104,"ancestor_record_offset":6536,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1375,"record_offset":7072,"parent_record_offset":7048,"ancestor_record_offset":6536,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1375,"record_offset":6824,"parent_record_offset":6792,"ancestor_record_offset":6536,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1375,"record_offset":7288,"parent_record_offset":7264,"ancestor_record_offset":6536,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1375,"record_offset":6880,"parent_record_offset":6848,"ancestor_record_offset":6536,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1375,"record_offset":6848,"parent_record_offset":6824,"ancestor_record_offset":6536,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1375,"record_offset":7556,"parent_record_offset":7528,"ancestor_record_offset":6536,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1375,"record_offset":7804,"parent_record_offset":7556,"ancestor_record_offset":6536,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1375,"record_offset":7776,"parent_record_offset":7556,"ancestor_record_offset":6536,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1375,"record_offset":7608,"parent_record_offset":7556,"ancestor_record_offset":6536,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1375,"record_offset":9128,"parent_record_offset":9096,"ancestor_record_offset":6536,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1375,"record_offset":9184,"parent_record_offset":9096,"ancestor_record_offset":6536,"inlinee":5322,"source_line":2440,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1375,"record_offset":9152,"parent_record_offset":9128,"ancestor_record_offset":6536,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1375,"record_offset":9208,"parent_record_offset":9184,"ancestor_record_offset":6536,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::drain_chunk_bitmap::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1375,"record_offset":9096,"parent_record_offset":9068,"ancestor_record_offset":6536,"inlinee":45946,"source_line":259,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::drain_chunk_bitmap::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1375,"record_offset":8892,"parent_record_offset":8868,"ancestor_record_offset":6536,"inlinee":45944,"source_line":267,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush<country_core::resources::roof_tile_ssbo::RoofTileSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 188
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":751,"record_offset":248,"rva":9282752,"va":5377991872}
// 编译实例 RVAs: 0x8da4c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cf9b0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6336094","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377991872,"raw_hex":"e8a2519fff","size":5,"soft_fail":false,"target_va":5371656624,"va":5377992713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5242343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377991872,"raw_hex":"e81902b0ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5377992050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087d5d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-382819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377991872,"raw_hex":"e89d28faff","size":5,"soft_fail":false,"target_va":5377611216,"va":5377994030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408cc980
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-56866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377991872,"raw_hex":"e8de21ffff","size":5,"soft_fail":false,"target_va":5377935744,"va":5377992605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412c88f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10410816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377991872,"raw_hex":"e840db9e00","size":5,"soft_fail":false,"target_va":5388404976,"va":5377994155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-463412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377991872,"raw_hex":"e8ccedf8ff","size":5,"soft_fail":false,"target_va":5377529888,"va":5377993295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t24926233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377991872,"raw_hex":"e819587c01","size":5,"soft_fail":false,"target_va":5402918528,"va":5377992290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6688599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377991872,"raw_hex":"e8a9f099ff","size":5,"soft_fail":false,"target_va":5371304880,"va":5377993474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t24984624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377991872,"raw_hex":"e8303c7d01","size":5,"soft_fail":false,"target_va":5402978880,"va":5377994251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t24986506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377991872,"raw_hex":"e88a437d01","size":5,"soft_fail":false,"target_va":5402979344,"va":5377992833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t2224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377991872,"raw_hex":"e8b0080000","size":5,"soft_fail":false,"target_va":5377996400,"va":5377994171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t24985830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377991872,"raw_hex":"e8e6407d01","size":5,"soft_fail":false,"target_va":5402980128,"va":5377994293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t3850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377991872,"raw_hex":"e80a0f0000","size":5,"soft_fail":false,"target_va":5377996592,"va":5377992737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t24987837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377991872,"raw_hex":"e8bd487d01","size":5,"soft_fail":false,"target_va":5402980656,"va":5377992814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t24977024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377991872,"raw_hex":"e8801e7d01","size":5,"soft_fail":false,"target_va":5402968992,"va":5377991963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t24978101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377991872,"raw_hex":"e8b5227d01","size":5,"soft_fail":false,"target_va":5402970800,"va":5377992694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t24971698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377991872,"raw_hex":"e8b2097d01","size":5,"soft_fail":false,"target_va":5402963904,"va":5377992201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t24972712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377991872,"raw_hex":"e8a80d7d01","size":5,"soft_fail":false,"target_va":5402965344,"va":5377992627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ProfilerScope::new<alloc::string::String>
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377991872,"raw_hex":"e826110000","size":5,"soft_fail":false,"target_va":5377997712,"va":5377993317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-1636579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377991872,"raw_hex":"e81d07e7ff","size":5,"soft_fail":false,"target_va":5376356144,"va":5377992718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":751,"record_offset":6500,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":751,"record_offset":8784,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":6584,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":7512,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::flatten::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":751,"record_offset":6724,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":5070,"source_line":65,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":751,"record_offset":9136,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::drain_chunk_bitmap
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":751,"record_offset":6612,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":45940,"source_line":255,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::take_out_waste
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":751,"record_offset":500,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":45936,"source_line":377,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":751,"record_offset":4904,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":751,"record_offset":5492,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":751,"record_offset":1276,"parent_record_offset":1252,"ancestor_record_offset":248,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":751,"record_offset":1252,"parent_record_offset":1228,"ancestor_record_offset":248,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":751,"record_offset":1228,"parent_record_offset":1200,"ancestor_record_offset":248,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":751,"record_offset":6548,"parent_record_offset":6524,"ancestor_record_offset":248,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":751,"record_offset":2460,"parent_record_offset":2432,"ancestor_record_offset":248,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":751,"record_offset":1424,"parent_record_offset":1392,"ancestor_record_offset":248,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":751,"record_offset":2432,"parent_record_offset":2400,"ancestor_record_offset":248,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":751,"record_offset":920,"parent_record_offset":896,"ancestor_record_offset":248,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":751,"record_offset":2400,"parent_record_offset":2376,"ancestor_record_offset":248,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":751,"record_offset":1392,"parent_record_offset":1364,"ancestor_record_offset":248,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":751,"record_offset":8832,"parent_record_offset":8784,"ancestor_record_offset":248,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":751,"record_offset":4408,"parent_record_offset":4372,"ancestor_record_offset":248,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":751,"record_offset":572,"parent_record_offset":500,"ancestor_record_offset":248,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":751,"record_offset":660,"parent_record_offset":500,"ancestor_record_offset":248,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":751,"record_offset":4372,"parent_record_offset":4348,"ancestor_record_offset":248,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":2540,"parent_record_offset":2332,"ancestor_record_offset":248,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":896,"parent_record_offset":860,"ancestor_record_offset":248,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":860,"parent_record_offset":500,"ancestor_record_offset":248,"inlinee":5990,"source_line":2129,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":2332,"parent_record_offset":2300,"ancestor_record_offset":248,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":952,"parent_record_offset":860,"ancestor_record_offset":248,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":2376,"parent_record_offset":2332,"ancestor_record_offset":248,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":980,"parent_record_offset":860,"ancestor_record_offset":248,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":751,"record_offset":3996,"parent_record_offset":500,"ancestor_record_offset":248,"inlinee":6077,"source_line":156,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":751,"record_offset":4108,"parent_record_offset":4084,"ancestor_record_offset":248,"inlinee":7344,"source_line":157,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":751,"record_offset":2688,"parent_record_offset":2644,"ancestor_record_offset":248,"inlinee":45740,"source_line":168,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$6::next_back::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":751,"record_offset":2940,"parent_record_offset":2912,"ancestor_record_offset":248,"inlinee":45742,"source_line":169,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":1604,"parent_record_offset":1096,"ancestor_record_offset":248,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":2268,"parent_record_offset":1096,"ancestor_record_offset":248,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":3040,"parent_record_offset":3012,"ancestor_record_offset":248,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":751,"record_offset":3104,"parent_record_offset":3040,"ancestor_record_offset":248,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":751,"record_offset":2512,"parent_record_offset":2332,"ancestor_record_offset":248,"inlinee":5454,"source_line":13,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":751,"record_offset":3212,"parent_record_offset":3188,"ancestor_record_offset":248,"inlinee":5437,"source_line":30,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":751,"record_offset":2300,"parent_record_offset":2268,"ancestor_record_offset":248,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] bit_iter::impl$34::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bit-iter","version":"1.1.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bit-iter-1.1.1/src/lib.rs
//     evidence={"module":751,"record_offset":6900,"parent_record_offset":6872,"ancestor_record_offset":248,"inlinee":45942,"source_line":98,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":751,"record_offset":8860,"parent_record_offset":8784,"ancestor_record_offset":248,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":751,"record_offset":7376,"parent_record_offset":7140,"ancestor_record_offset":248,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":751,"record_offset":6788,"parent_record_offset":6752,"ancestor_record_offset":248,"inlinee":5074,"source_line":839,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$35::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":751,"record_offset":6752,"parent_record_offset":6724,"ancestor_record_offset":248,"inlinee":5072,"source_line":605,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":751,"record_offset":7088,"parent_record_offset":7064,"ancestor_record_offset":248,"inlinee":5090,"source_line":358,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":751,"record_offset":7064,"parent_record_offset":6752,"ancestor_record_offset":248,"inlinee":5088,"source_line":49,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":751,"record_offset":6872,"parent_record_offset":6844,"ancestor_record_offset":248,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\rev.rs
//     evidence={"module":751,"record_offset":2236,"parent_record_offset":2212,"ancestor_record_offset":248,"inlinee":4521,"source_line":20,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\rev.rs
//     evidence={"module":751,"record_offset":2620,"parent_record_offset":2596,"ancestor_record_offset":248,"inlinee":21622,"source_line":60,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::double_ended::DoubleEndedIterator::rfold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\double_ended.rs
//     evidence={"module":751,"record_offset":2644,"parent_record_offset":2620,"ancestor_record_offset":248,"inlinee":21624,"source_line":300,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":751,"record_offset":2596,"parent_record_offset":2332,"ancestor_record_offset":248,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":751,"record_offset":3012,"parent_record_offset":2644,"ancestor_record_offset":248,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::rev
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":751,"record_offset":2212,"parent_record_offset":1096,"ancestor_record_offset":248,"inlinee":4519,"source_line":3348,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":751,"record_offset":6932,"parent_record_offset":6900,"ancestor_record_offset":248,"inlinee":6317,"source_line":169,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":751,"record_offset":1456,"parent_record_offset":1424,"ancestor_record_offset":248,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":751,"record_offset":5128,"parent_record_offset":5032,"ancestor_record_offset":248,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":751,"record_offset":6844,"parent_record_offset":6788,"ancestor_record_offset":248,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":751,"record_offset":6992,"parent_record_offset":6968,"ancestor_record_offset":248,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":751,"record_offset":6816,"parent_record_offset":6788,"ancestor_record_offset":248,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":751,"record_offset":2912,"parent_record_offset":2688,"ancestor_record_offset":248,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":751,"record_offset":6524,"parent_record_offset":6500,"ancestor_record_offset":248,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":751,"record_offset":1568,"parent_record_offset":1096,"ancestor_record_offset":248,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":751,"record_offset":5440,"parent_record_offset":5412,"ancestor_record_offset":248,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":751,"record_offset":1200,"parent_record_offset":1096,"ancestor_record_offset":248,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":751,"record_offset":1056,"parent_record_offset":1032,"ancestor_record_offset":248,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":751,"record_offset":7452,"parent_record_offset":7428,"ancestor_record_offset":248,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":751,"record_offset":4440,"parent_record_offset":4408,"ancestor_record_offset":248,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":751,"record_offset":2872,"parent_record_offset":2848,"ancestor_record_offset":248,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":751,"record_offset":2848,"parent_record_offset":2820,"ancestor_record_offset":248,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":751,"record_offset":1676,"parent_record_offset":1632,"ancestor_record_offset":248,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":751,"record_offset":2968,"parent_record_offset":2940,"ancestor_record_offset":248,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":751,"record_offset":3072,"parent_record_offset":3040,"ancestor_record_offset":248,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":751,"record_offset":8888,"parent_record_offset":8784,"ancestor_record_offset":248,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":751,"record_offset":796,"parent_record_offset":712,"ancestor_record_offset":248,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":751,"record_offset":820,"parent_record_offset":796,"ancestor_record_offset":248,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":751,"record_offset":5064,"parent_record_offset":5032,"ancestor_record_offset":248,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":751,"record_offset":1008,"parent_record_offset":860,"ancestor_record_offset":248,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":751,"record_offset":5092,"parent_record_offset":5064,"ancestor_record_offset":248,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":751,"record_offset":1032,"parent_record_offset":1008,"ancestor_record_offset":248,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":751,"record_offset":7428,"parent_record_offset":7404,"ancestor_record_offset":248,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":751,"record_offset":2820,"parent_record_offset":2720,"ancestor_record_offset":248,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":751,"record_offset":6140,"parent_record_offset":6116,"ancestor_record_offset":248,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":751,"record_offset":1632,"parent_record_offset":1096,"ancestor_record_offset":248,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":751,"record_offset":2720,"parent_record_offset":2688,"ancestor_record_offset":248,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":751,"record_offset":5032,"parent_record_offset":5004,"ancestor_record_offset":248,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":751,"record_offset":5520,"parent_record_offset":5492,"ancestor_record_offset":248,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":751,"record_offset":5652,"parent_record_offset":5624,"ancestor_record_offset":248,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":751,"record_offset":5624,"parent_record_offset":5592,"ancestor_record_offset":248,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":751,"record_offset":5568,"parent_record_offset":5544,"ancestor_record_offset":248,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":751,"record_offset":5592,"parent_record_offset":5568,"ancestor_record_offset":248,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":751,"record_offset":5544,"parent_record_offset":5520,"ancestor_record_offset":248,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":751,"record_offset":6116,"parent_record_offset":5592,"ancestor_record_offset":248,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":751,"record_offset":5228,"parent_record_offset":5196,"ancestor_record_offset":248,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":751,"record_offset":5388,"parent_record_offset":5228,"ancestor_record_offset":248,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":751,"record_offset":5196,"parent_record_offset":5172,"ancestor_record_offset":248,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":751,"record_offset":4940,"parent_record_offset":4904,"ancestor_record_offset":248,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":751,"record_offset":5412,"parent_record_offset":5388,"ancestor_record_offset":248,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":751,"record_offset":5004,"parent_record_offset":4968,"ancestor_record_offset":248,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":751,"record_offset":4968,"parent_record_offset":4940,"ancestor_record_offset":248,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":751,"record_offset":5680,"parent_record_offset":5652,"ancestor_record_offset":248,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":751,"record_offset":5928,"parent_record_offset":5680,"ancestor_record_offset":248,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":751,"record_offset":5900,"parent_record_offset":5680,"ancestor_record_offset":248,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":751,"record_offset":5732,"parent_record_offset":5680,"ancestor_record_offset":248,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":4252,"parent_record_offset":4220,"ancestor_record_offset":248,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":740,"parent_record_offset":712,"ancestor_record_offset":248,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":3356,"parent_record_offset":3324,"ancestor_record_offset":248,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":1856,"parent_record_offset":1828,"ancestor_record_offset":248,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":600,"parent_record_offset":572,"ancestor_record_offset":248,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_and
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":1744,"parent_record_offset":1712,"ancestor_record_offset":248,"inlinee":19100,"source_line":2728,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":7252,"parent_record_offset":7220,"ancestor_record_offset":248,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":7308,"parent_record_offset":7220,"ancestor_record_offset":248,"inlinee":5322,"source_line":2440,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":624,"parent_record_offset":600,"ancestor_record_offset":248,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_and
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":1768,"parent_record_offset":1744,"ancestor_record_offset":248,"inlinee":19101,"source_line":3439,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":3380,"parent_record_offset":3356,"ancestor_record_offset":248,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":764,"parent_record_offset":740,"ancestor_record_offset":248,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":7276,"parent_record_offset":7252,"ancestor_record_offset":248,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":7332,"parent_record_offset":7308,"ancestor_record_offset":248,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":4468,"parent_record_offset":4408,"ancestor_record_offset":248,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":4500,"parent_record_offset":4372,"ancestor_record_offset":248,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::free_impl
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":751,"record_offset":1096,"parent_record_offset":500,"ancestor_record_offset":248,"inlinee":45937,"source_line":140,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::bitmap_unset
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":751,"record_offset":1712,"parent_record_offset":1096,"ancestor_record_offset":248,"inlinee":45939,"source_line":588,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::drain_chunk_bitmap::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":751,"record_offset":7220,"parent_record_offset":7192,"ancestor_record_offset":248,"inlinee":45946,"source_line":259,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::drain_chunk_bitmap::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":751,"record_offset":7016,"parent_record_offset":6992,"ancestor_record_offset":248,"inlinee":45944,"source_line":267,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":751,"record_offset":688,"parent_record_offset":500,"ancestor_record_offset":248,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":751,"record_offset":4196,"parent_record_offset":4172,"ancestor_record_offset":248,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":751,"record_offset":1804,"parent_record_offset":1096,"ancestor_record_offset":248,"inlinee":5786,"source_line":479,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":751,"record_offset":3300,"parent_record_offset":3276,"ancestor_record_offset":248,"inlinee":9025,"source_line":1667,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":751,"record_offset":712,"parent_record_offset":688,"ancestor_record_offset":248,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":751,"record_offset":4220,"parent_record_offset":4196,"ancestor_record_offset":248,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":751,"record_offset":1828,"parent_record_offset":1804,"ancestor_record_offset":248,"inlinee":5787,"source_line":67,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":751,"record_offset":3324,"parent_record_offset":3300,"ancestor_record_offset":248,"inlinee":5795,"source_line":94,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 188
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1214,"record_offset":260,"rva":13206800,"va":5381915920}
// 编译实例 RVAs: 0xc98510
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cf9b0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10260142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381915920,"raw_hex":"e8527163ff","size":5,"soft_fail":false,"target_va":5371656624,"va":5381916761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9166391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381915920,"raw_hex":"e8c92174ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5381916098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087d5d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4306867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381915920,"raw_hex":"e84d48beff","size":5,"soft_fail":false,"target_va":5377611216,"va":5381918078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c97090
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381915920,"raw_hex":"e89ee8ffff","size":5,"soft_fail":false,"target_va":5381910672,"va":5381916653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141773400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11380224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381915920,"raw_hex":"e800a6ad00","size":5,"soft_fail":false,"target_va":5393298432,"va":5381918203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4387460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381915920,"raw_hex":"e87c0dbdff","size":5,"soft_fail":false,"target_va":5377529888,"va":5381917343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t21002185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381915920,"raw_hex":"e8c9774001","size":5,"soft_fail":false,"target_va":5402918528,"va":5381916338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10612647","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381915920,"raw_hex":"e859105eff","size":5,"soft_fail":false,"target_va":5371304880,"va":5381917522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t21060576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381915920,"raw_hex":"e8e05b4101","size":5,"soft_fail":false,"target_va":5402978880,"va":5381918299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t21062458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381915920,"raw_hex":"e83a634101","size":5,"soft_fail":false,"target_va":5402979344,"va":5381916881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t2992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381915920,"raw_hex":"e8b00b0000","size":5,"soft_fail":false,"target_va":5381921216,"va":5381918219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t21061782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381915920,"raw_hex":"e896604101","size":5,"soft_fail":false,"target_va":5402980128,"va":5381918341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381915920,"raw_hex":"e80a120000","size":5,"soft_fail":false,"target_va":5381921408,"va":5381916785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t21063789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381915920,"raw_hex":"e86d684101","size":5,"soft_fail":false,"target_va":5402980656,"va":5381916862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t21052976","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381915920,"raw_hex":"e8303e4101","size":5,"soft_fail":false,"target_va":5402968992,"va":5381916011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t21054053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381915920,"raw_hex":"e865424101","size":5,"soft_fail":false,"target_va":5402970800,"va":5381916742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t21047650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381915920,"raw_hex":"e862294101","size":5,"soft_fail":false,"target_va":5402963904,"va":5381916249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t21048664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381915920,"raw_hex":"e8582d4101","size":5,"soft_fail":false,"target_va":5402965344,"va":5381916675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ProfilerScope::new<alloc::string::String>
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381915920,"raw_hex":"e826140000","size":5,"soft_fail":false,"target_va":5381922528,"va":5381917365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-5560627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381915920,"raw_hex":"e8cd26abff","size":5,"soft_fail":false,"target_va":5376356144,"va":5381916766,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1214,"record_offset":6520,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1214,"record_offset":8804,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1214,"record_offset":6604,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1214,"record_offset":7532,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::flatten::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1214,"record_offset":6744,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5070,"source_line":65,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1214,"record_offset":9152,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::drain_chunk_bitmap
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1214,"record_offset":6632,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":45940,"source_line":255,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::take_out_waste
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1214,"record_offset":520,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":45936,"source_line":377,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1214,"record_offset":4924,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1214,"record_offset":5512,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1214,"record_offset":1296,"parent_record_offset":1272,"ancestor_record_offset":260,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1214,"record_offset":1272,"parent_record_offset":1248,"ancestor_record_offset":260,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1214,"record_offset":1248,"parent_record_offset":1220,"ancestor_record_offset":260,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1214,"record_offset":6568,"parent_record_offset":6544,"ancestor_record_offset":260,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1214,"record_offset":2480,"parent_record_offset":2452,"ancestor_record_offset":260,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1214,"record_offset":1444,"parent_record_offset":1412,"ancestor_record_offset":260,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1214,"record_offset":2452,"parent_record_offset":2420,"ancestor_record_offset":260,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1214,"record_offset":940,"parent_record_offset":916,"ancestor_record_offset":260,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1214,"record_offset":2420,"parent_record_offset":2396,"ancestor_record_offset":260,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1214,"record_offset":1412,"parent_record_offset":1384,"ancestor_record_offset":260,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1214,"record_offset":8852,"parent_record_offset":8804,"ancestor_record_offset":260,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1214,"record_offset":4428,"parent_record_offset":4392,"ancestor_record_offset":260,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1214,"record_offset":592,"parent_record_offset":520,"ancestor_record_offset":260,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1214,"record_offset":680,"parent_record_offset":520,"ancestor_record_offset":260,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1214,"record_offset":4392,"parent_record_offset":4368,"ancestor_record_offset":260,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1214,"record_offset":2560,"parent_record_offset":2352,"ancestor_record_offset":260,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1214,"record_offset":916,"parent_record_offset":880,"ancestor_record_offset":260,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1214,"record_offset":880,"parent_record_offset":520,"ancestor_record_offset":260,"inlinee":5990,"source_line":2129,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1214,"record_offset":2352,"parent_record_offset":2320,"ancestor_record_offset":260,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1214,"record_offset":972,"parent_record_offset":880,"ancestor_record_offset":260,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1214,"record_offset":2396,"parent_record_offset":2352,"ancestor_record_offset":260,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1214,"record_offset":1000,"parent_record_offset":880,"ancestor_record_offset":260,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":1214,"record_offset":4016,"parent_record_offset":520,"ancestor_record_offset":260,"inlinee":6077,"source_line":156,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":1214,"record_offset":4128,"parent_record_offset":4104,"ancestor_record_offset":260,"inlinee":7344,"source_line":157,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":1214,"record_offset":2708,"parent_record_offset":2664,"ancestor_record_offset":260,"inlinee":45740,"source_line":168,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$6::next_back::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":1214,"record_offset":2960,"parent_record_offset":2932,"ancestor_record_offset":260,"inlinee":45742,"source_line":169,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1214,"record_offset":1624,"parent_record_offset":1116,"ancestor_record_offset":260,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1214,"record_offset":2288,"parent_record_offset":1116,"ancestor_record_offset":260,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1214,"record_offset":3060,"parent_record_offset":3032,"ancestor_record_offset":260,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1214,"record_offset":3124,"parent_record_offset":3060,"ancestor_record_offset":260,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1214,"record_offset":2532,"parent_record_offset":2352,"ancestor_record_offset":260,"inlinee":5454,"source_line":13,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1214,"record_offset":3232,"parent_record_offset":3208,"ancestor_record_offset":260,"inlinee":5437,"source_line":30,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1214,"record_offset":2320,"parent_record_offset":2288,"ancestor_record_offset":260,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] bit_iter::impl$34::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bit-iter","version":"1.1.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bit-iter-1.1.1/src/lib.rs
//     evidence={"module":1214,"record_offset":6920,"parent_record_offset":6892,"ancestor_record_offset":260,"inlinee":45942,"source_line":98,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1214,"record_offset":8880,"parent_record_offset":8804,"ancestor_record_offset":260,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1214,"record_offset":7396,"parent_record_offset":7160,"ancestor_record_offset":260,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1214,"record_offset":6808,"parent_record_offset":6772,"ancestor_record_offset":260,"inlinee":5074,"source_line":839,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$35::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1214,"record_offset":6772,"parent_record_offset":6744,"ancestor_record_offset":260,"inlinee":5072,"source_line":605,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1214,"record_offset":7108,"parent_record_offset":7084,"ancestor_record_offset":260,"inlinee":5090,"source_line":358,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1214,"record_offset":7084,"parent_record_offset":6772,"ancestor_record_offset":260,"inlinee":5088,"source_line":49,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1214,"record_offset":6892,"parent_record_offset":6864,"ancestor_record_offset":260,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\rev.rs
//     evidence={"module":1214,"record_offset":2256,"parent_record_offset":2232,"ancestor_record_offset":260,"inlinee":4521,"source_line":20,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\rev.rs
//     evidence={"module":1214,"record_offset":2640,"parent_record_offset":2616,"ancestor_record_offset":260,"inlinee":21622,"source_line":60,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::double_ended::DoubleEndedIterator::rfold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\double_ended.rs
//     evidence={"module":1214,"record_offset":2664,"parent_record_offset":2640,"ancestor_record_offset":260,"inlinee":21624,"source_line":300,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1214,"record_offset":2616,"parent_record_offset":2352,"ancestor_record_offset":260,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1214,"record_offset":3032,"parent_record_offset":2664,"ancestor_record_offset":260,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::rev
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1214,"record_offset":2232,"parent_record_offset":1116,"ancestor_record_offset":260,"inlinee":4519,"source_line":3348,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1214,"record_offset":6952,"parent_record_offset":6920,"ancestor_record_offset":260,"inlinee":6317,"source_line":169,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1214,"record_offset":1476,"parent_record_offset":1444,"ancestor_record_offset":260,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":1214,"record_offset":5148,"parent_record_offset":5052,"ancestor_record_offset":260,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1214,"record_offset":6864,"parent_record_offset":6808,"ancestor_record_offset":260,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1214,"record_offset":7012,"parent_record_offset":6988,"ancestor_record_offset":260,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1214,"record_offset":6836,"parent_record_offset":6808,"ancestor_record_offset":260,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1214,"record_offset":2932,"parent_record_offset":2708,"ancestor_record_offset":260,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1214,"record_offset":6544,"parent_record_offset":6520,"ancestor_record_offset":260,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1214,"record_offset":1588,"parent_record_offset":1116,"ancestor_record_offset":260,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1214,"record_offset":5460,"parent_record_offset":5432,"ancestor_record_offset":260,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1214,"record_offset":1220,"parent_record_offset":1116,"ancestor_record_offset":260,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1214,"record_offset":1076,"parent_record_offset":1052,"ancestor_record_offset":260,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1214,"record_offset":7472,"parent_record_offset":7448,"ancestor_record_offset":260,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1214,"record_offset":4460,"parent_record_offset":4428,"ancestor_record_offset":260,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1214,"record_offset":2892,"parent_record_offset":2868,"ancestor_record_offset":260,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1214,"record_offset":2868,"parent_record_offset":2840,"ancestor_record_offset":260,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1214,"record_offset":1696,"parent_record_offset":1652,"ancestor_record_offset":260,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1214,"record_offset":2988,"parent_record_offset":2960,"ancestor_record_offset":260,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1214,"record_offset":3092,"parent_record_offset":3060,"ancestor_record_offset":260,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1214,"record_offset":8908,"parent_record_offset":8804,"ancestor_record_offset":260,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1214,"record_offset":816,"parent_record_offset":732,"ancestor_record_offset":260,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1214,"record_offset":840,"parent_record_offset":816,"ancestor_record_offset":260,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1214,"record_offset":5084,"parent_record_offset":5052,"ancestor_record_offset":260,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1214,"record_offset":1028,"parent_record_offset":880,"ancestor_record_offset":260,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1214,"record_offset":5112,"parent_record_offset":5084,"ancestor_record_offset":260,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1214,"record_offset":1052,"parent_record_offset":1028,"ancestor_record_offset":260,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1214,"record_offset":7448,"parent_record_offset":7424,"ancestor_record_offset":260,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1214,"record_offset":2840,"parent_record_offset":2740,"ancestor_record_offset":260,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1214,"record_offset":6160,"parent_record_offset":6136,"ancestor_record_offset":260,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1214,"record_offset":1652,"parent_record_offset":1116,"ancestor_record_offset":260,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1214,"record_offset":2740,"parent_record_offset":2708,"ancestor_record_offset":260,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1214,"record_offset":5052,"parent_record_offset":5024,"ancestor_record_offset":260,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1214,"record_offset":5540,"parent_record_offset":5512,"ancestor_record_offset":260,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1214,"record_offset":5672,"parent_record_offset":5644,"ancestor_record_offset":260,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1214,"record_offset":5644,"parent_record_offset":5612,"ancestor_record_offset":260,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1214,"record_offset":5588,"parent_record_offset":5564,"ancestor_record_offset":260,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1214,"record_offset":5612,"parent_record_offset":5588,"ancestor_record_offset":260,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1214,"record_offset":5564,"parent_record_offset":5540,"ancestor_record_offset":260,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1214,"record_offset":6136,"parent_record_offset":5612,"ancestor_record_offset":260,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1214,"record_offset":5248,"parent_record_offset":5216,"ancestor_record_offset":260,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1214,"record_offset":5408,"parent_record_offset":5248,"ancestor_record_offset":260,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1214,"record_offset":5216,"parent_record_offset":5192,"ancestor_record_offset":260,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1214,"record_offset":4960,"parent_record_offset":4924,"ancestor_record_offset":260,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1214,"record_offset":5432,"parent_record_offset":5408,"ancestor_record_offset":260,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1214,"record_offset":5024,"parent_record_offset":4988,"ancestor_record_offset":260,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1214,"record_offset":4988,"parent_record_offset":4960,"ancestor_record_offset":260,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1214,"record_offset":5700,"parent_record_offset":5672,"ancestor_record_offset":260,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1214,"record_offset":5948,"parent_record_offset":5700,"ancestor_record_offset":260,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1214,"record_offset":5920,"parent_record_offset":5700,"ancestor_record_offset":260,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1214,"record_offset":5752,"parent_record_offset":5700,"ancestor_record_offset":260,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1214,"record_offset":4272,"parent_record_offset":4240,"ancestor_record_offset":260,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1214,"record_offset":760,"parent_record_offset":732,"ancestor_record_offset":260,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1214,"record_offset":3376,"parent_record_offset":3344,"ancestor_record_offset":260,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1214,"record_offset":1876,"parent_record_offset":1848,"ancestor_record_offset":260,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1214,"record_offset":620,"parent_record_offset":592,"ancestor_record_offset":260,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_and
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1214,"record_offset":1764,"parent_record_offset":1732,"ancestor_record_offset":260,"inlinee":19100,"source_line":2728,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1214,"record_offset":7272,"parent_record_offset":7240,"ancestor_record_offset":260,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1214,"record_offset":7328,"parent_record_offset":7240,"ancestor_record_offset":260,"inlinee":5322,"source_line":2440,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1214,"record_offset":644,"parent_record_offset":620,"ancestor_record_offset":260,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_and
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1214,"record_offset":1788,"parent_record_offset":1764,"ancestor_record_offset":260,"inlinee":19101,"source_line":3439,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1214,"record_offset":3400,"parent_record_offset":3376,"ancestor_record_offset":260,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1214,"record_offset":784,"parent_record_offset":760,"ancestor_record_offset":260,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1214,"record_offset":7296,"parent_record_offset":7272,"ancestor_record_offset":260,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1214,"record_offset":7352,"parent_record_offset":7328,"ancestor_record_offset":260,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1214,"record_offset":4488,"parent_record_offset":4428,"ancestor_record_offset":260,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1214,"record_offset":4520,"parent_record_offset":4392,"ancestor_record_offset":260,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::free_impl
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1214,"record_offset":1116,"parent_record_offset":520,"ancestor_record_offset":260,"inlinee":45937,"source_line":140,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::bitmap_unset
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1214,"record_offset":1732,"parent_record_offset":1116,"ancestor_record_offset":260,"inlinee":45939,"source_line":588,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::drain_chunk_bitmap::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1214,"record_offset":7240,"parent_record_offset":7212,"ancestor_record_offset":260,"inlinee":45946,"source_line":259,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::drain_chunk_bitmap::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1214,"record_offset":7036,"parent_record_offset":7012,"ancestor_record_offset":260,"inlinee":45944,"source_line":267,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1214,"record_offset":708,"parent_record_offset":520,"ancestor_record_offset":260,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1214,"record_offset":4216,"parent_record_offset":4192,"ancestor_record_offset":260,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1214,"record_offset":1824,"parent_record_offset":1116,"ancestor_record_offset":260,"inlinee":5786,"source_line":479,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1214,"record_offset":3320,"parent_record_offset":3296,"ancestor_record_offset":260,"inlinee":9025,"source_line":1667,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1214,"record_offset":732,"parent_record_offset":708,"ancestor_record_offset":260,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1214,"record_offset":4240,"parent_record_offset":4216,"ancestor_record_offset":260,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1214,"record_offset":1848,"parent_record_offset":1824,"ancestor_record_offset":260,"inlinee":5787,"source_line":67,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1214,"record_offset":3344,"parent_record_offset":3320,"ancestor_record_offset":260,"inlinee":5795,"source_line":94,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::SparseInstanceBuffer::iter_chunk_bitmap
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 237
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF83C
// 首条位置: {"module":1375,"inlinee":63548}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1375,"record_offset":18288,"parent_record_offset":18264,"ancestor_record_offset":18264,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1375,"record_offset":18336,"parent_record_offset":18312,"ancestor_record_offset":18264,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1375,"record_offset":18312,"parent_record_offset":18288,"ancestor_record_offset":18264,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::impl$2::iter_chunk_bitmap::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 241
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF840
// 首条位置: {"module":1375,"inlinee":63552}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1375,"record_offset":19136,"parent_record_offset":19104,"ancestor_record_offset":19104,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1375,"record_offset":19160,"parent_record_offset":19136,"ancestor_record_offset":19104,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::impl$2::iter_chunk_bitmap::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 246
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF83E
// 首条位置: {"module":1375,"inlinee":63550}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::SparseInstanceBuffer::drain_chunk_bitmap
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 255
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB374
// 首条位置: {"module":751,"inlinee":45940}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":751,"record_offset":6636,"parent_record_offset":6612,"ancestor_record_offset":6612,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":751,"record_offset":6684,"parent_record_offset":6660,"ancestor_record_offset":6612,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":751,"record_offset":6660,"parent_record_offset":6636,"ancestor_record_offset":6612,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::impl$2::drain_chunk_bitmap::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 259
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB37A
// 首条位置: {"module":751,"inlinee":45946}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":7252,"parent_record_offset":7220,"ancestor_record_offset":7220,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::store
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":7308,"parent_record_offset":7220,"ancestor_record_offset":7220,"inlinee":5322,"source_line":2440,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":7276,"parent_record_offset":7252,"ancestor_record_offset":7220,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":7332,"parent_record_offset":7308,"ancestor_record_offset":7220,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::impl$2::drain_chunk_bitmap::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 267
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB378
// 首条位置: {"module":751,"inlinee":45944}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::SparseInstanceBuffer::iter_all_chunks
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 277
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF83B
// 首条位置: {"module":1375,"inlinee":63547}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::iter_chunk_bitmap
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1375,"record_offset":18264,"parent_record_offset":18240,"ancestor_record_offset":18240,"inlinee":63548,"source_line":237,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1375,"record_offset":18336,"parent_record_offset":18312,"ancestor_record_offset":18240,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1375,"record_offset":18288,"parent_record_offset":18264,"ancestor_record_offset":18240,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1375,"record_offset":18312,"parent_record_offset":18288,"ancestor_record_offset":18240,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::SparseInstanceBuffer::iter_allocation_chunks
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 284
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF839
// 首条位置: {"module":1375,"inlinee":63545}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1375,"record_offset":11736,"parent_record_offset":11704,"ancestor_record_offset":11704,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1375,"record_offset":11772,"parent_record_offset":11704,"ancestor_record_offset":11704,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1375,"record_offset":11820,"parent_record_offset":11796,"ancestor_record_offset":11704,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1375,"record_offset":11796,"parent_record_offset":11772,"ancestor_record_offset":11704,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::SparseInstanceBuffer::allocation_to_draw_multi_instanced<country_core::geometry::instanced_wall::BrickTransformSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 295
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1375,"record_offset":11452,"rva":16103504,"va":5384812624}
// 编译实例 RVAs: 0xf5b850
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12063269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384812624,"raw_hex":"e8dbed47ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5384812976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-13505709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384812624,"raw_hex":"e853eb31ff","size":5,"soft_fail":false,"target_va":5371307104,"va":5384812808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18165956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384812624,"raw_hex":"e8c4301501","size":5,"soft_fail":false,"target_va":5402978880,"va":5384812919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18166434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384812624,"raw_hex":"e8a2321501","size":5,"soft_fail":false,"target_va":5402979344,"va":5384812905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::iter_allocation_chunks
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1375,"record_offset":11704,"parent_record_offset":11452,"ancestor_record_offset":11452,"inlinee":63545,"source_line":284,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::to_draw_multi_instanced_impl
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1375,"record_offset":11860,"parent_record_offset":11452,"ancestor_record_offset":11452,"inlinee":63546,"source_line":305,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1375,"record_offset":12492,"parent_record_offset":12468,"ancestor_record_offset":11452,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1375,"record_offset":12468,"parent_record_offset":12368,"ancestor_record_offset":11452,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1375,"record_offset":12400,"parent_record_offset":12368,"ancestor_record_offset":11452,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1375,"record_offset":12012,"parent_record_offset":11988,"ancestor_record_offset":11452,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1375,"record_offset":12368,"parent_record_offset":12340,"ancestor_record_offset":11452,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":11988,"parent_record_offset":11952,"ancestor_record_offset":11452,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":11924,"parent_record_offset":11860,"ancestor_record_offset":11452,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":11952,"parent_record_offset":11860,"ancestor_record_offset":11452,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\copied.rs
//     evidence={"module":1375,"record_offset":12076,"parent_record_offset":11860,"ancestor_record_offset":11452,"inlinee":8603,"source_line":47,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1375,"record_offset":12432,"parent_record_offset":12400,"ancestor_record_offset":11452,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1375,"record_offset":12164,"parent_record_offset":12076,"ancestor_record_offset":11452,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1375,"record_offset":11736,"parent_record_offset":11704,"ancestor_record_offset":11452,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1375,"record_offset":12316,"parent_record_offset":11860,"ancestor_record_offset":11452,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1375,"record_offset":11820,"parent_record_offset":11796,"ancestor_record_offset":11452,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1375,"record_offset":12132,"parent_record_offset":12100,"ancestor_record_offset":11452,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1375,"record_offset":12044,"parent_record_offset":11952,"ancestor_record_offset":11452,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1375,"record_offset":11772,"parent_record_offset":11704,"ancestor_record_offset":11452,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1375,"record_offset":11796,"parent_record_offset":11772,"ancestor_record_offset":11452,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1375,"record_offset":12100,"parent_record_offset":12076,"ancestor_record_offset":11452,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::SparseInstanceBuffer::to_draw_multi_instanced_impl
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 305
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF83A
// 首条位置: {"module":1375,"inlinee":63546}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":11924,"parent_record_offset":11860,"ancestor_record_offset":11860,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":11952,"parent_record_offset":11860,"ancestor_record_offset":11860,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\copied.rs
//     evidence={"module":1375,"record_offset":12076,"parent_record_offset":11860,"ancestor_record_offset":11860,"inlinee":8603,"source_line":47,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::flatten::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1375,"record_offset":18608,"parent_record_offset":18380,"ancestor_record_offset":18380,"inlinee":5070,"source_line":65,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1375,"record_offset":12316,"parent_record_offset":11860,"ancestor_record_offset":11860,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1375,"record_offset":12492,"parent_record_offset":12468,"ancestor_record_offset":11860,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1375,"record_offset":12468,"parent_record_offset":12368,"ancestor_record_offset":11860,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1375,"record_offset":12400,"parent_record_offset":12368,"ancestor_record_offset":11860,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1375,"record_offset":12012,"parent_record_offset":11988,"ancestor_record_offset":11860,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1375,"record_offset":12368,"parent_record_offset":12340,"ancestor_record_offset":11860,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":11988,"parent_record_offset":11952,"ancestor_record_offset":11860,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] bit_iter::impl$34::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bit-iter","version":"1.1.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bit-iter-1.1.1/src/lib.rs
//     evidence={"module":1375,"record_offset":18784,"parent_record_offset":18756,"ancestor_record_offset":18380,"inlinee":45942,"source_line":98,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1375,"record_offset":19204,"parent_record_offset":19024,"ancestor_record_offset":18380,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1375,"record_offset":18672,"parent_record_offset":18636,"ancestor_record_offset":18380,"inlinee":5074,"source_line":839,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$35::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1375,"record_offset":18636,"parent_record_offset":18608,"ancestor_record_offset":18380,"inlinee":5072,"source_line":605,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1375,"record_offset":18972,"parent_record_offset":18948,"ancestor_record_offset":18380,"inlinee":5090,"source_line":358,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1375,"record_offset":18948,"parent_record_offset":18636,"ancestor_record_offset":18380,"inlinee":5088,"source_line":49,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1375,"record_offset":18756,"parent_record_offset":18728,"ancestor_record_offset":18380,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1375,"record_offset":18816,"parent_record_offset":18784,"ancestor_record_offset":18380,"inlinee":6317,"source_line":169,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1375,"record_offset":12432,"parent_record_offset":12400,"ancestor_record_offset":11860,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1375,"record_offset":18728,"parent_record_offset":18672,"ancestor_record_offset":18380,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1375,"record_offset":18876,"parent_record_offset":18852,"ancestor_record_offset":18380,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1375,"record_offset":18700,"parent_record_offset":18672,"ancestor_record_offset":18380,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1375,"record_offset":12164,"parent_record_offset":12076,"ancestor_record_offset":11860,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1375,"record_offset":18852,"parent_record_offset":18756,"ancestor_record_offset":18380,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1375,"record_offset":12340,"parent_record_offset":12316,"ancestor_record_offset":11860,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1375,"record_offset":19280,"parent_record_offset":19256,"ancestor_record_offset":18380,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1375,"record_offset":12132,"parent_record_offset":12100,"ancestor_record_offset":11860,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1375,"record_offset":12044,"parent_record_offset":11952,"ancestor_record_offset":11860,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1375,"record_offset":19256,"parent_record_offset":19232,"ancestor_record_offset":18380,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1375,"record_offset":12100,"parent_record_offset":12076,"ancestor_record_offset":11860,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1375,"record_offset":19136,"parent_record_offset":19104,"ancestor_record_offset":18380,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1375,"record_offset":19160,"parent_record_offset":19136,"ancestor_record_offset":18380,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::iter_chunk_bitmap::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1375,"record_offset":19104,"parent_record_offset":19076,"ancestor_record_offset":18380,"inlinee":63552,"source_line":241,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::iter_chunk_bitmap::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1375,"record_offset":18900,"parent_record_offset":18876,"ancestor_record_offset":18380,"inlinee":63550,"source_line":246,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::SparseInstanceBuffer::hide<country_core::geometry::instanced_wall::BrickTransformSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 349
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1375,"record_offset":12544,"rva":16103888,"va":5384813008}
// 编译实例 RVAs: 0xf5b9d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18165753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384813008,"raw_hex":"e8f92f1501","size":5,"soft_fail":false,"target_va":5402978880,"va":5384813122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18166229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384813008,"raw_hex":"e8d5311501","size":5,"soft_fail":false,"target_va":5402979344,"va":5384813110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":12804,"parent_record_offset":12544,"ancestor_record_offset":12544,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1375,"record_offset":12772,"parent_record_offset":12544,"ancestor_record_offset":12544,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1375,"record_offset":12828,"parent_record_offset":12544,"ancestor_record_offset":12544,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::bitmap_unset
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1375,"record_offset":12904,"parent_record_offset":12544,"ancestor_record_offset":12544,"inlinee":45939,"source_line":588,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1375,"record_offset":12868,"parent_record_offset":12828,"ancestor_record_offset":12544,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_and
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1375,"record_offset":12932,"parent_record_offset":12904,"ancestor_record_offset":12544,"inlinee":19100,"source_line":2728,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_and
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1375,"record_offset":12956,"parent_record_offset":12932,"ancestor_record_offset":12544,"inlinee":19101,"source_line":3439,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::SparseInstanceBuffer::show<country_core::geometry::instanced_wall::BrickTransformSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 364
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1375,"record_offset":12996,"rva":16104016,"va":5384813136}
// 编译实例 RVAs: 0xf5ba50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18165627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384813136,"raw_hex":"e87b2f1501","size":5,"soft_fail":false,"target_va":5402978880,"va":5384813248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18166103","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384813136,"raw_hex":"e857311501","size":5,"soft_fail":false,"target_va":5402979344,"va":5384813236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":13256,"parent_record_offset":12996,"ancestor_record_offset":12996,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1375,"record_offset":13224,"parent_record_offset":12996,"ancestor_record_offset":12996,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1375,"record_offset":13280,"parent_record_offset":12996,"ancestor_record_offset":12996,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::bitmap_set
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1375,"record_offset":13356,"parent_record_offset":12996,"ancestor_record_offset":12996,"inlinee":45947,"source_line":583,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1375,"record_offset":13320,"parent_record_offset":13280,"ancestor_record_offset":12996,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1375,"record_offset":13384,"parent_record_offset":13356,"ancestor_record_offset":12996,"inlinee":8935,"source_line":2794,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1375,"record_offset":13408,"parent_record_offset":13384,"ancestor_record_offset":12996,"inlinee":8936,"source_line":3471,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::SparseInstanceBuffer::take_out_waste
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 377
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB370
// 首条位置: {"module":751,"inlinee":45936}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":751,"record_offset":572,"parent_record_offset":500,"ancestor_record_offset":500,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":751,"record_offset":660,"parent_record_offset":500,"ancestor_record_offset":500,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":860,"parent_record_offset":500,"ancestor_record_offset":500,"inlinee":5990,"source_line":2129,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":751,"record_offset":3996,"parent_record_offset":500,"ancestor_record_offset":500,"inlinee":6077,"source_line":156,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":751,"record_offset":4172,"parent_record_offset":500,"ancestor_record_offset":500,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::free_impl
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":751,"record_offset":1096,"parent_record_offset":500,"ancestor_record_offset":500,"inlinee":45937,"source_line":140,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":751,"record_offset":688,"parent_record_offset":500,"ancestor_record_offset":500,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":751,"record_offset":1276,"parent_record_offset":1252,"ancestor_record_offset":500,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":751,"record_offset":1252,"parent_record_offset":1228,"ancestor_record_offset":500,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":751,"record_offset":1228,"parent_record_offset":1200,"ancestor_record_offset":500,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":751,"record_offset":2460,"parent_record_offset":2432,"ancestor_record_offset":500,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":751,"record_offset":1424,"parent_record_offset":1392,"ancestor_record_offset":500,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":751,"record_offset":2432,"parent_record_offset":2400,"ancestor_record_offset":500,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":751,"record_offset":920,"parent_record_offset":896,"ancestor_record_offset":500,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":751,"record_offset":2400,"parent_record_offset":2376,"ancestor_record_offset":500,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":751,"record_offset":1392,"parent_record_offset":1364,"ancestor_record_offset":500,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":751,"record_offset":4408,"parent_record_offset":4372,"ancestor_record_offset":500,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":751,"record_offset":4372,"parent_record_offset":4348,"ancestor_record_offset":500,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":2540,"parent_record_offset":2332,"ancestor_record_offset":500,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":896,"parent_record_offset":860,"ancestor_record_offset":500,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":1976,"parent_record_offset":1096,"ancestor_record_offset":500,"inlinee":5990,"source_line":2129,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":2332,"parent_record_offset":2300,"ancestor_record_offset":500,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":952,"parent_record_offset":860,"ancestor_record_offset":500,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":2376,"parent_record_offset":2332,"ancestor_record_offset":500,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":980,"parent_record_offset":860,"ancestor_record_offset":500,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":751,"record_offset":4108,"parent_record_offset":4084,"ancestor_record_offset":500,"inlinee":7344,"source_line":157,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":751,"record_offset":2688,"parent_record_offset":2644,"ancestor_record_offset":500,"inlinee":45740,"source_line":168,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$6::next_back::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":751,"record_offset":2940,"parent_record_offset":2912,"ancestor_record_offset":500,"inlinee":45742,"source_line":169,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":1604,"parent_record_offset":1096,"ancestor_record_offset":500,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":2268,"parent_record_offset":1096,"ancestor_record_offset":500,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":3040,"parent_record_offset":3012,"ancestor_record_offset":500,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":751,"record_offset":3104,"parent_record_offset":3040,"ancestor_record_offset":500,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":751,"record_offset":2512,"parent_record_offset":2332,"ancestor_record_offset":500,"inlinee":5454,"source_line":13,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":751,"record_offset":3212,"parent_record_offset":3188,"ancestor_record_offset":500,"inlinee":5437,"source_line":30,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":751,"record_offset":2300,"parent_record_offset":2268,"ancestor_record_offset":500,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\rev.rs
//     evidence={"module":751,"record_offset":2236,"parent_record_offset":2212,"ancestor_record_offset":500,"inlinee":4521,"source_line":20,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::impl$1::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\rev.rs
//     evidence={"module":751,"record_offset":2620,"parent_record_offset":2596,"ancestor_record_offset":500,"inlinee":21622,"source_line":60,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::double_ended::DoubleEndedIterator::rfold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\double_ended.rs
//     evidence={"module":751,"record_offset":2644,"parent_record_offset":2620,"ancestor_record_offset":500,"inlinee":21624,"source_line":300,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":751,"record_offset":2596,"parent_record_offset":2332,"ancestor_record_offset":500,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":751,"record_offset":3012,"parent_record_offset":2644,"ancestor_record_offset":500,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::rev
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":751,"record_offset":2212,"parent_record_offset":1096,"ancestor_record_offset":500,"inlinee":4519,"source_line":3348,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":751,"record_offset":1456,"parent_record_offset":1424,"ancestor_record_offset":500,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":751,"record_offset":2912,"parent_record_offset":2688,"ancestor_record_offset":500,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":751,"record_offset":1568,"parent_record_offset":1096,"ancestor_record_offset":500,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":751,"record_offset":1200,"parent_record_offset":1096,"ancestor_record_offset":500,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":751,"record_offset":1056,"parent_record_offset":1032,"ancestor_record_offset":500,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":751,"record_offset":4440,"parent_record_offset":4408,"ancestor_record_offset":500,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":751,"record_offset":2872,"parent_record_offset":2848,"ancestor_record_offset":500,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":751,"record_offset":2848,"parent_record_offset":2820,"ancestor_record_offset":500,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":751,"record_offset":1676,"parent_record_offset":1632,"ancestor_record_offset":500,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":751,"record_offset":2968,"parent_record_offset":2940,"ancestor_record_offset":500,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":751,"record_offset":3072,"parent_record_offset":3040,"ancestor_record_offset":500,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":751,"record_offset":796,"parent_record_offset":712,"ancestor_record_offset":500,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":751,"record_offset":820,"parent_record_offset":796,"ancestor_record_offset":500,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":751,"record_offset":1008,"parent_record_offset":860,"ancestor_record_offset":500,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":751,"record_offset":1032,"parent_record_offset":1008,"ancestor_record_offset":500,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":751,"record_offset":2820,"parent_record_offset":2720,"ancestor_record_offset":500,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":751,"record_offset":1632,"parent_record_offset":1096,"ancestor_record_offset":500,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":751,"record_offset":2720,"parent_record_offset":2688,"ancestor_record_offset":500,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":4252,"parent_record_offset":4220,"ancestor_record_offset":500,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":740,"parent_record_offset":712,"ancestor_record_offset":500,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":3356,"parent_record_offset":3324,"ancestor_record_offset":500,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":1856,"parent_record_offset":1828,"ancestor_record_offset":500,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":600,"parent_record_offset":572,"ancestor_record_offset":500,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_and
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":1744,"parent_record_offset":1712,"ancestor_record_offset":500,"inlinee":19100,"source_line":2728,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":624,"parent_record_offset":600,"ancestor_record_offset":500,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_and
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":1768,"parent_record_offset":1744,"ancestor_record_offset":500,"inlinee":19101,"source_line":3439,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":3380,"parent_record_offset":3356,"ancestor_record_offset":500,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":764,"parent_record_offset":740,"ancestor_record_offset":500,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":4468,"parent_record_offset":4408,"ancestor_record_offset":500,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":4500,"parent_record_offset":4372,"ancestor_record_offset":500,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::bitmap_unset
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":751,"record_offset":1712,"parent_record_offset":1096,"ancestor_record_offset":500,"inlinee":45939,"source_line":588,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":751,"record_offset":4196,"parent_record_offset":4172,"ancestor_record_offset":500,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":751,"record_offset":1804,"parent_record_offset":1096,"ancestor_record_offset":500,"inlinee":5786,"source_line":479,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":751,"record_offset":3300,"parent_record_offset":3276,"ancestor_record_offset":500,"inlinee":9025,"source_line":1667,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":751,"record_offset":712,"parent_record_offset":688,"ancestor_record_offset":500,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":751,"record_offset":4220,"parent_record_offset":4196,"ancestor_record_offset":500,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":751,"record_offset":1828,"parent_record_offset":1804,"ancestor_record_offset":500,"inlinee":5787,"source_line":67,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":751,"record_offset":3324,"parent_record_offset":3300,"ancestor_record_offset":500,"inlinee":5795,"source_line":94,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::SparseInstanceBuffer::take_out_waste<country_core::geometry::instanced_wall::BrickTransformSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 377
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1375,"record_offset":13448,"rva":16104144,"va":5384813264}
// 编译实例 RVAs: 0xf5bad0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cf9b0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13157440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384813264,"raw_hex":"e8c03b37ff","size":5,"soft_fail":false,"target_va":5371656624,"va":5384814059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12063703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384813264,"raw_hex":"e829ec47ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5384813410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d7990
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8930297","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384813264,"raw_hex":"e807bc77ff","size":5,"soft_fail":false,"target_va":5375883664,"va":5384813956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18104874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384813264,"raw_hex":"e82a421401","size":5,"soft_fail":false,"target_va":5402918528,"va":5384813649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18164752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384813264,"raw_hex":"e8102c1501","size":5,"soft_fail":false,"target_va":5402978880,"va":5384814123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18165247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384813264,"raw_hex":"e8ff2d1501","size":5,"soft_fail":false,"target_va":5402979344,"va":5384814092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t18155643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384813264,"raw_hex":"e87b081501","size":5,"soft_fail":false,"target_va":5402968992,"va":5384813344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t18156755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384813264,"raw_hex":"e8d30c1501","size":5,"soft_fail":false,"target_va":5402970800,"va":5384814040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t18150328","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384813264,"raw_hex":"e8b8f31401","size":5,"soft_fail":false,"target_va":5402963904,"va":5384813571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t18151360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384813264,"raw_hex":"e8c0f71401","size":5,"soft_fail":false,"target_va":5402965344,"va":5384813979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1375,"record_offset":13700,"parent_record_offset":13448,"ancestor_record_offset":13448,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1375,"record_offset":13788,"parent_record_offset":13448,"ancestor_record_offset":13448,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":13988,"parent_record_offset":13448,"ancestor_record_offset":13448,"inlinee":5990,"source_line":2129,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":1375,"record_offset":17104,"parent_record_offset":13448,"ancestor_record_offset":13448,"inlinee":6077,"source_line":156,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1375,"record_offset":17280,"parent_record_offset":13448,"ancestor_record_offset":13448,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::free_impl
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1375,"record_offset":14224,"parent_record_offset":13448,"ancestor_record_offset":13448,"inlinee":45937,"source_line":140,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1375,"record_offset":13816,"parent_record_offset":13448,"ancestor_record_offset":13448,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1375,"record_offset":14396,"parent_record_offset":14372,"ancestor_record_offset":13448,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1375,"record_offset":14372,"parent_record_offset":14348,"ancestor_record_offset":13448,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1375,"record_offset":14348,"parent_record_offset":14320,"ancestor_record_offset":13448,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1375,"record_offset":15572,"parent_record_offset":15544,"ancestor_record_offset":13448,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1375,"record_offset":14540,"parent_record_offset":14508,"ancestor_record_offset":13448,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1375,"record_offset":15544,"parent_record_offset":15512,"ancestor_record_offset":13448,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1375,"record_offset":14048,"parent_record_offset":14024,"ancestor_record_offset":13448,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1375,"record_offset":15512,"parent_record_offset":15488,"ancestor_record_offset":13448,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1375,"record_offset":14508,"parent_record_offset":14480,"ancestor_record_offset":13448,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1375,"record_offset":17516,"parent_record_offset":17480,"ancestor_record_offset":13448,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1375,"record_offset":17480,"parent_record_offset":17456,"ancestor_record_offset":13448,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":15652,"parent_record_offset":15448,"ancestor_record_offset":13448,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":14024,"parent_record_offset":13988,"ancestor_record_offset":13448,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":15092,"parent_record_offset":14224,"ancestor_record_offset":13448,"inlinee":5990,"source_line":2129,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":15448,"parent_record_offset":15416,"ancestor_record_offset":13448,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":14080,"parent_record_offset":13988,"ancestor_record_offset":13448,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":15488,"parent_record_offset":15448,"ancestor_record_offset":13448,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":14108,"parent_record_offset":13988,"ancestor_record_offset":13448,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":1375,"record_offset":17216,"parent_record_offset":17192,"ancestor_record_offset":13448,"inlinee":7344,"source_line":157,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":1375,"record_offset":15800,"parent_record_offset":15756,"ancestor_record_offset":13448,"inlinee":45740,"source_line":168,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$6::next_back::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":1375,"record_offset":16052,"parent_record_offset":16024,"ancestor_record_offset":13448,"inlinee":45742,"source_line":169,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":14720,"parent_record_offset":14224,"ancestor_record_offset":13448,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":15384,"parent_record_offset":14224,"ancestor_record_offset":13448,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":16152,"parent_record_offset":16124,"ancestor_record_offset":13448,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1375,"record_offset":16216,"parent_record_offset":16152,"ancestor_record_offset":13448,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1375,"record_offset":15624,"parent_record_offset":15448,"ancestor_record_offset":13448,"inlinee":5454,"source_line":13,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1375,"record_offset":16324,"parent_record_offset":16300,"ancestor_record_offset":13448,"inlinee":5437,"source_line":30,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1375,"record_offset":15416,"parent_record_offset":15384,"ancestor_record_offset":13448,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\rev.rs
//     evidence={"module":1375,"record_offset":15352,"parent_record_offset":15328,"ancestor_record_offset":13448,"inlinee":4521,"source_line":20,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\rev.rs
//     evidence={"module":1375,"record_offset":15732,"parent_record_offset":15708,"ancestor_record_offset":13448,"inlinee":21622,"source_line":60,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::double_ended::DoubleEndedIterator::rfold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\double_ended.rs
//     evidence={"module":1375,"record_offset":15756,"parent_record_offset":15732,"ancestor_record_offset":13448,"inlinee":21624,"source_line":300,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1375,"record_offset":15708,"parent_record_offset":15448,"ancestor_record_offset":13448,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1375,"record_offset":16124,"parent_record_offset":15756,"ancestor_record_offset":13448,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::rev
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1375,"record_offset":15328,"parent_record_offset":14224,"ancestor_record_offset":13448,"inlinee":4519,"source_line":3348,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1375,"record_offset":14572,"parent_record_offset":14540,"ancestor_record_offset":13448,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1375,"record_offset":16024,"parent_record_offset":15800,"ancestor_record_offset":13448,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1375,"record_offset":14684,"parent_record_offset":14224,"ancestor_record_offset":13448,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1375,"record_offset":14320,"parent_record_offset":14224,"ancestor_record_offset":13448,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1375,"record_offset":14184,"parent_record_offset":14160,"ancestor_record_offset":13448,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1375,"record_offset":17548,"parent_record_offset":17516,"ancestor_record_offset":13448,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1375,"record_offset":15984,"parent_record_offset":15960,"ancestor_record_offset":13448,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1375,"record_offset":15960,"parent_record_offset":15932,"ancestor_record_offset":13448,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1375,"record_offset":14792,"parent_record_offset":14748,"ancestor_record_offset":13448,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1375,"record_offset":16080,"parent_record_offset":16052,"ancestor_record_offset":13448,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1375,"record_offset":16184,"parent_record_offset":16152,"ancestor_record_offset":13448,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1375,"record_offset":13924,"parent_record_offset":13840,"ancestor_record_offset":13448,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1375,"record_offset":13948,"parent_record_offset":13924,"ancestor_record_offset":13448,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1375,"record_offset":14136,"parent_record_offset":13988,"ancestor_record_offset":13448,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1375,"record_offset":14160,"parent_record_offset":14136,"ancestor_record_offset":13448,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1375,"record_offset":15932,"parent_record_offset":15832,"ancestor_record_offset":13448,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1375,"record_offset":14748,"parent_record_offset":14224,"ancestor_record_offset":13448,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1375,"record_offset":15832,"parent_record_offset":15800,"ancestor_record_offset":13448,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1375,"record_offset":17360,"parent_record_offset":17328,"ancestor_record_offset":13448,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1375,"record_offset":13868,"parent_record_offset":13840,"ancestor_record_offset":13448,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1375,"record_offset":16468,"parent_record_offset":16436,"ancestor_record_offset":13448,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1375,"record_offset":14972,"parent_record_offset":14944,"ancestor_record_offset":13448,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1375,"record_offset":13728,"parent_record_offset":13700,"ancestor_record_offset":13448,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_and
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1375,"record_offset":14860,"parent_record_offset":14828,"ancestor_record_offset":13448,"inlinee":19100,"source_line":2728,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1375,"record_offset":13752,"parent_record_offset":13728,"ancestor_record_offset":13448,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_and
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1375,"record_offset":14884,"parent_record_offset":14860,"ancestor_record_offset":13448,"inlinee":19101,"source_line":3439,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1375,"record_offset":16492,"parent_record_offset":16468,"ancestor_record_offset":13448,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1375,"record_offset":13892,"parent_record_offset":13868,"ancestor_record_offset":13448,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1375,"record_offset":17576,"parent_record_offset":17516,"ancestor_record_offset":13448,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1375,"record_offset":17608,"parent_record_offset":17480,"ancestor_record_offset":13448,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::bitmap_unset
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1375,"record_offset":14828,"parent_record_offset":14224,"ancestor_record_offset":13448,"inlinee":45939,"source_line":588,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1375,"record_offset":17304,"parent_record_offset":17280,"ancestor_record_offset":13448,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1375,"record_offset":14920,"parent_record_offset":14224,"ancestor_record_offset":13448,"inlinee":5786,"source_line":479,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1375,"record_offset":16412,"parent_record_offset":16388,"ancestor_record_offset":13448,"inlinee":9025,"source_line":1667,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1375,"record_offset":13840,"parent_record_offset":13816,"ancestor_record_offset":13448,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1375,"record_offset":17328,"parent_record_offset":17304,"ancestor_record_offset":13448,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1375,"record_offset":14944,"parent_record_offset":14920,"ancestor_record_offset":13448,"inlinee":5787,"source_line":67,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1375,"record_offset":16436,"parent_record_offset":16412,"ancestor_record_offset":13448,"inlinee":5795,"source_line":94,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::impl$3::to_draw_multi_instanced<country_core::geometry::instanced_wall::BrickTransformSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 394
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1375,"record_offset":18012,"rva":16105376,"va":5384814496}
// 编译实例 RVAs: 0xf5bfa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12065237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384814496,"raw_hex":"e82be647ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5384814944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-13507630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384814496,"raw_hex":"e8d2e331ff","size":5,"soft_fail":false,"target_va":5371307104,"va":5384814729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18164455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384814496,"raw_hex":"e8e72a1501","size":5,"soft_fail":false,"target_va":5402979344,"va":5384814884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::iter_all_chunks
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1375,"record_offset":18240,"parent_record_offset":18012,"ancestor_record_offset":18012,"inlinee":63547,"source_line":277,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::to_draw_multi_instanced_impl
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1375,"record_offset":18380,"parent_record_offset":18012,"ancestor_record_offset":18012,"inlinee":63546,"source_line":305,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1375,"record_offset":19636,"parent_record_offset":19612,"ancestor_record_offset":18012,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1375,"record_offset":19612,"parent_record_offset":19512,"ancestor_record_offset":18012,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1375,"record_offset":19544,"parent_record_offset":19512,"ancestor_record_offset":18012,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1375,"record_offset":18544,"parent_record_offset":18520,"ancestor_record_offset":18012,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1375,"record_offset":19512,"parent_record_offset":19484,"ancestor_record_offset":18012,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":18520,"parent_record_offset":18484,"ancestor_record_offset":18012,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":18456,"parent_record_offset":18380,"ancestor_record_offset":18012,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":18484,"parent_record_offset":18380,"ancestor_record_offset":18012,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] bit_iter::impl$34::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bit-iter","version":"1.1.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bit-iter-1.1.1/src/lib.rs
//     evidence={"module":1375,"record_offset":18784,"parent_record_offset":18756,"ancestor_record_offset":18012,"inlinee":45942,"source_line":98,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1375,"record_offset":19204,"parent_record_offset":19024,"ancestor_record_offset":18012,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1375,"record_offset":18672,"parent_record_offset":18636,"ancestor_record_offset":18012,"inlinee":5074,"source_line":839,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$35::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1375,"record_offset":18636,"parent_record_offset":18608,"ancestor_record_offset":18012,"inlinee":5072,"source_line":605,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1375,"record_offset":18608,"parent_record_offset":18380,"ancestor_record_offset":18012,"inlinee":5070,"source_line":65,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1375,"record_offset":18972,"parent_record_offset":18948,"ancestor_record_offset":18012,"inlinee":5090,"source_line":358,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1375,"record_offset":18948,"parent_record_offset":18636,"ancestor_record_offset":18012,"inlinee":5088,"source_line":49,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1375,"record_offset":18756,"parent_record_offset":18728,"ancestor_record_offset":18012,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1375,"record_offset":18816,"parent_record_offset":18784,"ancestor_record_offset":18012,"inlinee":6317,"source_line":169,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1375,"record_offset":19576,"parent_record_offset":19544,"ancestor_record_offset":18012,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1375,"record_offset":18728,"parent_record_offset":18672,"ancestor_record_offset":18012,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1375,"record_offset":18876,"parent_record_offset":18852,"ancestor_record_offset":18012,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1375,"record_offset":18700,"parent_record_offset":18672,"ancestor_record_offset":18012,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1375,"record_offset":18852,"parent_record_offset":18756,"ancestor_record_offset":18012,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1375,"record_offset":19460,"parent_record_offset":18380,"ancestor_record_offset":18012,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1375,"record_offset":18336,"parent_record_offset":18312,"ancestor_record_offset":18012,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1375,"record_offset":19280,"parent_record_offset":19256,"ancestor_record_offset":18012,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1375,"record_offset":18576,"parent_record_offset":18484,"ancestor_record_offset":18012,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1375,"record_offset":18288,"parent_record_offset":18264,"ancestor_record_offset":18012,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1375,"record_offset":18312,"parent_record_offset":18288,"ancestor_record_offset":18012,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1375,"record_offset":19256,"parent_record_offset":19232,"ancestor_record_offset":18012,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1375,"record_offset":19232,"parent_record_offset":19204,"ancestor_record_offset":18012,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1375,"record_offset":19136,"parent_record_offset":19104,"ancestor_record_offset":18012,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1375,"record_offset":19160,"parent_record_offset":19136,"ancestor_record_offset":18012,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::iter_chunk_bitmap
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1375,"record_offset":18264,"parent_record_offset":18240,"ancestor_record_offset":18012,"inlinee":63548,"source_line":237,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::iter_chunk_bitmap::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1375,"record_offset":19104,"parent_record_offset":19076,"ancestor_record_offset":18012,"inlinee":63552,"source_line":241,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::iter_chunk_bitmap::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1375,"record_offset":18900,"parent_record_offset":18876,"ancestor_record_offset":18012,"inlinee":63550,"source_line":246,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::impl$4::drop
// 记录形式: inline_instance, inlinee_declaration; instances=26
// 源码行: 410
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x344A
// 首条位置: {"module":234,"inlinee":13386}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::Weak::upgrade
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":234,"record_offset":235664,"parent_record_offset":235604,"ancestor_record_offset":235604,"inlinee":11186,"source_line":2821,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":234,"record_offset":235980,"parent_record_offset":235604,"ancestor_record_offset":235604,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":234,"record_offset":236296,"parent_record_offset":235604,"ancestor_record_offset":235604,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] core::mem::take
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":234,"record_offset":236180,"parent_record_offset":235604,"ancestor_record_offset":235604,"inlinee":5326,"source_line":845,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=52
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":234,"record_offset":236420,"parent_record_offset":235604,"ancestor_record_offset":235604,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::AllocationHandle::is_null
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":500,"record_offset":211228,"parent_record_offset":211168,"ancestor_record_offset":211168,"inlinee":27768,"source_line":511,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":234,"record_offset":236008,"parent_record_offset":235604,"ancestor_record_offset":235604,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":234,"record_offset":236356,"parent_record_offset":236332,"ancestor_record_offset":235604,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::rc::is_dangling
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\rc.rs
//     evidence={"module":234,"record_offset":235716,"parent_record_offset":235692,"ancestor_record_offset":235604,"inlinee":4706,"source_line":2858,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":234,"record_offset":236656,"parent_record_offset":236620,"ancestor_record_offset":235604,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Weak::inner
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":234,"record_offset":235692,"parent_record_offset":235664,"ancestor_record_offset":235604,"inlinee":4704,"source_line":2896,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":234,"record_offset":236620,"parent_record_offset":236596,"ancestor_record_offset":235604,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$39::upgrade::checked_increment
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":234,"record_offset":235872,"parent_record_offset":235844,"ancestor_record_offset":235604,"inlinee":11189,"source_line":2826,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":234,"record_offset":236332,"parent_record_offset":236296,"ancestor_record_offset":235604,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":234,"record_offset":236204,"parent_record_offset":236180,"ancestor_record_offset":235604,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":234,"record_offset":235844,"parent_record_offset":235748,"ancestor_record_offset":235604,"inlinee":4401,"source_line":166,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":500,"record_offset":211248,"parent_record_offset":211228,"ancestor_record_offset":211168,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":500,"record_offset":211268,"parent_record_offset":211248,"ancestor_record_offset":211168,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":234,"record_offset":236688,"parent_record_offset":236656,"ancestor_record_offset":235604,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":234,"record_offset":236232,"parent_record_offset":236204,"ancestor_record_offset":235604,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":234,"record_offset":236260,"parent_record_offset":236204,"ancestor_record_offset":235604,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":234,"record_offset":236116,"parent_record_offset":236032,"ancestor_record_offset":235604,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":234,"record_offset":236140,"parent_record_offset":236116,"ancestor_record_offset":235604,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":234,"record_offset":236500,"parent_record_offset":236468,"ancestor_record_offset":235604,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":234,"record_offset":236060,"parent_record_offset":236032,"ancestor_record_offset":235604,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":234,"record_offset":235916,"parent_record_offset":235748,"ancestor_record_offset":235604,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_update
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":234,"record_offset":235748,"parent_record_offset":235664,"ancestor_record_offset":235604,"inlinee":11187,"source_line":2878,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":234,"record_offset":235788,"parent_record_offset":235748,"ancestor_record_offset":235604,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":234,"record_offset":236524,"parent_record_offset":236500,"ancestor_record_offset":235604,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":234,"record_offset":235940,"parent_record_offset":235916,"ancestor_record_offset":235604,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":234,"record_offset":235812,"parent_record_offset":235788,"ancestor_record_offset":235604,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":234,"record_offset":236716,"parent_record_offset":236656,"ancestor_record_offset":235604,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":234,"record_offset":236748,"parent_record_offset":236620,"ancestor_record_offset":235604,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=26
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":234,"record_offset":236444,"parent_record_offset":236420,"ancestor_record_offset":235604,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":234,"record_offset":236032,"parent_record_offset":236008,"ancestor_record_offset":235604,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=26
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":234,"record_offset":236468,"parent_record_offset":236444,"ancestor_record_offset":235604,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00234.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::impl$5::drop
// 记录形式: inline_instance, inlinee_declaration; instances=100
// 源码行: 435
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x397D
// 首条位置: {"module":266,"inlinee":14717}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=64
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":266,"record_offset":8640,"parent_record_offset":8616,"ancestor_record_offset":8616,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=64
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":266,"record_offset":8732,"parent_record_offset":8640,"ancestor_record_offset":8616,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=64
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":266,"record_offset":8676,"parent_record_offset":8640,"ancestor_record_offset":8616,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=64
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":266,"record_offset":8700,"parent_record_offset":8676,"ancestor_record_offset":8616,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00266.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::push<country_core::geometry::instanced_wall::BrickTransformSsbo>
// 记录形式: compiled_procedure; instances=4
// 源码行: 442
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1214,"record_offset":9596,"rva":13209856,"va":5381918976}
// 编译实例 RVAs: 0xc99100, 0xf5c180, 0x17e7370, 0x1adfd20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-10610832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381918976,"raw_hex":"e870175eff","size":5,"soft_fail":false,"target_va":5371308432,"va":5381919259,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=28
//     evidence={"assembly":"\tcallq\t21059800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381918976,"raw_hex":"e8d8584101","size":5,"soft_fail":false,"target_va":5402979344,"va":5381919539,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t21044403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381918976,"raw_hex":"e8b31c4101","size":5,"soft_fail":false,"target_va":5402963904,"va":5381919496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t21045814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381918976,"raw_hex":"e836224101","size":5,"soft_fail":false,"target_va":5402965344,"va":5381919525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1214,"record_offset":10112,"parent_record_offset":9596,"ancestor_record_offset":9596,"inlinee":11505,"source_line":2050,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1214,"record_offset":10244,"parent_record_offset":9596,"ancestor_record_offset":9596,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1214,"record_offset":10548,"parent_record_offset":9596,"ancestor_record_offset":9596,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1214,"record_offset":10216,"parent_record_offset":9596,"ancestor_record_offset":9596,"inlinee":8014,"source_line":450,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::bitmap_set
//     owner=private_source_ownership_candidate; count=12
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1214,"record_offset":9840,"parent_record_offset":9596,"ancestor_record_offset":9596,"inlinee":45947,"source_line":583,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1214,"record_offset":9932,"parent_record_offset":9596,"ancestor_record_offset":9596,"inlinee":5786,"source_line":479,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1214,"record_offset":10180,"parent_record_offset":10148,"ancestor_record_offset":9596,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1214,"record_offset":10276,"parent_record_offset":10244,"ancestor_record_offset":9596,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1214,"record_offset":10148,"parent_record_offset":10112,"ancestor_record_offset":9596,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1214,"record_offset":10332,"parent_record_offset":10244,"ancestor_record_offset":9596,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1214,"record_offset":10048,"parent_record_offset":9960,"ancestor_record_offset":9596,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1214,"record_offset":10072,"parent_record_offset":10048,"ancestor_record_offset":9596,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1214,"record_offset":10636,"parent_record_offset":10604,"ancestor_record_offset":9596,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1214,"record_offset":9992,"parent_record_offset":9960,"ancestor_record_offset":9596,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_or
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1214,"record_offset":9872,"parent_record_offset":9840,"ancestor_record_offset":9596,"inlinee":8935,"source_line":2794,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1214,"record_offset":10660,"parent_record_offset":10636,"ancestor_record_offset":9596,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1214,"record_offset":10016,"parent_record_offset":9992,"ancestor_record_offset":9596,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_or
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1214,"record_offset":9896,"parent_record_offset":9872,"ancestor_record_offset":9596,"inlinee":8936,"source_line":3471,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1214,"record_offset":10576,"parent_record_offset":10548,"ancestor_record_offset":9596,"inlinee":9025,"source_line":1667,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1214,"record_offset":9960,"parent_record_offset":9932,"ancestor_record_offset":9596,"inlinee":5787,"source_line":67,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1214,"record_offset":10604,"parent_record_offset":10576,"ancestor_record_offset":9596,"inlinee":5795,"source_line":94,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::push<country_core::resources::roof_tile_ssbo::RoofTileSsbo>
// 记录形式: compiled_procedure; instances=2
// 源码行: 442
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":751,"record_offset":9580,"rva":9285808,"va":5377994928}
// 编译实例 RVAs: 0x8db0b0, 0x17e7670
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6686758","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377994928,"raw_hex":"e8daf799ff","size":5,"soft_fail":false,"target_va":5371308432,"va":5377995185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=14
//     evidence={"assembly":"\tcallq\t24983896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377994928,"raw_hex":"e858397d01","size":5,"soft_fail":false,"target_va":5402979344,"va":5377995443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t24968499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377994928,"raw_hex":"e833fd7c01","size":5,"soft_fail":false,"target_va":5402963904,"va":5377995400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t24969910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377994928,"raw_hex":"e8b6027d01","size":5,"soft_fail":false,"target_va":5402965344,"va":5377995429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":10092,"parent_record_offset":9580,"ancestor_record_offset":9580,"inlinee":11505,"source_line":2050,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":10224,"parent_record_offset":9580,"ancestor_record_offset":9580,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":751,"record_offset":10528,"parent_record_offset":9580,"ancestor_record_offset":9580,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":751,"record_offset":10196,"parent_record_offset":9580,"ancestor_record_offset":9580,"inlinee":8014,"source_line":450,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::bitmap_set
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":751,"record_offset":9820,"parent_record_offset":9580,"ancestor_record_offset":9580,"inlinee":45947,"source_line":583,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":751,"record_offset":9912,"parent_record_offset":9580,"ancestor_record_offset":9580,"inlinee":5786,"source_line":479,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":751,"record_offset":10160,"parent_record_offset":10128,"ancestor_record_offset":9580,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":10256,"parent_record_offset":10224,"ancestor_record_offset":9580,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":10128,"parent_record_offset":10092,"ancestor_record_offset":9580,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":751,"record_offset":10312,"parent_record_offset":10224,"ancestor_record_offset":9580,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":751,"record_offset":10028,"parent_record_offset":9940,"ancestor_record_offset":9580,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":751,"record_offset":10052,"parent_record_offset":10028,"ancestor_record_offset":9580,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":10616,"parent_record_offset":10584,"ancestor_record_offset":9580,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":9972,"parent_record_offset":9940,"ancestor_record_offset":9580,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_or
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":9852,"parent_record_offset":9820,"ancestor_record_offset":9580,"inlinee":8935,"source_line":2794,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":10640,"parent_record_offset":10616,"ancestor_record_offset":9580,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":9996,"parent_record_offset":9972,"ancestor_record_offset":9580,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_or
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":9876,"parent_record_offset":9852,"ancestor_record_offset":9580,"inlinee":8936,"source_line":3471,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":751,"record_offset":10556,"parent_record_offset":10528,"ancestor_record_offset":9580,"inlinee":9025,"source_line":1667,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":751,"record_offset":9940,"parent_record_offset":9912,"ancestor_record_offset":9580,"inlinee":5787,"source_line":67,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":751,"record_offset":10584,"parent_record_offset":10556,"ancestor_record_offset":9580,"inlinee":5795,"source_line":94,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::push<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>
// 记录形式: compiled_procedure; instances=2
// 源码行: 442
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1214,"record_offset":10920,"rva":13210624,"va":5381919744}
// 编译实例 RVAs: 0xc99400, 0x198f400
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10611574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381919744,"raw_hex":"e88a145eff","size":5,"soft_fail":false,"target_va":5371308432,"va":5381920001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=14
//     evidence={"assembly":"\tcallq\t21059080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381919744,"raw_hex":"e808564101","size":5,"soft_fail":false,"target_va":5402979344,"va":5381920259,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t21043683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381919744,"raw_hex":"e8e3194101","size":5,"soft_fail":false,"target_va":5402963904,"va":5381920216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t21045094","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381919744,"raw_hex":"e8661f4101","size":5,"soft_fail":false,"target_va":5402965344,"va":5381920245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1214,"record_offset":11440,"parent_record_offset":10920,"ancestor_record_offset":10920,"inlinee":11505,"source_line":2050,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1214,"record_offset":11572,"parent_record_offset":10920,"ancestor_record_offset":10920,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1214,"record_offset":11876,"parent_record_offset":10920,"ancestor_record_offset":10920,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1214,"record_offset":11544,"parent_record_offset":10920,"ancestor_record_offset":10920,"inlinee":8014,"source_line":450,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::bitmap_set
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1214,"record_offset":11168,"parent_record_offset":10920,"ancestor_record_offset":10920,"inlinee":45947,"source_line":583,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1214,"record_offset":11260,"parent_record_offset":10920,"ancestor_record_offset":10920,"inlinee":5786,"source_line":479,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1214,"record_offset":11508,"parent_record_offset":11476,"ancestor_record_offset":10920,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1214,"record_offset":11604,"parent_record_offset":11572,"ancestor_record_offset":10920,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1214,"record_offset":11476,"parent_record_offset":11440,"ancestor_record_offset":10920,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1214,"record_offset":11660,"parent_record_offset":11572,"ancestor_record_offset":10920,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1214,"record_offset":11376,"parent_record_offset":11288,"ancestor_record_offset":10920,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1214,"record_offset":11400,"parent_record_offset":11376,"ancestor_record_offset":10920,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1214,"record_offset":11964,"parent_record_offset":11932,"ancestor_record_offset":10920,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1214,"record_offset":11320,"parent_record_offset":11288,"ancestor_record_offset":10920,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_or
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1214,"record_offset":11200,"parent_record_offset":11168,"ancestor_record_offset":10920,"inlinee":8935,"source_line":2794,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1214,"record_offset":11988,"parent_record_offset":11964,"ancestor_record_offset":10920,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1214,"record_offset":11344,"parent_record_offset":11320,"ancestor_record_offset":10920,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_or
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1214,"record_offset":11224,"parent_record_offset":11200,"ancestor_record_offset":10920,"inlinee":8936,"source_line":3471,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1214,"record_offset":11904,"parent_record_offset":11876,"ancestor_record_offset":10920,"inlinee":9025,"source_line":1667,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1214,"record_offset":11288,"parent_record_offset":11260,"ancestor_record_offset":10920,"inlinee":5787,"source_line":67,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1214,"record_offset":11932,"parent_record_offset":11904,"ancestor_record_offset":10920,"inlinee":5795,"source_line":94,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::clear<country_core::geometry::instanced_wall::BrickTransformSsbo>
// 记录形式: compiled_procedure; instances=3
// 源码行: 487
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1214,"record_offset":12248,"rva":13211344,"va":5381920464}
// 编译实例 RVAs: 0xc996d0, 0xf5c480, 0x1ae0020
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d7990
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8932633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384815744,"raw_hex":"e8e7b277ff","size":5,"soft_fail":false,"target_va":5375883664,"va":5384816292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409b0b70
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18020057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396889632,"raw_hex":"e82709edfe","size":5,"soft_fail":false,"target_va":5378870128,"va":5396890180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c97090
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381920464,"raw_hex":"e897d7ffff","size":5,"soft_fail":false,"target_va":5381910672,"va":5381921012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t20997807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381920464,"raw_hex":"e8af664001","size":5,"soft_fail":false,"target_va":5402918528,"va":5381920716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t21057747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381920464,"raw_hex":"e8d3504101","size":5,"soft_fail":false,"target_va":5402978880,"va":5381921128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t21058241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381920464,"raw_hex":"e8c1524101","size":5,"soft_fail":false,"target_va":5402979344,"va":5381921098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t21042841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381920464,"raw_hex":"e899164101","size":5,"soft_fail":false,"target_va":5402963904,"va":5381921058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t21044130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381920464,"raw_hex":"e8a21b4101","size":5,"soft_fail":false,"target_va":5402965344,"va":5381921209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tjmp\t21044253","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5381920464,"raw_hex":"e91d1c4101","size":5,"soft_fail":false,"target_va":5402965344,"va":5381921086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1214,"record_offset":13032,"parent_record_offset":12248,"ancestor_record_offset":12248,"inlinee":5990,"source_line":2129,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1214,"record_offset":12632,"parent_record_offset":12248,"ancestor_record_offset":12248,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1214,"record_offset":13324,"parent_record_offset":12248,"ancestor_record_offset":12248,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::rev
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1214,"record_offset":13268,"parent_record_offset":12248,"ancestor_record_offset":12248,"inlinee":4519,"source_line":3348,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1214,"record_offset":12512,"parent_record_offset":12248,"ancestor_record_offset":12248,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1214,"record_offset":12568,"parent_record_offset":12248,"ancestor_record_offset":12248,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1214,"record_offset":14340,"parent_record_offset":12248,"ancestor_record_offset":12248,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1214,"record_offset":12604,"parent_record_offset":12248,"ancestor_record_offset":12248,"inlinee":8014,"source_line":450,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1214,"record_offset":12660,"parent_record_offset":12248,"ancestor_record_offset":12248,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::bitmap_unset
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1214,"record_offset":12736,"parent_record_offset":12248,"ancestor_record_offset":12248,"inlinee":45939,"source_line":588,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1214,"record_offset":12824,"parent_record_offset":12248,"ancestor_record_offset":12248,"inlinee":5786,"source_line":479,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1214,"record_offset":13524,"parent_record_offset":13496,"ancestor_record_offset":12248,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1214,"record_offset":13496,"parent_record_offset":13464,"ancestor_record_offset":12248,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1214,"record_offset":13092,"parent_record_offset":13068,"ancestor_record_offset":12248,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1214,"record_offset":13464,"parent_record_offset":13440,"ancestor_record_offset":12248,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1214,"record_offset":13604,"parent_record_offset":13396,"ancestor_record_offset":12248,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1214,"record_offset":13068,"parent_record_offset":13032,"ancestor_record_offset":12248,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1214,"record_offset":13396,"parent_record_offset":13360,"ancestor_record_offset":12248,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1214,"record_offset":13124,"parent_record_offset":13032,"ancestor_record_offset":12248,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1214,"record_offset":13440,"parent_record_offset":13396,"ancestor_record_offset":12248,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1214,"record_offset":13152,"parent_record_offset":13032,"ancestor_record_offset":12248,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$6::next_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":1214,"record_offset":13752,"parent_record_offset":13708,"ancestor_record_offset":12248,"inlinee":45740,"source_line":168,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$6::next_back::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":1214,"record_offset":14004,"parent_record_offset":13976,"ancestor_record_offset":12248,"inlinee":45742,"source_line":169,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1214,"record_offset":14104,"parent_record_offset":14076,"ancestor_record_offset":12248,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1214,"record_offset":14168,"parent_record_offset":14104,"ancestor_record_offset":12248,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1214,"record_offset":13576,"parent_record_offset":13396,"ancestor_record_offset":12248,"inlinee":5454,"source_line":13,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1214,"record_offset":14276,"parent_record_offset":14252,"ancestor_record_offset":12248,"inlinee":5437,"source_line":30,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1214,"record_offset":13360,"parent_record_offset":13324,"ancestor_record_offset":12248,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\rev.rs
//     evidence={"module":1214,"record_offset":13292,"parent_record_offset":13268,"ancestor_record_offset":12248,"inlinee":4521,"source_line":20,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::impl$1::fold
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\rev.rs
//     evidence={"module":1214,"record_offset":13684,"parent_record_offset":13660,"ancestor_record_offset":12248,"inlinee":21622,"source_line":60,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::double_ended::DoubleEndedIterator::rfold
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\double_ended.rs
//     evidence={"module":1214,"record_offset":13708,"parent_record_offset":13684,"ancestor_record_offset":12248,"inlinee":21624,"source_line":300,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1214,"record_offset":13660,"parent_record_offset":13396,"ancestor_record_offset":12248,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1214,"record_offset":14076,"parent_record_offset":13708,"ancestor_record_offset":12248,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1214,"record_offset":12536,"parent_record_offset":12512,"ancestor_record_offset":12248,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1214,"record_offset":13976,"parent_record_offset":13752,"ancestor_record_offset":12248,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1214,"record_offset":14204,"parent_record_offset":13708,"ancestor_record_offset":12248,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1214,"record_offset":13228,"parent_record_offset":13204,"ancestor_record_offset":12248,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1214,"record_offset":13936,"parent_record_offset":13912,"ancestor_record_offset":12248,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1214,"record_offset":13912,"parent_record_offset":13884,"ancestor_record_offset":12248,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1214,"record_offset":12700,"parent_record_offset":12660,"ancestor_record_offset":12248,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1214,"record_offset":14032,"parent_record_offset":14004,"ancestor_record_offset":12248,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1214,"record_offset":14136,"parent_record_offset":14104,"ancestor_record_offset":12248,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1214,"record_offset":12940,"parent_record_offset":12852,"ancestor_record_offset":12248,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1214,"record_offset":12964,"parent_record_offset":12940,"ancestor_record_offset":12248,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1214,"record_offset":13180,"parent_record_offset":13032,"ancestor_record_offset":12248,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1214,"record_offset":13204,"parent_record_offset":13180,"ancestor_record_offset":12248,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1214,"record_offset":13884,"parent_record_offset":13784,"ancestor_record_offset":12248,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1214,"record_offset":13784,"parent_record_offset":13752,"ancestor_record_offset":12248,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1214,"record_offset":14432,"parent_record_offset":14396,"ancestor_record_offset":12248,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1214,"record_offset":12884,"parent_record_offset":12852,"ancestor_record_offset":12248,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_and
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1214,"record_offset":12764,"parent_record_offset":12736,"ancestor_record_offset":12248,"inlinee":19100,"source_line":2728,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_and
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1214,"record_offset":12788,"parent_record_offset":12764,"ancestor_record_offset":12248,"inlinee":19101,"source_line":3439,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1214,"record_offset":14456,"parent_record_offset":14432,"ancestor_record_offset":12248,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1214,"record_offset":12908,"parent_record_offset":12884,"ancestor_record_offset":12248,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1214,"record_offset":14368,"parent_record_offset":14340,"ancestor_record_offset":12248,"inlinee":9025,"source_line":1667,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1214,"record_offset":12852,"parent_record_offset":12824,"ancestor_record_offset":12248,"inlinee":5787,"source_line":67,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1214,"record_offset":14396,"parent_record_offset":14368,"ancestor_record_offset":12248,"inlinee":5795,"source_line":94,"archive":"_pdb/A/modules/01214.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::clear<country_core::resources::roof_tile_ssbo::RoofTileSsbo>
// 记录形式: compiled_procedure; instances=2
// 源码行: 487
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":751,"record_offset":10900,"rva":9286528,"va":5377995648}
// 编译实例 RVAs: 0x8db380, 0x17e7940
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408cc980
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-60457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377995648,"raw_hex":"e8d713ffff","size":5,"soft_fail":false,"target_va":5377935744,"va":5377996196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d00f30
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11430969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393774912,"raw_hex":"e8c79351ff","size":5,"soft_fail":false,"target_va":5382344496,"va":5393775460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t24922623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377995648,"raw_hex":"e8ff497c01","size":5,"soft_fail":false,"target_va":5402918528,"va":5377995900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t24982563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377995648,"raw_hex":"e823347d01","size":5,"soft_fail":false,"target_va":5402978880,"va":5377996312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t24983057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377995648,"raw_hex":"e811367d01","size":5,"soft_fail":false,"target_va":5402979344,"va":5377996282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t24967657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377995648,"raw_hex":"e8e9f97c01","size":5,"soft_fail":false,"target_va":5402963904,"va":5377996242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t24968946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377995648,"raw_hex":"e8f2fe7c01","size":5,"soft_fail":false,"target_va":5402965344,"va":5377996393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tjmp\t24969069","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5377995648,"raw_hex":"e96dff7c01","size":5,"soft_fail":false,"target_va":5402965344,"va":5377996270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":11680,"parent_record_offset":10900,"ancestor_record_offset":10900,"inlinee":5990,"source_line":2129,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":11280,"parent_record_offset":10900,"ancestor_record_offset":10900,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":11972,"parent_record_offset":10900,"ancestor_record_offset":10900,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::rev
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":751,"record_offset":11916,"parent_record_offset":10900,"ancestor_record_offset":10900,"inlinee":4519,"source_line":3348,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":751,"record_offset":11160,"parent_record_offset":10900,"ancestor_record_offset":10900,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":751,"record_offset":11216,"parent_record_offset":10900,"ancestor_record_offset":10900,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":751,"record_offset":12988,"parent_record_offset":10900,"ancestor_record_offset":10900,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":751,"record_offset":11252,"parent_record_offset":10900,"ancestor_record_offset":10900,"inlinee":8014,"source_line":450,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":751,"record_offset":11308,"parent_record_offset":10900,"ancestor_record_offset":10900,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::bitmap_unset
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":751,"record_offset":11384,"parent_record_offset":10900,"ancestor_record_offset":10900,"inlinee":45939,"source_line":588,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":751,"record_offset":11472,"parent_record_offset":10900,"ancestor_record_offset":10900,"inlinee":5786,"source_line":479,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":751,"record_offset":12172,"parent_record_offset":12144,"ancestor_record_offset":10900,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":751,"record_offset":12144,"parent_record_offset":12112,"ancestor_record_offset":10900,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":751,"record_offset":11740,"parent_record_offset":11716,"ancestor_record_offset":10900,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":751,"record_offset":12112,"parent_record_offset":12088,"ancestor_record_offset":10900,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":12252,"parent_record_offset":12044,"ancestor_record_offset":10900,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":11716,"parent_record_offset":11680,"ancestor_record_offset":10900,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":12044,"parent_record_offset":12008,"ancestor_record_offset":10900,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":11772,"parent_record_offset":11680,"ancestor_record_offset":10900,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":12088,"parent_record_offset":12044,"ancestor_record_offset":10900,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":11800,"parent_record_offset":11680,"ancestor_record_offset":10900,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":751,"record_offset":12400,"parent_record_offset":12356,"ancestor_record_offset":10900,"inlinee":45740,"source_line":168,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$6::next_back::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":751,"record_offset":12652,"parent_record_offset":12624,"ancestor_record_offset":10900,"inlinee":45742,"source_line":169,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":751,"record_offset":12752,"parent_record_offset":12724,"ancestor_record_offset":10900,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":751,"record_offset":12816,"parent_record_offset":12752,"ancestor_record_offset":10900,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":751,"record_offset":12224,"parent_record_offset":12044,"ancestor_record_offset":10900,"inlinee":5454,"source_line":13,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":751,"record_offset":12924,"parent_record_offset":12900,"ancestor_record_offset":10900,"inlinee":5437,"source_line":30,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":751,"record_offset":12008,"parent_record_offset":11972,"ancestor_record_offset":10900,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\rev.rs
//     evidence={"module":751,"record_offset":11940,"parent_record_offset":11916,"ancestor_record_offset":10900,"inlinee":4521,"source_line":20,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::impl$1::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\rev.rs
//     evidence={"module":751,"record_offset":12332,"parent_record_offset":12308,"ancestor_record_offset":10900,"inlinee":21622,"source_line":60,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::double_ended::DoubleEndedIterator::rfold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\double_ended.rs
//     evidence={"module":751,"record_offset":12356,"parent_record_offset":12332,"ancestor_record_offset":10900,"inlinee":21624,"source_line":300,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":751,"record_offset":12308,"parent_record_offset":12044,"ancestor_record_offset":10900,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":751,"record_offset":12724,"parent_record_offset":12356,"ancestor_record_offset":10900,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":751,"record_offset":11184,"parent_record_offset":11160,"ancestor_record_offset":10900,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":751,"record_offset":12624,"parent_record_offset":12400,"ancestor_record_offset":10900,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":751,"record_offset":12852,"parent_record_offset":12356,"ancestor_record_offset":10900,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":751,"record_offset":11876,"parent_record_offset":11852,"ancestor_record_offset":10900,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":751,"record_offset":12584,"parent_record_offset":12560,"ancestor_record_offset":10900,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":751,"record_offset":12560,"parent_record_offset":12532,"ancestor_record_offset":10900,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":751,"record_offset":11348,"parent_record_offset":11308,"ancestor_record_offset":10900,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":751,"record_offset":12680,"parent_record_offset":12652,"ancestor_record_offset":10900,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":751,"record_offset":12784,"parent_record_offset":12752,"ancestor_record_offset":10900,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":751,"record_offset":11588,"parent_record_offset":11500,"ancestor_record_offset":10900,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":751,"record_offset":11612,"parent_record_offset":11588,"ancestor_record_offset":10900,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":751,"record_offset":11828,"parent_record_offset":11680,"ancestor_record_offset":10900,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":751,"record_offset":11852,"parent_record_offset":11828,"ancestor_record_offset":10900,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":751,"record_offset":12532,"parent_record_offset":12432,"ancestor_record_offset":10900,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":751,"record_offset":12432,"parent_record_offset":12400,"ancestor_record_offset":10900,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":13080,"parent_record_offset":13044,"ancestor_record_offset":10900,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":11532,"parent_record_offset":11500,"ancestor_record_offset":10900,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_and
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":11412,"parent_record_offset":11384,"ancestor_record_offset":10900,"inlinee":19100,"source_line":2728,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_and
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":11436,"parent_record_offset":11412,"ancestor_record_offset":10900,"inlinee":19101,"source_line":3439,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":13104,"parent_record_offset":13080,"ancestor_record_offset":10900,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":11556,"parent_record_offset":11532,"ancestor_record_offset":10900,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":751,"record_offset":13016,"parent_record_offset":12988,"ancestor_record_offset":10900,"inlinee":9025,"source_line":1667,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":751,"record_offset":11500,"parent_record_offset":11472,"ancestor_record_offset":10900,"inlinee":5787,"source_line":67,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":751,"record_offset":13044,"parent_record_offset":13016,"ancestor_record_offset":10900,"inlinee":5795,"source_line":94,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::clear<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 487
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3214,"record_offset":1576,"rva":26801872,"va":5395510992}
// 编译实例 RVAs: 0x198f6d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1412e6260
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6985369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395510992,"raw_hex":"e8676995ff","size":5,"soft_fail":false,"target_va":5388526176,"va":5395511540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7407279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395510992,"raw_hex":"e8af067100","size":5,"soft_fail":false,"target_va":5402918528,"va":5395511244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7467219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395510992,"raw_hex":"e8d3f07100","size":5,"soft_fail":false,"target_va":5402978880,"va":5395511656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7467713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395510992,"raw_hex":"e8c1f27100","size":5,"soft_fail":false,"target_va":5402979344,"va":5395511626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7452313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395510992,"raw_hex":"e899b67100","size":5,"soft_fail":false,"target_va":5402963904,"va":5395511586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7453602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395510992,"raw_hex":"e8a2bb7100","size":5,"soft_fail":false,"target_va":5402965344,"va":5395511737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t7453725","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5395510992,"raw_hex":"e91dbc7100","size":5,"soft_fail":false,"target_va":5402965344,"va":5395511614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3214,"record_offset":2364,"parent_record_offset":1576,"ancestor_record_offset":1576,"inlinee":5990,"source_line":2129,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3214,"record_offset":1964,"parent_record_offset":1576,"ancestor_record_offset":1576,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3214,"record_offset":2656,"parent_record_offset":1576,"ancestor_record_offset":1576,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::rev
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3214,"record_offset":2600,"parent_record_offset":1576,"ancestor_record_offset":1576,"inlinee":4519,"source_line":3348,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3214,"record_offset":1844,"parent_record_offset":1576,"ancestor_record_offset":1576,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3214,"record_offset":1900,"parent_record_offset":1576,"ancestor_record_offset":1576,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3214,"record_offset":3672,"parent_record_offset":1576,"ancestor_record_offset":1576,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3214,"record_offset":1936,"parent_record_offset":1576,"ancestor_record_offset":1576,"inlinee":8014,"source_line":450,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3214,"record_offset":1992,"parent_record_offset":1576,"ancestor_record_offset":1576,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::bitmap_unset
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":3214,"record_offset":2068,"parent_record_offset":1576,"ancestor_record_offset":1576,"inlinee":45939,"source_line":588,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":3214,"record_offset":2156,"parent_record_offset":1576,"ancestor_record_offset":1576,"inlinee":5786,"source_line":479,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3214,"record_offset":2856,"parent_record_offset":2828,"ancestor_record_offset":1576,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3214,"record_offset":2828,"parent_record_offset":2796,"ancestor_record_offset":1576,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3214,"record_offset":2424,"parent_record_offset":2400,"ancestor_record_offset":1576,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3214,"record_offset":2796,"parent_record_offset":2772,"ancestor_record_offset":1576,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3214,"record_offset":2936,"parent_record_offset":2728,"ancestor_record_offset":1576,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3214,"record_offset":2400,"parent_record_offset":2364,"ancestor_record_offset":1576,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3214,"record_offset":2728,"parent_record_offset":2692,"ancestor_record_offset":1576,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3214,"record_offset":2456,"parent_record_offset":2364,"ancestor_record_offset":1576,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3214,"record_offset":2772,"parent_record_offset":2728,"ancestor_record_offset":1576,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3214,"record_offset":2484,"parent_record_offset":2364,"ancestor_record_offset":1576,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":3214,"record_offset":3084,"parent_record_offset":3040,"ancestor_record_offset":1576,"inlinee":45740,"source_line":168,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$6::next_back::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":3214,"record_offset":3336,"parent_record_offset":3308,"ancestor_record_offset":1576,"inlinee":45742,"source_line":169,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3214,"record_offset":3436,"parent_record_offset":3408,"ancestor_record_offset":1576,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":3214,"record_offset":3500,"parent_record_offset":3436,"ancestor_record_offset":1576,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":3214,"record_offset":2908,"parent_record_offset":2728,"ancestor_record_offset":1576,"inlinee":5454,"source_line":13,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":3214,"record_offset":3608,"parent_record_offset":3584,"ancestor_record_offset":1576,"inlinee":5437,"source_line":30,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3214,"record_offset":2692,"parent_record_offset":2656,"ancestor_record_offset":1576,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\rev.rs
//     evidence={"module":3214,"record_offset":2624,"parent_record_offset":2600,"ancestor_record_offset":1576,"inlinee":4521,"source_line":20,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\rev.rs
//     evidence={"module":3214,"record_offset":3016,"parent_record_offset":2992,"ancestor_record_offset":1576,"inlinee":21622,"source_line":60,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::double_ended::DoubleEndedIterator::rfold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\double_ended.rs
//     evidence={"module":3214,"record_offset":3040,"parent_record_offset":3016,"ancestor_record_offset":1576,"inlinee":21624,"source_line":300,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3214,"record_offset":2992,"parent_record_offset":2728,"ancestor_record_offset":1576,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3214,"record_offset":3408,"parent_record_offset":3040,"ancestor_record_offset":1576,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3214,"record_offset":1868,"parent_record_offset":1844,"ancestor_record_offset":1576,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3214,"record_offset":3308,"parent_record_offset":3084,"ancestor_record_offset":1576,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3214,"record_offset":3536,"parent_record_offset":3040,"ancestor_record_offset":1576,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3214,"record_offset":2560,"parent_record_offset":2536,"ancestor_record_offset":1576,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3214,"record_offset":3268,"parent_record_offset":3244,"ancestor_record_offset":1576,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3214,"record_offset":3244,"parent_record_offset":3216,"ancestor_record_offset":1576,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3214,"record_offset":2032,"parent_record_offset":1992,"ancestor_record_offset":1576,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3214,"record_offset":3364,"parent_record_offset":3336,"ancestor_record_offset":1576,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3214,"record_offset":3468,"parent_record_offset":3436,"ancestor_record_offset":1576,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3214,"record_offset":2272,"parent_record_offset":2184,"ancestor_record_offset":1576,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3214,"record_offset":2296,"parent_record_offset":2272,"ancestor_record_offset":1576,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3214,"record_offset":2512,"parent_record_offset":2364,"ancestor_record_offset":1576,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3214,"record_offset":2536,"parent_record_offset":2512,"ancestor_record_offset":1576,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3214,"record_offset":3216,"parent_record_offset":3116,"ancestor_record_offset":1576,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3214,"record_offset":3116,"parent_record_offset":3084,"ancestor_record_offset":1576,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3214,"record_offset":3764,"parent_record_offset":3728,"ancestor_record_offset":1576,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3214,"record_offset":2216,"parent_record_offset":2184,"ancestor_record_offset":1576,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_and
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3214,"record_offset":2096,"parent_record_offset":2068,"ancestor_record_offset":1576,"inlinee":19100,"source_line":2728,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_and
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3214,"record_offset":2120,"parent_record_offset":2096,"ancestor_record_offset":1576,"inlinee":19101,"source_line":3439,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3214,"record_offset":3788,"parent_record_offset":3764,"ancestor_record_offset":1576,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3214,"record_offset":2240,"parent_record_offset":2216,"ancestor_record_offset":1576,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":3214,"record_offset":3700,"parent_record_offset":3672,"ancestor_record_offset":1576,"inlinee":9025,"source_line":1667,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":3214,"record_offset":2184,"parent_record_offset":2156,"ancestor_record_offset":1576,"inlinee":5787,"source_line":67,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":3214,"record_offset":3728,"parent_record_offset":3700,"ancestor_record_offset":1576,"inlinee":5795,"source_line":94,"archive":"_pdb/A/modules/03214.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::AllocationHandle::is_null
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 511, 512
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6C78
// 首条位置: {"module":494,"record_offset":120188,"rva":7078928,"va":5375788048}
// 编译实例 RVAs: 0x6c0410
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":494,"record_offset":120344,"parent_record_offset":120188,"ancestor_record_offset":120188,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/00494.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":494,"record_offset":120364,"parent_record_offset":120344,"ancestor_record_offset":120188,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00494.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::impl$10::new_storage_buffer<country_core::geometry::instanced_wall::BrickTransformSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 528
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1375,"record_offset":23480,"rva":16107376,"va":5384816496}
// 编译实例 RVAs: 0xf5c770
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12066888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384816496,"raw_hex":"e8b8df47ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5384816579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12068044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384816496,"raw_hex":"e834db47ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5384817751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8577845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384816496,"raw_hex":"e8cb1c7dff","size":5,"soft_fail":false,"target_va":5376239920,"va":5384817760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f838c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t159291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384816496,"raw_hex":"e83b6e0200","size":5,"soft_fail":false,"target_va":5384976576,"va":5384817280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fffa50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t667575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384816496,"raw_hex":"e8b72f0a00","size":5,"soft_fail":false,"target_va":5385484880,"va":5384817300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18776780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384816496,"raw_hex":"e8cc821e01","size":5,"soft_fail":false,"target_va":5403593392,"va":5384816607,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t18160547","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384816496,"raw_hex":"e8a31b1501","size":5,"soft_fail":false,"target_va":5402978144,"va":5384817592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18161570","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384816496,"raw_hex":"e8a21f1501","size":5,"soft_fail":false,"target_va":5402979152,"va":5384817577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1375,"record_offset":23704,"parent_record_offset":23480,"ancestor_record_offset":23480,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1375,"record_offset":23760,"parent_record_offset":23480,"ancestor_record_offset":23480,"inlinee":63553,"source_line":543,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1375,"record_offset":24016,"parent_record_offset":23992,"ancestor_record_offset":23480,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1375,"record_offset":24040,"parent_record_offset":24016,"ancestor_record_offset":23480,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1375,"record_offset":26112,"parent_record_offset":26088,"ancestor_record_offset":23480,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1375,"record_offset":25876,"parent_record_offset":25848,"ancestor_record_offset":23480,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1375,"record_offset":23992,"parent_record_offset":23964,"ancestor_record_offset":23480,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1375,"record_offset":26088,"parent_record_offset":26064,"ancestor_record_offset":23480,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1375,"record_offset":25848,"parent_record_offset":25820,"ancestor_record_offset":23480,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1375,"record_offset":26064,"parent_record_offset":26040,"ancestor_record_offset":23480,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1375,"record_offset":23964,"parent_record_offset":23936,"ancestor_record_offset":23480,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1375,"record_offset":23936,"parent_record_offset":23908,"ancestor_record_offset":23480,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1375,"record_offset":24108,"parent_record_offset":23936,"ancestor_record_offset":23480,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1375,"record_offset":26360,"parent_record_offset":26336,"ancestor_record_offset":23480,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1375,"record_offset":25820,"parent_record_offset":25792,"ancestor_record_offset":23480,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$52::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1375,"record_offset":25792,"parent_record_offset":23760,"ancestor_record_offset":23480,"inlinee":5311,"source_line":3285,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":25252,"parent_record_offset":25228,"ancestor_record_offset":23480,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":24144,"parent_record_offset":23872,"ancestor_record_offset":23480,"inlinee":4231,"source_line":2627,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":24956,"parent_record_offset":24920,"ancestor_record_offset":23480,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":23908,"parent_record_offset":23872,"ancestor_record_offset":23480,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":23844,"parent_record_offset":23760,"ancestor_record_offset":23480,"inlinee":4220,"source_line":2685,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":24856,"parent_record_offset":24824,"ancestor_record_offset":23480,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":25448,"parent_record_offset":25416,"ancestor_record_offset":23480,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1375,"record_offset":25228,"parent_record_offset":24920,"ancestor_record_offset":23480,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$0::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1375,"record_offset":23872,"parent_record_offset":23844,"ancestor_record_offset":23480,"inlinee":4222,"source_line":14,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1375,"record_offset":24888,"parent_record_offset":24856,"ancestor_record_offset":23480,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1375,"record_offset":24920,"parent_record_offset":24888,"ancestor_record_offset":23480,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1375,"record_offset":26624,"parent_record_offset":26596,"ancestor_record_offset":23480,"inlinee":4115,"source_line":199,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$49::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\mod.rs
//     evidence={"module":1375,"record_offset":24260,"parent_record_offset":24236,"ancestor_record_offset":23480,"inlinee":25444,"source_line":439,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$57::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\mod.rs
//     evidence={"module":1375,"record_offset":24292,"parent_record_offset":24208,"ancestor_record_offset":23480,"inlinee":11220,"source_line":439,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1375,"record_offset":24748,"parent_record_offset":24724,"ancestor_record_offset":23480,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1375,"record_offset":25300,"parent_record_offset":25276,"ancestor_record_offset":23480,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1375,"record_offset":25352,"parent_record_offset":25324,"ancestor_record_offset":23480,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1375,"record_offset":25576,"parent_record_offset":25552,"ancestor_record_offset":23480,"inlinee":4620,"source_line":207,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1375,"record_offset":24724,"parent_record_offset":24700,"ancestor_record_offset":23480,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1375,"record_offset":24700,"parent_record_offset":24632,"ancestor_record_offset":23480,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1375,"record_offset":24824,"parent_record_offset":23760,"ancestor_record_offset":23480,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1375,"record_offset":25324,"parent_record_offset":25300,"ancestor_record_offset":23480,"inlinee":5097,"source_line":2580,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1375,"record_offset":25276,"parent_record_offset":25252,"ancestor_record_offset":23480,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1375,"record_offset":25416,"parent_record_offset":25352,"ancestor_record_offset":23480,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1375,"record_offset":26648,"parent_record_offset":26624,"ancestor_record_offset":23480,"inlinee":4117,"source_line":392,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1375,"record_offset":25600,"parent_record_offset":25576,"ancestor_record_offset":23480,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1375,"record_offset":23728,"parent_record_offset":23704,"ancestor_record_offset":23480,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1375,"record_offset":26040,"parent_record_offset":23760,"ancestor_record_offset":23480,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1375,"record_offset":24064,"parent_record_offset":24040,"ancestor_record_offset":23480,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1375,"record_offset":24168,"parent_record_offset":24144,"ancestor_record_offset":23480,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] country_core::geometry::instanced_wall::impl$34::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/geometry/instanced_wall.rs
//     evidence={"module":1375,"record_offset":24208,"parent_record_offset":23760,"ancestor_record_offset":23480,"inlinee":63555,"source_line":38,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$11::new::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1375,"record_offset":25680,"parent_record_offset":23760,"ancestor_record_offset":23480,"inlinee":63558,"source_line":556,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$11::new::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1375,"record_offset":25376,"parent_record_offset":25352,"ancestor_record_offset":23480,"inlinee":61493,"source_line":563,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::render::impl$4::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/render/mod.rs
//     evidence={"module":1375,"record_offset":24236,"parent_record_offset":24208,"ancestor_record_offset":23480,"inlinee":63557,"source_line":8,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::impl$10::new_storage_buffer<country_core::resources::roof_tile_ssbo::RoofTileSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 528
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1375,"record_offset":27316,"rva":16109456,"va":5384818576}
// 编译实例 RVAs: 0xf5cf90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12068956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384818576,"raw_hex":"e8a4d747ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5384818647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12069998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384818576,"raw_hex":"e892d347ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5384819705,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8579799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384818576,"raw_hex":"e829157dff","size":5,"soft_fail":false,"target_va":5376239920,"va":5384819714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f82a40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t153593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384818576,"raw_hex":"e8f9570200","size":5,"soft_fail":false,"target_va":5384972864,"va":5384819266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fffa50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t665589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384818576,"raw_hex":"e8f5270a00","size":5,"soft_fail":false,"target_va":5385484880,"va":5384819286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18774712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384818576,"raw_hex":"e8b87a1e01","size":5,"soft_fail":false,"target_va":5403593392,"va":5384818675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t18158571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384818576,"raw_hex":"e8eb131501","size":5,"soft_fail":false,"target_va":5402978144,"va":5384819568,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18159594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384818576,"raw_hex":"e8ea171501","size":5,"soft_fail":false,"target_va":5402979152,"va":5384819553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1375,"record_offset":27532,"parent_record_offset":27316,"ancestor_record_offset":27316,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1375,"record_offset":27588,"parent_record_offset":27316,"ancestor_record_offset":27316,"inlinee":63553,"source_line":543,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1375,"record_offset":27844,"parent_record_offset":27820,"ancestor_record_offset":27316,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1375,"record_offset":27868,"parent_record_offset":27844,"ancestor_record_offset":27316,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1375,"record_offset":29800,"parent_record_offset":29776,"ancestor_record_offset":27316,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1375,"record_offset":29564,"parent_record_offset":29536,"ancestor_record_offset":27316,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1375,"record_offset":27820,"parent_record_offset":27792,"ancestor_record_offset":27316,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1375,"record_offset":29776,"parent_record_offset":29752,"ancestor_record_offset":27316,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1375,"record_offset":29536,"parent_record_offset":29508,"ancestor_record_offset":27316,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1375,"record_offset":29752,"parent_record_offset":29728,"ancestor_record_offset":27316,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1375,"record_offset":27792,"parent_record_offset":27764,"ancestor_record_offset":27316,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1375,"record_offset":27764,"parent_record_offset":27736,"ancestor_record_offset":27316,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1375,"record_offset":27936,"parent_record_offset":27764,"ancestor_record_offset":27316,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1375,"record_offset":30048,"parent_record_offset":30024,"ancestor_record_offset":27316,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1375,"record_offset":29508,"parent_record_offset":29480,"ancestor_record_offset":27316,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$52::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1375,"record_offset":29480,"parent_record_offset":27588,"ancestor_record_offset":27316,"inlinee":5311,"source_line":3285,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":28956,"parent_record_offset":28932,"ancestor_record_offset":27316,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":27972,"parent_record_offset":27700,"ancestor_record_offset":27316,"inlinee":4231,"source_line":2627,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":28660,"parent_record_offset":28624,"ancestor_record_offset":27316,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":27736,"parent_record_offset":27700,"ancestor_record_offset":27316,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":27672,"parent_record_offset":27588,"ancestor_record_offset":27316,"inlinee":4220,"source_line":2685,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":28560,"parent_record_offset":28528,"ancestor_record_offset":27316,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":29144,"parent_record_offset":29116,"ancestor_record_offset":27316,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1375,"record_offset":28932,"parent_record_offset":28624,"ancestor_record_offset":27316,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$0::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1375,"record_offset":27700,"parent_record_offset":27672,"ancestor_record_offset":27316,"inlinee":4222,"source_line":14,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1375,"record_offset":28592,"parent_record_offset":28560,"ancestor_record_offset":27316,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1375,"record_offset":28624,"parent_record_offset":28592,"ancestor_record_offset":27316,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1375,"record_offset":30312,"parent_record_offset":30284,"ancestor_record_offset":27316,"inlinee":4115,"source_line":199,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1375,"record_offset":28452,"parent_record_offset":28428,"ancestor_record_offset":27316,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1375,"record_offset":29004,"parent_record_offset":28980,"ancestor_record_offset":27316,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1375,"record_offset":29056,"parent_record_offset":29028,"ancestor_record_offset":27316,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1375,"record_offset":29264,"parent_record_offset":29240,"ancestor_record_offset":27316,"inlinee":4620,"source_line":207,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1375,"record_offset":28428,"parent_record_offset":28404,"ancestor_record_offset":27316,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1375,"record_offset":28404,"parent_record_offset":28336,"ancestor_record_offset":27316,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1375,"record_offset":28528,"parent_record_offset":27588,"ancestor_record_offset":27316,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1375,"record_offset":29028,"parent_record_offset":29004,"ancestor_record_offset":27316,"inlinee":5097,"source_line":2580,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1375,"record_offset":28980,"parent_record_offset":28956,"ancestor_record_offset":27316,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1375,"record_offset":29116,"parent_record_offset":29056,"ancestor_record_offset":27316,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1375,"record_offset":30336,"parent_record_offset":30312,"ancestor_record_offset":27316,"inlinee":4117,"source_line":392,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1375,"record_offset":29288,"parent_record_offset":29264,"ancestor_record_offset":27316,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1375,"record_offset":27556,"parent_record_offset":27532,"ancestor_record_offset":27316,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1375,"record_offset":29728,"parent_record_offset":27588,"ancestor_record_offset":27316,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1375,"record_offset":27892,"parent_record_offset":27868,"ancestor_record_offset":27316,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1375,"record_offset":27996,"parent_record_offset":27972,"ancestor_record_offset":27316,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$11::new::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1375,"record_offset":29368,"parent_record_offset":27588,"ancestor_record_offset":27316,"inlinee":63558,"source_line":556,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$11::new::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1375,"record_offset":29080,"parent_record_offset":29056,"ancestor_record_offset":27316,"inlinee":61493,"source_line":563,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::impl$10::new_storage_buffer<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 528
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1375,"record_offset":31004,"rva":16111232,"va":5384820352}
// 编译实例 RVAs: 0xf5d680
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12070732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384820352,"raw_hex":"e8b4d047ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5384820423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12071646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384820352,"raw_hex":"e822cd47ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5384821353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8581447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384820352,"raw_hex":"e8b90e7dff","size":5,"soft_fail":false,"target_va":5376239920,"va":5384821362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f84200
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t158025","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384820352,"raw_hex":"e849690200","size":5,"soft_fail":false,"target_va":5384978944,"va":5384820914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fffa50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t663941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384820352,"raw_hex":"e885210a00","size":5,"soft_fail":false,"target_va":5385484880,"va":5384820934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: sparse_instance_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18772936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384820352,"raw_hex":"e8c8731e01","size":5,"soft_fail":false,"target_va":5403593392,"va":5384820451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t18156923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384820352,"raw_hex":"e87b0d1501","size":5,"soft_fail":false,"target_va":5402978144,"va":5384821216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18157946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384820352,"raw_hex":"e87a111501","size":5,"soft_fail":false,"target_va":5402979152,"va":5384821201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1375,"record_offset":31228,"parent_record_offset":31004,"ancestor_record_offset":31004,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1375,"record_offset":31284,"parent_record_offset":31004,"ancestor_record_offset":31004,"inlinee":63553,"source_line":543,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1375,"record_offset":31540,"parent_record_offset":31516,"ancestor_record_offset":31004,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1375,"record_offset":31564,"parent_record_offset":31540,"ancestor_record_offset":31004,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1375,"record_offset":33496,"parent_record_offset":33472,"ancestor_record_offset":31004,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1375,"record_offset":33260,"parent_record_offset":33232,"ancestor_record_offset":31004,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1375,"record_offset":31516,"parent_record_offset":31488,"ancestor_record_offset":31004,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1375,"record_offset":33472,"parent_record_offset":33448,"ancestor_record_offset":31004,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1375,"record_offset":33232,"parent_record_offset":33204,"ancestor_record_offset":31004,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1375,"record_offset":33448,"parent_record_offset":33424,"ancestor_record_offset":31004,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1375,"record_offset":31488,"parent_record_offset":31460,"ancestor_record_offset":31004,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1375,"record_offset":31460,"parent_record_offset":31432,"ancestor_record_offset":31004,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1375,"record_offset":31632,"parent_record_offset":31460,"ancestor_record_offset":31004,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1375,"record_offset":33744,"parent_record_offset":33720,"ancestor_record_offset":31004,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1375,"record_offset":33204,"parent_record_offset":33176,"ancestor_record_offset":31004,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$52::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1375,"record_offset":33176,"parent_record_offset":31284,"ancestor_record_offset":31004,"inlinee":5311,"source_line":3285,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":32652,"parent_record_offset":32628,"ancestor_record_offset":31004,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":31668,"parent_record_offset":31396,"ancestor_record_offset":31004,"inlinee":4231,"source_line":2627,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":32356,"parent_record_offset":32320,"ancestor_record_offset":31004,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":31432,"parent_record_offset":31396,"ancestor_record_offset":31004,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":31368,"parent_record_offset":31284,"ancestor_record_offset":31004,"inlinee":4220,"source_line":2685,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":32256,"parent_record_offset":32224,"ancestor_record_offset":31004,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":32840,"parent_record_offset":32812,"ancestor_record_offset":31004,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1375,"record_offset":32628,"parent_record_offset":32320,"ancestor_record_offset":31004,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$0::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1375,"record_offset":31396,"parent_record_offset":31368,"ancestor_record_offset":31004,"inlinee":4222,"source_line":14,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1375,"record_offset":32288,"parent_record_offset":32256,"ancestor_record_offset":31004,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1375,"record_offset":32320,"parent_record_offset":32288,"ancestor_record_offset":31004,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1375,"record_offset":34008,"parent_record_offset":33980,"ancestor_record_offset":31004,"inlinee":4115,"source_line":199,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1375,"record_offset":32148,"parent_record_offset":32124,"ancestor_record_offset":31004,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1375,"record_offset":32700,"parent_record_offset":32676,"ancestor_record_offset":31004,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1375,"record_offset":32752,"parent_record_offset":32724,"ancestor_record_offset":31004,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1375,"record_offset":32960,"parent_record_offset":32936,"ancestor_record_offset":31004,"inlinee":4620,"source_line":207,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1375,"record_offset":32124,"parent_record_offset":32100,"ancestor_record_offset":31004,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1375,"record_offset":32100,"parent_record_offset":32032,"ancestor_record_offset":31004,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1375,"record_offset":32224,"parent_record_offset":31284,"ancestor_record_offset":31004,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1375,"record_offset":32724,"parent_record_offset":32700,"ancestor_record_offset":31004,"inlinee":5097,"source_line":2580,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1375,"record_offset":32676,"parent_record_offset":32652,"ancestor_record_offset":31004,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1375,"record_offset":32812,"parent_record_offset":32752,"ancestor_record_offset":31004,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1375,"record_offset":34032,"parent_record_offset":34008,"ancestor_record_offset":31004,"inlinee":4117,"source_line":392,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1375,"record_offset":32984,"parent_record_offset":32960,"ancestor_record_offset":31004,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1375,"record_offset":31252,"parent_record_offset":31228,"ancestor_record_offset":31004,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1375,"record_offset":33424,"parent_record_offset":31284,"ancestor_record_offset":31004,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1375,"record_offset":31588,"parent_record_offset":31564,"ancestor_record_offset":31004,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1375,"record_offset":31692,"parent_record_offset":31668,"ancestor_record_offset":31004,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$11::new::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1375,"record_offset":33064,"parent_record_offset":31284,"ancestor_record_offset":31004,"inlinee":63558,"source_line":556,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$11::new::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1375,"record_offset":32776,"parent_record_offset":32752,"ancestor_record_offset":31004,"inlinee":61493,"source_line":563,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::SparseInstanceBuffer::new
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 543
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF841
// 首条位置: {"module":1375,"inlinee":63553}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$52::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1375,"record_offset":25792,"parent_record_offset":23760,"ancestor_record_offset":23760,"inlinee":5311,"source_line":3285,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":23844,"parent_record_offset":23760,"ancestor_record_offset":23760,"inlinee":4220,"source_line":2685,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1375,"record_offset":24824,"parent_record_offset":23760,"ancestor_record_offset":23760,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1375,"record_offset":26040,"parent_record_offset":23760,"ancestor_record_offset":23760,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_direct] country_core::geometry::instanced_wall::impl$34::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/geometry/instanced_wall.rs
//     evidence={"module":1375,"record_offset":24208,"parent_record_offset":23760,"ancestor_record_offset":23760,"inlinee":63555,"source_line":38,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::impl$11::new::closure$0
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1375,"record_offset":25680,"parent_record_offset":23760,"ancestor_record_offset":23760,"inlinee":63558,"source_line":556,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1375,"record_offset":24016,"parent_record_offset":23992,"ancestor_record_offset":23760,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1375,"record_offset":24040,"parent_record_offset":24016,"ancestor_record_offset":23760,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1375,"record_offset":26112,"parent_record_offset":26088,"ancestor_record_offset":23760,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1375,"record_offset":25876,"parent_record_offset":25848,"ancestor_record_offset":23760,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1375,"record_offset":23992,"parent_record_offset":23964,"ancestor_record_offset":23760,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1375,"record_offset":26088,"parent_record_offset":26064,"ancestor_record_offset":23760,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1375,"record_offset":25848,"parent_record_offset":25820,"ancestor_record_offset":23760,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1375,"record_offset":26064,"parent_record_offset":26040,"ancestor_record_offset":23760,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1375,"record_offset":23964,"parent_record_offset":23936,"ancestor_record_offset":23760,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1375,"record_offset":23936,"parent_record_offset":23908,"ancestor_record_offset":23760,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1375,"record_offset":24108,"parent_record_offset":23936,"ancestor_record_offset":23760,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1375,"record_offset":26360,"parent_record_offset":26336,"ancestor_record_offset":23760,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1375,"record_offset":25820,"parent_record_offset":25792,"ancestor_record_offset":23760,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":25252,"parent_record_offset":25228,"ancestor_record_offset":23760,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":24144,"parent_record_offset":23872,"ancestor_record_offset":23760,"inlinee":4231,"source_line":2627,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":24956,"parent_record_offset":24920,"ancestor_record_offset":23760,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":23908,"parent_record_offset":23872,"ancestor_record_offset":23760,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":24856,"parent_record_offset":24824,"ancestor_record_offset":23760,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1375,"record_offset":25448,"parent_record_offset":25416,"ancestor_record_offset":23760,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1375,"record_offset":25228,"parent_record_offset":24920,"ancestor_record_offset":23760,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$0::from_elem
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1375,"record_offset":23872,"parent_record_offset":23844,"ancestor_record_offset":23760,"inlinee":4222,"source_line":14,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1375,"record_offset":24888,"parent_record_offset":24856,"ancestor_record_offset":23760,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1375,"record_offset":24920,"parent_record_offset":24888,"ancestor_record_offset":23760,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1375,"record_offset":26624,"parent_record_offset":26596,"ancestor_record_offset":23760,"inlinee":4115,"source_line":199,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$49::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\mod.rs
//     evidence={"module":1375,"record_offset":24260,"parent_record_offset":24236,"ancestor_record_offset":23760,"inlinee":25444,"source_line":439,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$57::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\mod.rs
//     evidence={"module":1375,"record_offset":24292,"parent_record_offset":24208,"ancestor_record_offset":23760,"inlinee":11220,"source_line":439,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1375,"record_offset":24748,"parent_record_offset":24724,"ancestor_record_offset":23760,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1375,"record_offset":25300,"parent_record_offset":25276,"ancestor_record_offset":23760,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1375,"record_offset":25352,"parent_record_offset":25324,"ancestor_record_offset":23760,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1375,"record_offset":25576,"parent_record_offset":25552,"ancestor_record_offset":23760,"inlinee":4620,"source_line":207,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1375,"record_offset":24724,"parent_record_offset":24700,"ancestor_record_offset":23760,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1375,"record_offset":24700,"parent_record_offset":24632,"ancestor_record_offset":23760,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1375,"record_offset":25704,"parent_record_offset":25680,"ancestor_record_offset":23760,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1375,"record_offset":25324,"parent_record_offset":25300,"ancestor_record_offset":23760,"inlinee":5097,"source_line":2580,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1375,"record_offset":25276,"parent_record_offset":25252,"ancestor_record_offset":23760,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1375,"record_offset":25416,"parent_record_offset":25352,"ancestor_record_offset":23760,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1375,"record_offset":26648,"parent_record_offset":26624,"ancestor_record_offset":23760,"inlinee":4117,"source_line":392,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1375,"record_offset":25600,"parent_record_offset":25576,"ancestor_record_offset":23760,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1375,"record_offset":26176,"parent_record_offset":26152,"ancestor_record_offset":23760,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1375,"record_offset":24064,"parent_record_offset":24040,"ancestor_record_offset":23760,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1375,"record_offset":24168,"parent_record_offset":24144,"ancestor_record_offset":23760,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$11::new::closure$1
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1375,"record_offset":25376,"parent_record_offset":25352,"ancestor_record_offset":23760,"inlinee":61493,"source_line":563,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::render::impl$4::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/render/mod.rs
//     evidence={"module":1375,"record_offset":24236,"parent_record_offset":24208,"ancestor_record_offset":23760,"inlinee":63557,"source_line":8,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::impl$11::new::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 556
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF846
// 首条位置: {"module":1375,"inlinee":63558}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1375,"record_offset":25704,"parent_record_offset":25680,"ancestor_record_offset":25680,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01375.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::impl$11::new::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 563
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF035
// 首条位置: {"module":1308,"inlinee":61493}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::SparseInstanceBuffer::buffer
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 577
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6AD8
// 首条位置: {"module":485,"inlinee":27352}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::SparseInstanceBuffer::buffer<country_core::geometry::instanced_wall::BrickTransformSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 578
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1375,"record_offset":34700,"rva":531360,"va":5369240480}
// 编译实例 RVAs: 0x81ba0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::impl$12::handle<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 578
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1375,"record_offset":36108,"rva":531360,"va":5369240480}
// 编译实例 RVAs: 0x81ba0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::bitmap_set
// 记录形式: inline_instance, inlinee_declaration; instances=31
// 源码行: 583
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB37B
// 首条位置: {"module":751,"inlinee":45947}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::fetch_or
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":9852,"parent_record_offset":9820,"ancestor_record_offset":9820,"inlinee":8935,"source_line":2794,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_or
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":9876,"parent_record_offset":9852,"ancestor_record_offset":9820,"inlinee":8936,"source_line":3471,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::bitmap_unset
// 记录形式: inline_instance, inlinee_declaration; instances=16
// 源码行: 588
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB373
// 首条位置: {"module":751,"inlinee":45939}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::fetch_and
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":1744,"parent_record_offset":1712,"ancestor_record_offset":1712,"inlinee":19100,"source_line":2728,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_and
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":751,"record_offset":1768,"parent_record_offset":1744,"ancestor_record_offset":1712,"inlinee":19101,"source_line":3439,"archive":"_pdb/A/modules/00751.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::impl$12::as_any<country_core::geometry::instanced_wall::BrickTransformSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 593
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1375,"record_offset":34912,"rva":16112880,"va":5384822000}
// 编译实例 RVAs: 0xf5dcf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::impl$12::as_any<country_core::resources::roof_tile_ssbo::RoofTileSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 593
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1375,"record_offset":35312,"rva":16112912,"va":5384822032}
// 编译实例 RVAs: 0xf5dd10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::impl$12::as_any<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 593
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1375,"record_offset":35112,"rva":16112896,"va":5384822016}
// 编译实例 RVAs: 0xf5dd00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::impl$12::as_any_mut<country_core::geometry::instanced_wall::BrickTransformSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 593
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1375,"record_offset":35504,"rva":16112880,"va":5384822000}
// 编译实例 RVAs: 0xf5dcf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::impl$12::as_any_mut<country_core::resources::roof_tile_ssbo::RoofTileSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 593
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1375,"record_offset":35912,"rva":16112912,"va":5384822032}
// 编译实例 RVAs: 0xf5dd10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::sparse_instance_buffer::impl$12::as_any_mut<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 593
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1375,"record_offset":35708,"rva":16112896,"va":5384822016}
// 编译实例 RVAs: 0xf5dd00
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
// 完整结构及原始类型记录: sparse_instance_buffer.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::sparse_instance_buffer::AllocationHandle
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::drain::Drain<country_core::render::sparse_instance_buffer::AllocationHandle,alloc::alloc::Global>", "constructor": "alloc::vec::drain::Drain", "arguments": ["country_core::render::sparse_instance_buffer::AllocationHandle", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::drain::Drain<country_core::render::sparse_instance_buffer::AllocationHandle,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::drain::Drain<country_core::render::sparse_instance_buffer::AllocationHandle,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<country_core::render::sparse_instance_buffer::AllocationHandle,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["country_core::render::sparse_instance_buffer::AllocationHandle", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::sparse_instance_buffer::ChunkId
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 15
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::drain::Drain<country_core::render::sparse_instance_buffer::ChunkId,alloc::alloc::Global>", "constructor": "alloc::vec::drain::Drain", "arguments": ["country_core::render::sparse_instance_buffer::ChunkId", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::drain::Drain<country_core::render::sparse_instance_buffer::ChunkId,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::drain::Drain<country_core::render::sparse_instance_buffer::ChunkId,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::rev::Rev<alloc::vec::drain::Drain<country_core::render::sparse_instance_buffer::ChunkId,alloc::alloc::Global> >", "constructor": "core::iter::adapters::rev::Rev", "arguments": ["alloc::vec::drain::Drain<country_core::render::sparse_instance_buffer::ChunkId,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::rev::Rev<alloc::vec::drain::Drain<country_core::render::sparse_instance_buffer::ChunkId,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::rev::Rev<alloc::vec::drain::Drain<country_core::render::sparse_instance_buffer::ChunkId,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<country_core::render::sparse_instance_buffer::ChunkId,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["country_core::render::sparse_instance_buffer::ChunkId", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle<country_core::render::sparse_instance_buffer::ChunkId,alloc::alloc::Global>", "constructor": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle", "arguments": ["country_core::render::sparse_instance_buffer::ChunkId", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::sparse_instance_buffer::SparseInstanceBuffer
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 16; 过程引用数: 13
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::SparseInstanceBuffer<country_core::resources::roof_tile_ssbo::RoofTileSsbo>", "constructor": "country_core::render::sparse_instance_buffer::SparseInstanceBuffer", "arguments": ["country_core::resources::roof_tile_ssbo::RoofTileSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBuffer<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::render::sparse_instance_buffer::SparseInstanceBuffer<country_core::resources::roof_tile_ssbo::RoofTileSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::SparseInstanceBuffer<country_core::geometry::instanced_wall::BrickTransformSsbo>", "constructor": "country_core::render::sparse_instance_buffer::SparseInstanceBuffer", "arguments": ["country_core::geometry::instanced_wall::BrickTransformSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBuffer<country_core::geometry::instanced_wall::BrickTransformSsbo> >", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::render::sparse_instance_buffer::SparseInstanceBuffer<country_core::geometry::instanced_wall::BrickTransformSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush<country_core::resources::roof_tile_ssbo::RoofTileSsbo>", "constructor": "country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush", "arguments": ["country_core::resources::roof_tile_ssbo::RoofTileSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>", "constructor": "country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush", "arguments": ["country_core::resources::roofs::roof_buffers::WoodenPlankSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::SparseInstanceBuffer<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>", "constructor": "country_core::render::sparse_instance_buffer::SparseInstanceBuffer", "arguments": ["country_core::resources::roofs::roof_buffers::WoodenPlankSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::sparse_instance_buffer::SparseInstanceBuffer<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::sparse_instance_buffer::SparseInstanceBuffer<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::sparse_instance_buffer::SparseInstanceBuffer<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::sparse_instance_buffer::SparseInstanceBuffer<country_core::resources::roof_tile_ssbo::RoofTileSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::render::sparse_instance_buffer::SparseInstanceBuffer<country_core::geometry::instanced_wall::BrickTransformSsbo> >", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::render::sparse_instance_buffer::SparseInstanceBuffer<country_core::geometry::instanced_wall::BrickTransformSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush<country_core::geometry::instanced_wall::BrickTransformSsbo>", "constructor": "country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush", "arguments": ["country_core::geometry::instanced_wall::BrickTransformSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::SparseInstanceBuffer::allocation_to_draw_multi_instanced<country_core::geometry::instanced_wall::BrickTransformSsbo>", "constructor": "country_core::render::sparse_instance_buffer::SparseInstanceBuffer::allocation_to_draw_multi_instanced", "arguments": ["country_core::geometry::instanced_wall::BrickTransformSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::SparseInstanceBuffer::hide<country_core::geometry::instanced_wall::BrickTransformSsbo>", "constructor": "country_core::render::sparse_instance_buffer::SparseInstanceBuffer::hide", "arguments": ["country_core::geometry::instanced_wall::BrickTransformSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::SparseInstanceBuffer::show<country_core::geometry::instanced_wall::BrickTransformSsbo>", "constructor": "country_core::render::sparse_instance_buffer::SparseInstanceBuffer::show", "arguments": ["country_core::geometry::instanced_wall::BrickTransformSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::SparseInstanceBuffer::take_out_waste<country_core::geometry::instanced_wall::BrickTransformSsbo>", "constructor": "country_core::render::sparse_instance_buffer::SparseInstanceBuffer::take_out_waste", "arguments": ["country_core::geometry::instanced_wall::BrickTransformSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::SparseInstanceBuffer::buffer<country_core::geometry::instanced_wall::BrickTransformSsbo>", "constructor": "country_core::render::sparse_instance_buffer::SparseInstanceBuffer::buffer", "arguments": ["country_core::geometry::instanced_wall::BrickTransformSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 21
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::geometry::instanced_wall::BrickTransformSsbo>", "constructor": "country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation", "arguments": ["country_core::geometry::instanced_wall::BrickTransformSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::geometry::instanced_wall::BrickTransformSsbo> >", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::geometry::instanced_wall::BrickTransformSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo>", "constructor": "country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation", "arguments": ["country_core::resources::roof_tile_ssbo::RoofTileSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >", "constructor": "core::option::Iter", "arguments": ["country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >", "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>", "country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0>", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >", "core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>", "country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::flatten::FlatMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>", "constructor": "country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation", "arguments": ["country_core::resources::roofs::roof_buffers::WoodenPlankSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo> >", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::geometry::instanced_wall::BrickTransformSsbo> >", "constructor": "core::option::Option", "arguments": ["country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::geometry::instanced_wall::BrickTransformSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::geometry::instanced_wall::BrickTransformSsbo> > >", "constructor": "enum2$", "arguments": ["core::option::Option<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::geometry::instanced_wall::BrickTransformSsbo> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::geometry::instanced_wall::BrickTransformSsbo> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::geometry::instanced_wall::BrickTransformSsbo> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::walls::WallId,country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >", "constructor": "tuple$", "arguments": ["country_core::resources::walls::WallId", "country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::walls::WallId,country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::equivalent_key::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >", "constructor": "hashbrown::map::equivalent_key::closure_env$0", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::WallId", "country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::remove_entry<tuple$<country_core::resources::walls::WallId,country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> > >", "constructor": "hashbrown::raw::RawTable::remove_entry", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >", "alloc::alloc::Global", "hashbrown::map::equivalent_key::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::WallId,country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo>,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::resources::walls::WallId", "country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::resources::walls::WallId,country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::WallId,country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::resources::walls::WallId,country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::resources::walls::WallId,country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::impl$3::next<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0>", "constructor": "core::iter::adapters::flatten::impl$3::next", "arguments": ["bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >", "core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>", "country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::impl$3::size_hint<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0>", "constructor": "core::iter::adapters::flatten::impl$3::size_hint", "arguments": ["bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >", "core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>", "country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >", "constructor": "ref_mut$", "arguments": ["core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::and_then_or_clear<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants,enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)>", "constructor": "core::iter::adapters::flatten::and_then_or_clear", "arguments": ["core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>", "country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants", "enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants,core::iter::adapters::flatten::FlatMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants", "core::iter::adapters::flatten::FlatMap<bevy_ecs::query::iter::QueryIter<tuple$<ref$<country_core::resources::roofs::roof_tiles::RoofTiles>,ref$<country_core::resources::roofs::roof_tile_sim::RoofTileSimulation>,ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> >,tuple$<> >,core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::sparse_instance_buffer::SparseInstanceBufferWasteBin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 7
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<country_core::render::sparse_instance_buffer::SparseInstanceBufferWasteBin,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["country_core::render::sparse_instance_buffer::SparseInstanceBufferWasteBin", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 14
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::push<country_core::resources::roof_tile_ssbo::RoofTileSsbo>", "constructor": "country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::push", "arguments": ["country_core::resources::roof_tile_ssbo::RoofTileSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::clear<country_core::resources::roof_tile_ssbo::RoofTileSsbo>", "constructor": "country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::clear", "arguments": ["country_core::resources::roof_tile_ssbo::RoofTileSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::push<country_core::geometry::instanced_wall::BrickTransformSsbo>", "constructor": "country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::push", "arguments": ["country_core::geometry::instanced_wall::BrickTransformSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::push<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>", "constructor": "country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::push", "arguments": ["country_core::resources::roofs::roof_buffers::WoodenPlankSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::clear<country_core::geometry::instanced_wall::BrickTransformSsbo>", "constructor": "country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::clear", "arguments": ["country_core::geometry::instanced_wall::BrickTransformSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::clear<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>", "constructor": "country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::clear", "arguments": ["country_core::resources::roofs::roof_buffers::WoodenPlankSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: sparse_instance_buffer.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: sparse_instance_buffer.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 287; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyRequests>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyRequests>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,bevy_ecs::change_detection::ResMut<system_decorator::backup::DecoratorBackup>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>", "bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>", "bevy_ecs::change_detection::ResMut<system_decorator::backup::DecoratorBackup>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::backup::DecoratorBackup>
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::collision_geo::DormerCollisionGeoLib>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::collision_geo::DormerCollisionGeoLib>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::collision_geo::DormerCollisionGeoLib>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::components::LanternLightParams>,ref$<country_core::resources::color_id::ColorId>,enum2$<core::option::Option<ref$<country_core::components::SemiTransparent> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::lantern::CacheLights>,bevy_ecs::event::EventWriter<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>,bevy_ecs::event::EventReader<country_core::resources::events::LightLanternCmd>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::components::LanternLightParams>,ref$<country_core::resources::color_id::ColorId>,enum2$<core::option::Option<ref$<country_core::components::SemiTransparent> > > >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::lantern::CacheLights>", "bevy_ecs::event::EventWriter<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>", "bevy_ecs::event::EventReader<country_core::resources::events::LightLanternCmd>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::components::LanternLightParams>,ref$<country_core::resources::color_id::ColorId>,enum2$<core::option::Option<ref$<country_core::components::SemiTransparent> > > >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::lantern::CacheLights>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::events::LightLanternCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time_of_day::TimeOfDay>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::commands::Commands,bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::generate_clothline::CacheClothes>,bevy_ecs::event::EventWriter<country_core::systems::audio::clutter_audio::PlayClutterSoundCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::system::commands::Commands", "bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::generate_clothline::CacheClothes>", "bevy_ecs::event::EventWriter<country_core::systems::audio::clutter_audio::PlayClutterSoundCmd>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::generate_clothline::CacheClothes>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::clutter_audio::PlayClutterSoundCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::WindowClutterCandidates>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>", "bevy_ecs::change_detection::ResMut<country_core::systems::collision::WindowClutterCandidates>", "bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::WindowClutterCandidates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>,bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::CacheDormerPositions>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>", "bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::CacheDormerPositions>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::CacheDormerPositions>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::AllocationHandle::is_null
// 信息 ID: c1ec932d646a07cf876dda66
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 9 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::geometry::instanced_wall::BrickTransformSsbo> > @ 0x368f62 -> 0x6c0410 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> > | core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::geometry::instanced_wall::BrickTransformSsbo> > @ 0x8cd082 -> 0x6c0410 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::geometry::instanced_wall::BrickTransformSsbo> > @ 0x996e02 -> 0x6c0410 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::geometry::instanced_wall::BrickTransformSsbo> > @ 0x9a5552 -> 0x6c0410 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::geometry::instanced_wall::BrickTransformSsbo> > | core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo> > @ 0x9b0db2 -> 0x6c0410 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<enum2$<core::option::Option<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::geometry::instanced_wall::BrickTransformSsbo> > > > @ 0xc9788c -> 0x6c0410 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> > | core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::geometry::instanced_wall::BrickTransformSsbo> > @ 0xd01be2 -> 0x6c0410 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo> > @ 0x12e6372 -> 0x6c0410 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::geometry::instanced_wall::BrickTransformSsbo> > @ 0x155c262 -> 0x6c0410 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >; module=500, inline_record=211228; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::SparseInstanceBuffer::allocate
// 信息 ID: 5e459e1601d223fffd1e1465
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 13 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_roof::visual::assemble_roof_tiles::assemble_roof_tiles; module=266, inline_record=7412; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_walls::construct_walls; module=371, inline_record=22384; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections; module=397, inline_record=17876; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::publish_public_wall_state::publish_public_wall_state; module=445, inline_record=37876; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::publish_public_wall_state::publish_public_wall_state; module=445, inline_record=38384; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::publish_public_wall_state::publish_public_wall_state; module=445, inline_record=38892; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::publish_public_wall_state::publish_public_wall_state; module=445, inline_record=39676; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::publish_public_wall_state::publish_public_wall_state; module=445, inline_record=40184; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::SparseInstanceBuffer::allocation_to_draw_multi_instanced<country_core::geometry::instanced_wall::BrickTransformSsbo>
// 信息 ID: 18cb1e94b170d729e650814d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 4f12dbb1c2f8707c218c2f1d: enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// [类型关系] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::allocation_to_draw_multi_instanced -> ["country_core::geometry::instanced_wall::BrickTransformSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists @ 0x6119cc -> 0xf5b850 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists_pass2 @ 0x6124ae -> 0xf5b850 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::flatten::and_then_or_clear<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants,enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)> @ 0xd6eb98 -> 0xf5b850 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::SparseInstanceBuffer::buffer
// 信息 ID: 01306c7db3723b9c9245502f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::draw_shaders::draw_bricks::indirect_draw_bricks<0>; module=485, inline_record=39480; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::draw_shaders::draw_bricks::indirect_draw_bricks<1>; module=485, inline_record=45308; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::SparseInstanceBuffer::buffer<country_core::geometry::instanced_wall::BrickTransformSsbo>
// 信息 ID: 394518f244893f2bf723a9d5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::buffer -> ["country_core::geometry::instanced_wall::BrickTransformSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::SparseInstanceBuffer::drain_chunk_bitmap
// 信息 ID: dfcde01ca5c895eb90afdcbd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush<country_core::resources::roof_tile_ssbo::RoofTileSsbo>; module=751, inline_record=6612; 不是独立调用边。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>; module=1214, inline_record=6632; 不是独立调用边。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush<country_core::geometry::instanced_wall::BrickTransformSsbo>; module=1375, inline_record=8488; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::SparseInstanceBuffer::free_impl
// 信息 ID: 0133dfb91cd1cf0850f76468
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush<country_core::resources::roof_tile_ssbo::RoofTileSsbo>; module=751, inline_record=1096; 不是独立调用边。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>; module=1214, inline_record=1116; 不是独立调用边。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::take_out_waste<country_core::geometry::instanced_wall::BrickTransformSsbo>; module=1375, inline_record=14224; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush<country_core::geometry::instanced_wall::BrickTransformSsbo>
// 信息 ID: 7f3a42c887017f217a11f9b8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush -> ["country_core::geometry::instanced_wall::BrickTransformSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::upload_instanced_wall_ssbo::gpu_flush_brick_buffers @ 0x6423a9 -> 0xf5b150 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush<country_core::resources::roof_tile_ssbo::RoofTileSsbo>
// 信息 ID: 29b9be6f841410c005b4ae23
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5d58aab8b83fbd13d0042c71: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// [类型关系] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush -> ["country_core::resources::roof_tile_ssbo::RoofTileSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> @ 0x8ea2cf -> 0x8da4c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)> > @ 0x12d5d75 -> 0x8da4c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>
// 信息 ID: 47fec20c5bd3e66703df7ba0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush -> ["country_core::resources::roofs::roof_buffers::WoodenPlankSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::gpu_flush_plank_buffers @ 0x4e6256 -> 0xc98510 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::SparseInstanceBuffer::hide<country_core::geometry::instanced_wall::BrickTransformSsbo>
// 信息 ID: d9e6f9ba2a52acda370f2190
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::hide -> ["country_core::geometry::instanced_wall::BrickTransformSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists @ 0x611a13 -> 0xf5b9d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists_pass2 @ 0x6124f5 -> 0xf5b9d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::SparseInstanceBuffer::iter_all_chunks
// 信息 ID: 2a84f7d595da2b6ff20d48b7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::impl$3::to_draw_multi_instanced<country_core::geometry::instanced_wall::BrickTransformSsbo>; module=1375, inline_record=18240; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::SparseInstanceBuffer::iter_allocation_chunks
// 信息 ID: 26e412d08a91d117218299f4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::allocation_to_draw_multi_instanced<country_core::geometry::instanced_wall::BrickTransformSsbo>; module=1375, inline_record=11704; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::SparseInstanceBuffer::iter_chunk_bitmap
// 信息 ID: 6d62b02914c2487212bc091e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::impl$3::to_draw_multi_instanced<country_core::geometry::instanced_wall::BrickTransformSsbo>; module=1375, inline_record=18264; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::SparseInstanceBuffer::new
// 信息 ID: 1fb77a2327e793b3a6be2ea6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::impl$10::new_storage_buffer<country_core::geometry::instanced_wall::BrickTransformSsbo>; module=1375, inline_record=23760; 不是独立调用边。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::impl$10::new_storage_buffer<country_core::resources::roof_tile_ssbo::RoofTileSsbo>; module=1375, inline_record=27588; 不是独立调用边。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::impl$10::new_storage_buffer<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>; module=1375, inline_record=31284; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::SparseInstanceBuffer::show<country_core::geometry::instanced_wall::BrickTransformSsbo>
// 信息 ID: 47fb6ee4c5478ad5dd56da0a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::show -> ["country_core::geometry::instanced_wall::BrickTransformSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists @ 0x611a90 -> 0xf5ba50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists_pass2 @ 0x612579 -> 0xf5ba50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::SparseInstanceBuffer::take_out_waste
// 信息 ID: 0986a5e1e52a07705f5dc4c4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush<country_core::resources::roof_tile_ssbo::RoofTileSsbo>; module=751, inline_record=500; 不是独立调用边。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>; module=1214, inline_record=520; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::SparseInstanceBuffer::take_out_waste<country_core::geometry::instanced_wall::BrickTransformSsbo>
// 信息 ID: d933048cdeee9d3b2d94e023
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::take_out_waste -> ["country_core::geometry::instanced_wall::BrickTransformSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections @ 0x53b26e -> 0xf5bad0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush<country_core::geometry::instanced_wall::BrickTransformSsbo> @ 0xf5b17e -> 0xf5bad0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::SparseInstanceBuffer::to_draw_multi_instanced_impl
// 信息 ID: af2f4b5e8819f5341cc67d3d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::allocation_to_draw_multi_instanced<country_core::geometry::instanced_wall::BrickTransformSsbo>; module=1375, inline_record=11860; 不是独立调用边。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::impl$3::to_draw_multi_instanced<country_core::geometry::instanced_wall::BrickTransformSsbo>; module=1375, inline_record=18380; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
// 信息 ID: 02a8c95d4ef4620f16d43cd2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 32 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_roof::visual::assemble_roof_tiles::assemble_roof_tiles; module=266, inline_record=8124; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments::closure$1; module=360, inline_record=14780; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments::closure$1; module=360, inline_record=15548; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_arches::construct_arches; module=361, inline_record=11992; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::construct_lintels; module=363, inline_record=10212; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_gates::systems::construct_gates; module=366, inline_record=14424; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::gpu_clear_wall_gate_pieces; module=368, inline_record=2720; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::place_pillars::place_pillars; module=369, inline_record=15508; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::clear<country_core::geometry::instanced_wall::BrickTransformSsbo>
// 信息 ID: 57314ad04105765d4124ebfd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::clear -> ["country_core::geometry::instanced_wall::BrickTransformSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments::closure$1 @ 0x4d506e -> 0xc996d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments::closure$1 @ 0x4d5169 -> 0xc996d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x4d7a33 -> 0xc996d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::gpu_clear_wall_gate_pieces @ 0x4e6119 -> 0xc996d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcce50a -> 0xc996d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::geometry::instanced_wall::InstancedWall::upload_to_storage_buffer @ 0x5fe58a -> 0xf5c480 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::stitch_bricks::spawn_stitch_bricks @ 0x14e3b11 -> 0x1ae0020 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::clear<country_core::resources::roof_tile_ssbo::RoofTileSsbo>
// 信息 ID: 70b278825184707e8d8253bd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::clear -> ["country_core::resources::roof_tile_ssbo::RoofTileSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_roof::visual::assemble_roof_tiles::assemble_roof_tiles @ 0x3f0590 -> 0x8db380 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs @ 0x17a99a2 -> 0x17e7940 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::chimney_assemble_bricks::chimney_assemble_bricks @ 0x1c6c689 -> 0x17e7940 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::clear<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>
// 信息 ID: f4bc641f2a9836beb529c1e6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::clear -> ["country_core::resources::roofs::roof_buffers::WoodenPlankSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] timberframe::timberframe_rectangle::construct_timberframe_rectangle @ 0x12e9995 -> 0x198f6d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::push<country_core::geometry::instanced_wall::BrickTransformSsbo>
// 信息 ID: 92a36c55b1dd50c557846101
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::push -> ["country_core::geometry::instanced_wall::BrickTransformSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 13 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x4f1584 -> 0xc99100 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret @ 0xca1054 -> 0xc99100 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets @ 0xca5955 -> 0xc99100 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches::closure$3 @ 0xcba468 -> 0xc99100 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_gates::construct_wooden_gate_doors @ 0xcc1dbb -> 0xc99100 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::place_pillars::place_pillars::closure$4 @ 0xcc7e19 -> 0xc99100 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_bottom_layer::add_bottom_brick<ref_mut$<system_wall_constructor::construct_elevation_supports::circle_bottom_layer::construct_circle_bottom_layer::closure_env$0> > | system_wall_constructor::construct_elevation_supports::circle_bottom_layer::add_bottom_brick<ref_mut$<system_wall_constructor::construct_elevation_supports::rectangle_bottom_layer::construct_rect_bottom_layer::closure_env$1> > @ 0x1773c1d -> 0xc99100 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::util_cross_brick_pillar::construct_crossbrick_pillar<ref_mut$<system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base::closure_env$1> > @ 0x1c3206c -> 0xc99100 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections @ 0x53c2e7 -> 0xf5c180 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::geometry::instanced_wall::InstancedWall::upload_to_storage_buffer @ 0x5fe5f4 -> 0xf5c180 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::chimney_assemble_bricks::construct_crossbrick_chimney<system_decorator::decorator_visual::chimney_assemble_bricks::chimney_assemble_bricks::closure_env$2> @ 0x17e455f -> 0x17e7370 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::stitch_bricks::spawn_stitch_bricks @ 0x14e3cce -> 0x1adfd20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_roof_merlons::construct_inter_roof_merlons::construct_inter_roof_merlons @ 0x1acddc3 -> 0x1adfd20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::push<country_core::resources::roof_tile_ssbo::RoofTileSsbo>
// 信息 ID: 0bb3cdb0d2d333e2bc752dd8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::push -> ["country_core::resources::roof_tile_ssbo::RoofTileSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 10 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$0 @ 0x8e0009 -> 0x8db0b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure$0 @ 0x8e1839 -> 0x8db0b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd189c3 -> 0x17e7670 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd18bc6 -> 0x17e7670 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd18c75 -> 0x17e7670 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd18e59 -> 0x17e7670 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd18f08 -> 0x17e7670 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd184fc -> 0x17e7670 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::util_assemble_gothic_dormer::assemble_gothic_dormer_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$6> @ 0x1c415af -> 0x17e7670 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::util_assemble_gothic_dormer::assemble_gothic_dormer_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$6> @ 0x1c4201e -> 0x17e7670 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::push<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>
// 信息 ID: e2353eda2462797864708c65
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::push -> ["country_core::resources::roofs::roof_buffers::WoodenPlankSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 30 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::place_pillars::place_pillars @ 0x4e798a -> 0xc99400 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret @ 0xca0e57 -> 0xc99400 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_bottom_layer::construct_rect_bottom_layer @ 0xca280d -> 0xc99400 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets @ 0xca5665 -> 0xc99400 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets @ 0xca8129 -> 0xc99400 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> > @ 0xcbac09 -> 0xc99400 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> > @ 0xcbaeb3 -> 0xc99400 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::add_diagonal_support<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0xcbd5d5 -> 0xc99400 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_fences::process_segment::process_segment<system_wall_constructor::construct_fences::construct_fences::closure_env$0> @ 0xcc9c79 -> 0xc99400 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_fences::process_segment::process_segment<system_wall_constructor::construct_fences::construct_fences::closure_env$0> @ 0xcca32a -> 0xc99400 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::roof_outline @ 0xccda18 -> 0xc99400 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::roof_outline @ 0xccdc8b -> 0xc99400 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd0e4e -> 0xc99400 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd0339 -> 0xc99400 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd2d6b -> 0xc99400 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd3179 -> 0xc99400 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::bitmap_set
// 信息 ID: 672a8d475f0044f72862e58d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 25 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::push<country_core::resources::roof_tile_ssbo::RoofTileSsbo>; module=751, inline_record=9820; 不是独立调用边。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::push<country_core::resources::roof_tile_ssbo::RoofTileSsbo>; module=751, inline_record=10344; 不是独立调用边。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::push<country_core::resources::roof_tile_ssbo::RoofTileSsbo>; module=751, inline_record=10436; 不是独立调用边。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::push<country_core::geometry::instanced_wall::BrickTransformSsbo>; module=1214, inline_record=9840; 不是独立调用边。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::push<country_core::geometry::instanced_wall::BrickTransformSsbo>; module=1214, inline_record=10364; 不是独立调用边。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::push<country_core::geometry::instanced_wall::BrickTransformSsbo>; module=1214, inline_record=10456; 不是独立调用边。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::push<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>; module=1214, inline_record=11168; 不是独立调用边。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::push<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>; module=1214, inline_record=11692; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::bitmap_unset
// 信息 ID: 5c008bf700ddc282017ac568
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 10 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush<country_core::resources::roof_tile_ssbo::RoofTileSsbo>; module=751, inline_record=1712; 不是独立调用边。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::clear<country_core::resources::roof_tile_ssbo::RoofTileSsbo>; module=751, inline_record=11384; 不是独立调用边。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>; module=1214, inline_record=1732; 不是独立调用边。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::clear<country_core::geometry::instanced_wall::BrickTransformSsbo>; module=1214, inline_record=12736; 不是独立调用边。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::hide<country_core::geometry::instanced_wall::BrickTransformSsbo>; module=1375, inline_record=12904; 不是独立调用边。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::take_out_waste<country_core::geometry::instanced_wall::BrickTransformSsbo>; module=1375, inline_record=14828; 不是独立调用边。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::clear<country_core::geometry::instanced_wall::BrickTransformSsbo>; module=1375, inline_record=21500; 不是独立调用边。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::clear<country_core::resources::roof_tile_ssbo::RoofTileSsbo>; module=2795, inline_record=3384; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::impl$10::new_storage_buffer<country_core::geometry::instanced_wall::BrickTransformSsbo>
// 信息 ID: b714648019a6fb6f218d3967
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::sparse_instance_buffer::impl$10::new_storage_buffer -> ["country_core::geometry::instanced_wall::BrickTransformSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::geometry::instanced_wall::BrickTransformSsbo> @ 0xd2b687 -> 0xf5c770 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::impl$10::new_storage_buffer<country_core::resources::roof_tile_ssbo::RoofTileSsbo>
// 信息 ID: 3172bdd4899f4739a61e1330
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::sparse_instance_buffer::impl$10::new_storage_buffer -> ["country_core::resources::roof_tile_ssbo::RoofTileSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::resources::roof_tile_ssbo::RoofTileSsbo> @ 0xd28957 -> 0xf5cf90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::impl$10::new_storage_buffer<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>
// 信息 ID: a47044d4afc58ff121448a5d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::sparse_instance_buffer::impl$10::new_storage_buffer -> ["country_core::resources::roofs::roof_buffers::WoodenPlankSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo> @ 0xd301b7 -> 0xf5d680 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::impl$11::new::closure$0
// 信息 ID: 7aa62ef4b8cfd0e31384ee04
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::sparse_instance_buffer::impl$11::new", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::impl$10::new_storage_buffer<country_core::geometry::instanced_wall::BrickTransformSsbo>; module=1375, inline_record=25680; 不是独立调用边。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::impl$10::new_storage_buffer<country_core::geometry::instanced_wall::BrickTransformSsbo>; module=1375, inline_record=25736; 不是独立调用边。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::impl$10::new_storage_buffer<country_core::resources::roof_tile_ssbo::RoofTileSsbo>; module=1375, inline_record=29368; 不是独立调用边。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::impl$10::new_storage_buffer<country_core::resources::roof_tile_ssbo::RoofTileSsbo>; module=1375, inline_record=29424; 不是独立调用边。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::impl$10::new_storage_buffer<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>; module=1375, inline_record=33064; 不是独立调用边。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::impl$10::new_storage_buffer<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>; module=1375, inline_record=33120; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::impl$11::new::closure$1
// 信息 ID: 1f7a918f8e322c16eded6fec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::render::sparse_instance_buffer::impl$11::new", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::impl$10::new_storage_buffer<country_core::geometry::instanced_wall::BrickTransformSsbo>; module=1375, inline_record=25376; 不是独立调用边。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::impl$10::new_storage_buffer<country_core::resources::roof_tile_ssbo::RoofTileSsbo>; module=1375, inline_record=29080; 不是独立调用边。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::impl$10::new_storage_buffer<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>; module=1375, inline_record=32776; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::impl$12::as_any<country_core::geometry::instanced_wall::BrickTransformSsbo>
// 信息 ID: 58fb6be3754a9946be81e4f1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::sparse_instance_buffer::impl$12::as_any -> ["country_core::geometry::instanced_wall::BrickTransformSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::impl$12::as_any<country_core::resources::roof_tile_ssbo::RoofTileSsbo>
// 信息 ID: b5737316b6b4e93e6b21ebd1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::sparse_instance_buffer::impl$12::as_any -> ["country_core::resources::roof_tile_ssbo::RoofTileSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::impl$12::as_any<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>
// 信息 ID: 6412825f2b9ca71e63e73196
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::sparse_instance_buffer::impl$12::as_any -> ["country_core::resources::roofs::roof_buffers::WoodenPlankSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::impl$12::as_any_mut<country_core::geometry::instanced_wall::BrickTransformSsbo>
// 信息 ID: ef063ab35496e4b71acde21a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::sparse_instance_buffer::impl$12::as_any_mut -> ["country_core::geometry::instanced_wall::BrickTransformSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::impl$12::as_any_mut<country_core::resources::roof_tile_ssbo::RoofTileSsbo>
// 信息 ID: e0785fffd631e4493f0ffee3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::sparse_instance_buffer::impl$12::as_any_mut -> ["country_core::resources::roof_tile_ssbo::RoofTileSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::impl$12::as_any_mut<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>
// 信息 ID: ee74c59d76d0af4fccc05667
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::sparse_instance_buffer::impl$12::as_any_mut -> ["country_core::resources::roofs::roof_buffers::WoodenPlankSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::impl$12::handle<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>
// 信息 ID: 88ec92b36d71de5f247b6393
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::sparse_instance_buffer::impl$12::handle -> ["country_core::resources::roofs::roof_buffers::WoodenPlankSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::impl$2::drain_chunk_bitmap::closure$0
// 信息 ID: 4fd4e19bc733ddc89646dae1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::sparse_instance_buffer::impl$2::drain_chunk_bitmap", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush<country_core::resources::roof_tile_ssbo::RoofTileSsbo>; module=751, inline_record=7220; 不是独立调用边。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>; module=1214, inline_record=7240; 不是独立调用边。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush<country_core::geometry::instanced_wall::BrickTransformSsbo>; module=1375, inline_record=9096; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::impl$2::drain_chunk_bitmap::closure$0::closure$0
// 信息 ID: 20af63df2c43ceb4e2d7e4a3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::sparse_instance_buffer::impl$2::drain_chunk_bitmap", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush<country_core::resources::roof_tile_ssbo::RoofTileSsbo>; module=751, inline_record=7016; 不是独立调用边。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>; module=1214, inline_record=7036; 不是独立调用边。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::gpu_flush<country_core::geometry::instanced_wall::BrickTransformSsbo>; module=1375, inline_record=8892; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::impl$2::iter_chunk_bitmap::closure$0
// 信息 ID: b8c18dbb056ca60e7462d007
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::sparse_instance_buffer::impl$2::iter_chunk_bitmap", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::impl$3::to_draw_multi_instanced<country_core::geometry::instanced_wall::BrickTransformSsbo>; module=1375, inline_record=19104; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::impl$2::iter_chunk_bitmap::closure$0::closure$0
// 信息 ID: 7258e91b8e730b41f49b07ba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::sparse_instance_buffer::impl$2::iter_chunk_bitmap", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::sparse_instance_buffer::impl$3::to_draw_multi_instanced<country_core::geometry::instanced_wall::BrickTransformSsbo>; module=1375, inline_record=18900; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
// 信息 ID: 91d4526fd75c7dec08403c75
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::sparse_instance_buffer::impl$2::write_allocation", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 32 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_roof::visual::assemble_roof_tiles::assemble_roof_tiles; module=266, inline_record=8460; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments::closure$1; module=360, inline_record=15140; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments::closure$1; module=360, inline_record=15888; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_arches::construct_arches; module=361, inline_record=12332; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::construct_lintels; module=363, inline_record=10556; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_gates::systems::construct_gates; module=366, inline_record=14780; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::gpu_clear_wall_gate_pieces; module=368, inline_record=3076; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::place_pillars::place_pillars; module=369, inline_record=15848; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::impl$3::to_draw_multi_instanced<country_core::geometry::instanced_wall::BrickTransformSsbo>
// 信息 ID: 6f11e02e22d9dc9f4f518f76
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::sparse_instance_buffer::impl$3::to_draw_multi_instanced -> ["country_core::geometry::instanced_wall::BrickTransformSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists @ 0x611a42 -> 0xf5bfa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists @ 0x611aaf -> 0xf5bfa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists_pass2 @ 0x612524 -> 0xf5bfa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists_pass2 @ 0x612598 -> 0xf5bfa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external<country_core::resources::roof_tile_ssbo::RoofTileSsbo,country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketRoofTile> @ 0x68fe52 -> 0xf5bfa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket_external<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo,country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketWoodenPlank> @ 0x690412 -> 0xf5bfa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::impl$4::drop
// 信息 ID: f03528c919a2b8aae6cd9772
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 13 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::geometry::instanced_wall::BrickTransformSsbo> >; module=234, inline_record=235604; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >; module=500, inline_record=211168; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >; module=738, inline_record=25920; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::geometry::instanced_wall::BrickTransformSsbo> >; module=738, inline_record=33680; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::geometry::instanced_wall::BrickTransformSsbo> >; module=877, inline_record=28672; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::geometry::instanced_wall::BrickTransformSsbo> >; module=886, inline_record=41540; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::geometry::instanced_wall::BrickTransformSsbo> >; module=895, inline_record=21228; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo> >; module=895, inline_record=24212; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::sparse_instance_buffer::impl$5::drop
// 信息 ID: 8e961f96adc0b4a09780e686
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 64 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_roof::visual::assemble_roof_tiles::assemble_roof_tiles; module=266, inline_record=8616; 不是独立调用边。
// [PDB 内联宿主] system_roof::visual::assemble_roof_tiles::assemble_roof_tiles; module=266, inline_record=9936; 不是独立调用边。
// [PDB 内联宿主] system_roof::visual::assemble_roof_tiles::assemble_roof_tiles; module=266, inline_record=10116; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments::closure$1; module=360, inline_record=16044; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments::closure$1; module=360, inline_record=16228; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments::closure$1; module=360, inline_record=16412; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments::closure$1; module=360, inline_record=16592; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_arches::construct_arches; module=361, inline_record=29900; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
