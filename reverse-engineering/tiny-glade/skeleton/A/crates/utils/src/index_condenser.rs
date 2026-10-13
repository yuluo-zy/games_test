// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/utils/src/index_condenser.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: ddee69e15855c872b553aeb84f33947bb62b895508fb7b2ba947ec47fb33ff2e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: index_condenser.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 410, 455, 497, 1371
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::index_condenser::impl$6::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5BCF
// 首条位置: {"module":410,"inlinee":23503}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::index_condenser::impl$6::fmt<country_core::render::nani::gpu_world::InstanceKey>
// 记录形式: compiled_procedure; instances=2
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":455,"record_offset":8076,"rva":6433936,"va":5375143056}
// 编译实例 RVAs: 0x622c90, 0xf37a20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field2_finish
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2441039","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375143056,"raw_hex":"e84f3f2500","size":5,"soft_fail":false,"target_va":5377584208,"va":5375143164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::index_condenser::impl$3::default
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 83
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5BF5
// 首条位置: {"module":1371,"inlinee":23541}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1371,"record_offset":43432,"parent_record_offset":43408,"ancestor_record_offset":43408,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/01371.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1371,"record_offset":43704,"parent_record_offset":43676,"ancestor_record_offset":43408,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/01371.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1371,"record_offset":43756,"parent_record_offset":43732,"ancestor_record_offset":43408,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01371.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1371,"record_offset":43732,"parent_record_offset":43676,"ancestor_record_offset":43408,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01371.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1371,"record_offset":43780,"parent_record_offset":43756,"ancestor_record_offset":43408,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01371.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1371,"record_offset":43632,"parent_record_offset":43608,"ancestor_record_offset":43408,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01371.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1371,"record_offset":43804,"parent_record_offset":43780,"ancestor_record_offset":43408,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01371.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1371,"record_offset":43480,"parent_record_offset":43456,"ancestor_record_offset":43408,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/01371.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1371,"record_offset":43556,"parent_record_offset":43528,"ancestor_record_offset":43408,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/01371.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1371,"record_offset":43676,"parent_record_offset":43528,"ancestor_record_offset":43408,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/01371.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1371,"record_offset":43456,"parent_record_offset":43432,"ancestor_record_offset":43408,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/01371.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1371,"record_offset":43580,"parent_record_offset":43556,"ancestor_record_offset":43408,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/01371.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":1371,"record_offset":43608,"parent_record_offset":43580,"ancestor_record_offset":43408,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/01371.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1371,"record_offset":43528,"parent_record_offset":43504,"ancestor_record_offset":43408,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/01371.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1371,"record_offset":43504,"parent_record_offset":43480,"ancestor_record_offset":43408,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/01371.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::index_condenser::IndexCondenser::get
// 记录形式: inline_instance, inlinee_declaration; instances=16
// 源码行: 96
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5BD4
// 首条位置: {"module":410,"inlinee":23508}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cell::Cell::set
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":410,"record_offset":6072,"parent_record_offset":5120,"ancestor_record_offset":5120,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":410,"record_offset":6044,"parent_record_offset":5120,"ancestor_record_offset":5120,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":410,"record_offset":5160,"parent_record_offset":5120,"ancestor_record_offset":5120,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::nani::gpu_world::impl$38::eq
//     owner=private_source_ownership_candidate; count=13
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":410,"record_offset":5216,"parent_record_offset":5120,"ancestor_record_offset":5120,"inlinee":23506,"source_line":632,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":410,"record_offset":5244,"parent_record_offset":5120,"ancestor_record_offset":5120,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":410,"record_offset":6096,"parent_record_offset":6072,"ancestor_record_offset":5120,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":410,"record_offset":5828,"parent_record_offset":5804,"ancestor_record_offset":5120,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":410,"record_offset":5492,"parent_record_offset":5468,"ancestor_record_offset":5120,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":410,"record_offset":5576,"parent_record_offset":5552,"ancestor_record_offset":5120,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":410,"record_offset":5516,"parent_record_offset":5492,"ancestor_record_offset":5120,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":410,"record_offset":6120,"parent_record_offset":6096,"ancestor_record_offset":5120,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":410,"record_offset":5688,"parent_record_offset":5664,"ancestor_record_offset":5120,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":410,"record_offset":6144,"parent_record_offset":6120,"ancestor_record_offset":5120,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":410,"record_offset":5184,"parent_record_offset":5160,"ancestor_record_offset":5120,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::impl$38::eq
//     owner=private_source_ownership_candidate; count=13
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":410,"record_offset":5852,"parent_record_offset":5828,"ancestor_record_offset":5120,"inlinee":23506,"source_line":632,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":410,"record_offset":5804,"parent_record_offset":5780,"ancestor_record_offset":5120,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":410,"record_offset":5268,"parent_record_offset":5244,"ancestor_record_offset":5120,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":410,"record_offset":5292,"parent_record_offset":5268,"ancestor_record_offset":5120,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":410,"record_offset":5780,"parent_record_offset":5756,"ancestor_record_offset":5120,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":410,"record_offset":5320,"parent_record_offset":5292,"ancestor_record_offset":5120,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":5988,"parent_record_offset":5396,"ancestor_record_offset":5120,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":5372,"parent_record_offset":5348,"ancestor_record_offset":5120,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":5348,"parent_record_offset":5292,"ancestor_record_offset":5120,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":5396,"parent_record_offset":5372,"ancestor_record_offset":5120,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":410,"record_offset":5636,"parent_record_offset":5608,"ancestor_record_offset":5120,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":410,"record_offset":5664,"parent_record_offset":5636,"ancestor_record_offset":5120,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":410,"record_offset":5724,"parent_record_offset":5608,"ancestor_record_offset":5120,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":410,"record_offset":5608,"parent_record_offset":5396,"ancestor_record_offset":5120,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":5440,"parent_record_offset":5396,"ancestor_record_offset":5120,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":5756,"parent_record_offset":5396,"ancestor_record_offset":5120,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":410,"record_offset":5468,"parent_record_offset":5396,"ancestor_record_offset":5120,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=26
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":410,"record_offset":5552,"parent_record_offset":5396,"ancestor_record_offset":5120,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":410,"record_offset":5896,"parent_record_offset":5396,"ancestor_record_offset":5120,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::index_condenser::IndexCondenser::insert<country_core::render::nani::gpu_world::InstanceKey>
// 记录形式: compiled_procedure; instances=1
// 源码行: 126
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":410,"record_offset":3748,"rva":5688928,"va":5374398048}
// 编译实例 RVAs: 0x56ce60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dce30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: index_condenser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4784226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374398048,"raw_hex":"e89effb6ff","size":5,"soft_fail":false,"target_va":5369613872,"va":5374398093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f301b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: index_condenser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10236479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374398048,"raw_hex":"e83f329c00","size":5,"soft_fail":false,"target_va":5384634800,"va":5374398316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-3089861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374398048,"raw_hex":"e83bdad0ff","size":5,"soft_fail":false,"target_va":5371308432,"va":5374398288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":4764,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":410,"record_offset":4736,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":410,"record_offset":3944,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":410,"record_offset":4884,"parent_record_offset":3748,"ancestor_record_offset":3748,"inlinee":8309,"source_line":1105,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":410,"record_offset":4820,"parent_record_offset":4796,"ancestor_record_offset":3748,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":4796,"parent_record_offset":4764,"ancestor_record_offset":3748,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":410,"record_offset":4520,"parent_record_offset":4496,"ancestor_record_offset":3748,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":410,"record_offset":4192,"parent_record_offset":4168,"ancestor_record_offset":3748,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":410,"record_offset":4276,"parent_record_offset":4252,"ancestor_record_offset":3748,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":410,"record_offset":4216,"parent_record_offset":4192,"ancestor_record_offset":3748,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":410,"record_offset":4380,"parent_record_offset":4356,"ancestor_record_offset":3748,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":410,"record_offset":4852,"parent_record_offset":4764,"ancestor_record_offset":3748,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::impl$38::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":410,"record_offset":4544,"parent_record_offset":4520,"ancestor_record_offset":3748,"inlinee":23506,"source_line":632,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":410,"record_offset":4496,"parent_record_offset":4472,"ancestor_record_offset":3748,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":410,"record_offset":3968,"parent_record_offset":3944,"ancestor_record_offset":3748,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":410,"record_offset":3992,"parent_record_offset":3968,"ancestor_record_offset":3748,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":410,"record_offset":4472,"parent_record_offset":4448,"ancestor_record_offset":3748,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":410,"record_offset":4020,"parent_record_offset":3992,"ancestor_record_offset":3748,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":4680,"parent_record_offset":4096,"ancestor_record_offset":3748,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":4072,"parent_record_offset":4048,"ancestor_record_offset":3748,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":4048,"parent_record_offset":3992,"ancestor_record_offset":3748,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":4096,"parent_record_offset":4072,"ancestor_record_offset":3748,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":410,"record_offset":4332,"parent_record_offset":4308,"ancestor_record_offset":3748,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":410,"record_offset":4356,"parent_record_offset":4332,"ancestor_record_offset":3748,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":410,"record_offset":4416,"parent_record_offset":4308,"ancestor_record_offset":3748,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":410,"record_offset":4308,"parent_record_offset":4096,"ancestor_record_offset":3748,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":4140,"parent_record_offset":4096,"ancestor_record_offset":3748,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":4448,"parent_record_offset":4096,"ancestor_record_offset":3748,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":410,"record_offset":4168,"parent_record_offset":4096,"ancestor_record_offset":3748,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":410,"record_offset":4252,"parent_record_offset":4096,"ancestor_record_offset":3748,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":410,"record_offset":4588,"parent_record_offset":4096,"ancestor_record_offset":3748,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::index_condenser::IndexCondenser::remove<country_core::render::nani::gpu_world::InstanceKey>
// 记录形式: compiled_procedure; instances=1
// 源码行: 145
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":410,"record_offset":4916,"rva":5689232,"va":5374398352}
// 编译实例 RVAs: 0x56cf90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dce30
//     owner=unresolved_namespace; count=3
//     target_aliases=4 candidates; complete list: index_condenser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4784549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374398352,"raw_hex":"e85bfeb6ff","size":5,"soft_fail":false,"target_va":5369613872,"va":5374398416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df2d00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: index_condenser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8936488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374398352,"raw_hex":"e8285c8800","size":5,"soft_fail":false,"target_va":5383335168,"va":5374398675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec$LT$T$C$A$GT$::swap_remove::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28579238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374398352,"raw_hex":"e8a615b401","size":5,"soft_fail":false,"target_va":5402978176,"va":5374398933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28579960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374398352,"raw_hex":"e87818b401","size":5,"soft_fail":false,"target_va":5402978880,"va":5374398915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::swap_remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":6188,"parent_record_offset":4916,"ancestor_record_offset":4916,"inlinee":9718,"source_line":1487,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":410,"record_offset":7348,"parent_record_offset":4916,"ancestor_record_offset":4916,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":410,"record_offset":6472,"parent_record_offset":4916,"ancestor_record_offset":4916,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":410,"record_offset":6536,"parent_record_offset":4916,"ancestor_record_offset":4916,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":410,"record_offset":6356,"parent_record_offset":4916,"ancestor_record_offset":4916,"inlinee":7345,"source_line":1160,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] utils::index_condenser::IndexCondenser::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/index_condenser.rs
//     evidence={"module":410,"record_offset":5120,"parent_record_offset":4916,"ancestor_record_offset":4916,"inlinee":23508,"source_line":96,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":410,"record_offset":6260,"parent_record_offset":6228,"ancestor_record_offset":4916,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":6228,"parent_record_offset":6188,"ancestor_record_offset":4916,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":6324,"parent_record_offset":6188,"ancestor_record_offset":4916,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":410,"record_offset":6096,"parent_record_offset":6072,"ancestor_record_offset":4916,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":410,"record_offset":6072,"parent_record_offset":5120,"ancestor_record_offset":4916,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":410,"record_offset":5828,"parent_record_offset":5804,"ancestor_record_offset":4916,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":410,"record_offset":5492,"parent_record_offset":5468,"ancestor_record_offset":4916,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":410,"record_offset":5576,"parent_record_offset":5552,"ancestor_record_offset":4916,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":410,"record_offset":6292,"parent_record_offset":6188,"ancestor_record_offset":4916,"inlinee":5999,"source_line":3064,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":410,"record_offset":5516,"parent_record_offset":5492,"ancestor_record_offset":4916,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":410,"record_offset":6120,"parent_record_offset":6096,"ancestor_record_offset":4916,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":410,"record_offset":5688,"parent_record_offset":5664,"ancestor_record_offset":4916,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":410,"record_offset":6044,"parent_record_offset":5120,"ancestor_record_offset":4916,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":410,"record_offset":6144,"parent_record_offset":6120,"ancestor_record_offset":4916,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":410,"record_offset":5160,"parent_record_offset":5120,"ancestor_record_offset":4916,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":410,"record_offset":5184,"parent_record_offset":5160,"ancestor_record_offset":4916,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::gpu_world::impl$38::eq
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/gpu_world.rs
//     evidence={"module":410,"record_offset":5216,"parent_record_offset":5120,"ancestor_record_offset":4916,"inlinee":23506,"source_line":632,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":410,"record_offset":5804,"parent_record_offset":5780,"ancestor_record_offset":4916,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":410,"record_offset":5268,"parent_record_offset":5244,"ancestor_record_offset":4916,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":410,"record_offset":5292,"parent_record_offset":5268,"ancestor_record_offset":4916,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":410,"record_offset":6592,"parent_record_offset":6564,"ancestor_record_offset":4916,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":410,"record_offset":6564,"parent_record_offset":6536,"ancestor_record_offset":4916,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":410,"record_offset":6380,"parent_record_offset":6356,"ancestor_record_offset":4916,"inlinee":7346,"source_line":1895,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":410,"record_offset":6404,"parent_record_offset":6380,"ancestor_record_offset":4916,"inlinee":7347,"source_line":1934,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":410,"record_offset":5780,"parent_record_offset":5756,"ancestor_record_offset":4916,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":410,"record_offset":5320,"parent_record_offset":5292,"ancestor_record_offset":4916,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":5988,"parent_record_offset":5396,"ancestor_record_offset":4916,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":5372,"parent_record_offset":5348,"ancestor_record_offset":4916,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":5348,"parent_record_offset":5292,"ancestor_record_offset":4916,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":6652,"parent_record_offset":6592,"ancestor_record_offset":4916,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":5396,"parent_record_offset":5372,"ancestor_record_offset":4916,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":410,"record_offset":5636,"parent_record_offset":5608,"ancestor_record_offset":4916,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":410,"record_offset":5664,"parent_record_offset":5636,"ancestor_record_offset":4916,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":410,"record_offset":5724,"parent_record_offset":5608,"ancestor_record_offset":4916,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":410,"record_offset":5608,"parent_record_offset":5396,"ancestor_record_offset":4916,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":5440,"parent_record_offset":5396,"ancestor_record_offset":4916,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":410,"record_offset":5756,"parent_record_offset":5396,"ancestor_record_offset":4916,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":410,"record_offset":5468,"parent_record_offset":5396,"ancestor_record_offset":4916,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":410,"record_offset":5552,"parent_record_offset":5396,"ancestor_record_offset":4916,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":410,"record_offset":5896,"parent_record_offset":5396,"ancestor_record_offset":4916,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":410,"record_offset":5244,"parent_record_offset":5120,"ancestor_record_offset":4916,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00410.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::index_condenser::IndexCondenser::clear
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 172
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5BF1
// 首条位置: {"module":455,"inlinee":23537}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":27556,"parent_record_offset":27128,"ancestor_record_offset":27128,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":497,"record_offset":27160,"parent_record_offset":27128,"ancestor_record_offset":27128,"inlinee":17320,"source_line":710,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":497,"record_offset":27492,"parent_record_offset":27456,"ancestor_record_offset":27128,"inlinee":8557,"source_line":3145,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":497,"record_offset":27252,"parent_record_offset":27216,"ancestor_record_offset":27128,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::clear
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":497,"record_offset":27188,"parent_record_offset":27160,"ancestor_record_offset":27128,"inlinee":14997,"source_line":1011,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::clear
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":497,"record_offset":27216,"parent_record_offset":27188,"ancestor_record_offset":27128,"inlinee":14998,"source_line":1092,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::clear_no_drop
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":497,"record_offset":27336,"parent_record_offset":27308,"ancestor_record_offset":27128,"inlinee":15001,"source_line":1086,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::clear_no_drop
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":497,"record_offset":27364,"parent_record_offset":27336,"ancestor_record_offset":27128,"inlinee":15002,"source_line":3337,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::ctrl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":497,"record_offset":27456,"parent_record_offset":27364,"ancestor_record_offset":27128,"inlinee":11753,"source_line":2799,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":497,"record_offset":27428,"parent_record_offset":27364,"ancestor_record_offset":27128,"inlinee":6230,"source_line":2827,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bucket_mask_to_capacity
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":497,"record_offset":27400,"parent_record_offset":27364,"ancestor_record_offset":27128,"inlinee":11754,"source_line":222,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::clear::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":497,"record_offset":27308,"parent_record_offset":27280,"ancestor_record_offset":27128,"inlinee":15000,"source_line":1098,"archive":"_pdb/A/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::scopeguard::impl$3::drop
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\scopeguard.rs
//     evidence={"module":497,"record_offset":27280,"parent_record_offset":27252,"ancestor_record_offset":27128,"inlinee":8805,"source_line":69,"archive":"_pdb/A/modules/00497.jsonl.gz"}
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
// 完整结构及原始类型记录: index_condenser.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::index_condenser::Condex
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "utils::index_condenser::Condex<country_core::render::nani::gpu_world::InstanceKey>", "constructor": "utils::index_condenser::Condex", "arguments": ["country_core::render::nani::gpu_world::InstanceKey"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$51::fmt<utils::index_condenser::Condex<country_core::render::nani::gpu_world::InstanceKey> >", "constructor": "core::fmt::impl$51::fmt", "arguments": ["utils::index_condenser::Condex<country_core::render::nani::gpu_world::InstanceKey>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<utils::index_condenser::Condex<country_core::render::nani::gpu_world::InstanceKey> >", "constructor": "core::ptr::drop_in_place", "arguments": ["utils::index_condenser::Condex<country_core::render::nani::gpu_world::InstanceKey>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::index_condenser::IndexCondenser
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 5; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "utils::index_condenser::IndexCondenser<country_core::render::nani::gpu_world::InstanceKey>", "constructor": "utils::index_condenser::IndexCondenser", "arguments": ["country_core::render::nani::gpu_world::InstanceKey"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<utils::index_condenser::IndexCondenser<country_core::render::nani::gpu_world::InstanceKey> >", "constructor": "core::ptr::drop_in_place", "arguments": ["utils::index_condenser::IndexCondenser<country_core::render::nani::gpu_world::InstanceKey>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::index_condenser::IndexCondenser::insert<country_core::render::nani::gpu_world::InstanceKey>", "constructor": "utils::index_condenser::IndexCondenser::insert", "arguments": ["country_core::render::nani::gpu_world::InstanceKey"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::index_condenser::IndexCondenser::remove<country_core::render::nani::gpu_world::InstanceKey>", "constructor": "utils::index_condenser::IndexCondenser::remove", "arguments": ["country_core::render::nani::gpu_world::InstanceKey"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: index_condenser.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: index_condenser.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: utils::index_condenser::IndexCondenser::clear
// 信息 ID: e5b816e80cc52e3f739c8e09
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::clearing::uninit_clearing; module=497, inline_record=27128; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::index_condenser::IndexCondenser::get
// 信息 ID: a6696ad2a0ee31554b563477
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 13 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::index_condenser::IndexCondenser::remove<country_core::render::nani::gpu_world::InstanceKey>; module=410, inline_record=5120; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::update_instance_mesh; module=455, inline_record=10632; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::update_instance_data<country_core::render::nani::instance_data::SolidVertexColorInstanceData>; module=1371, inline_record=15668; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::update_instance_data<country_core::render::nani::instance_data::SolidVertexColorBricksInstanceData>; module=1371, inline_record=18080; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::update_instance_data<country_core::resources::placement_data::PlacementDataSsbo>; module=1371, inline_record=20476; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::update_instance_data<country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData>; module=1371, inline_record=22888; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::update_instance_data<country_core::render::nani::instance_data::PlantInstanceData>; module=1371, inline_record=25284; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::update_instance_data<country_core::render::nani::instance_data::WrenInstanceData>; module=1371, inline_record=27680; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::index_condenser::IndexCondenser::insert<country_core::render::nani::gpu_world::InstanceKey>
// 信息 ID: 1886bc371c5a1ff7a56804bd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::index_condenser::IndexCondenser::insert -> ["country_core::render::nani::gpu_world::InstanceKey"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::extend_no_data @ 0x623959 -> 0x56ce60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::gpu_world::GpuWorld::create_instances<country_core::resources::placement_data::PlacementDataSsbo,country_core::systems::prefab::create_prefab_gpu_instances::closure_env$0> @ 0xf37b74 -> 0x56ce60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::index_condenser::IndexCondenser::remove<country_core::render::nani::gpu_world::InstanceKey>
// 信息 ID: 38394883f9ea9c59e321a774
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::index_condenser::IndexCondenser::remove -> ["country_core::render::nani::gpu_world::InstanceKey"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::nani::gpu_world::GpuWorld::remove_instance @ 0x62308f -> 0x56cf90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::index_condenser::impl$3::default
// 信息 ID: d835ca3918b34963cc86c0e9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::new<country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData>; module=1371, inline_record=43408; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::new<country_core::render::nani::instance_data::WrenInstanceData>; module=1371, inline_record=45528; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::new<country_core::render::nani::instance_data::StoneFloorInstanceData>; module=1371, inline_record=47656; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::new<country_core::render::nani::instance_data::SolidVertexColorInstanceData>; module=1371, inline_record=49788; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::new<country_core::render::nani::instance_data::PlantInstanceData>; module=1371, inline_record=51912; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::new<country_core::render::nani::instance_data::SolidVertexColorBricksInstanceData>; module=1371, inline_record=54052; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::new<country_core::resources::placement_data::PlacementDataSsbo>; module=1371, inline_record=56208; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::new<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData>; module=1371, inline_record=58352; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::index_condenser::impl$6::fmt
// 信息 ID: 04b240fb61f40cf8c065d3b6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$51::fmt<utils::index_condenser::Condex<country_core::render::nani::gpu_world::InstanceKey> >; module=410, inline_record=640; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::index_condenser::impl$6::fmt<country_core::render::nani::gpu_world::InstanceKey>
// 信息 ID: 172eb862a2036cf870f73c59
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::index_condenser::impl$6::fmt -> ["country_core::render::nani::gpu_world::InstanceKey"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
