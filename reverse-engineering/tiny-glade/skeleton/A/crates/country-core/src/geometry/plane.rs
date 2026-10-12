// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/geometry/plane.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 251ed0f3809f9912a982914d3e76677dd9f3359c159994ab793a737274f0e45e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: plane.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 414, 1365
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
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::geometry::plane::impl$1::from
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1365,"record_offset":11580,"rva":15854368,"va":5384563488}
// 编译实例 RVAs: 0xf1eb20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: plane.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11939177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384563488,"raw_hex":"e897d249ff","size":5,"soft_fail":false,"target_va":5372625024,"va":5384564196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: plane.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11813997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384563488,"raw_hex":"e893bb4bff","size":5,"soft_fail":false,"target_va":5372749696,"va":5384563688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: plane.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11815087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384563488,"raw_hex":"e851b74bff","size":5,"soft_fail":false,"target_va":5372749712,"va":5384564794,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bf6e0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: plane.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9828249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384563488,"raw_hex":"e867086aff","size":5,"soft_fail":false,"target_va":5374736096,"va":5384564340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bfc80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plane.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9826951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384563488,"raw_hex":"e8790d6aff","size":5,"soft_fail":false,"target_va":5374737536,"va":5384564482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e4670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plane.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8629073","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384563488,"raw_hex":"e8af547cff","size":5,"soft_fail":false,"target_va":5375936112,"va":5384565180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d72c40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plane.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1753864","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384563488,"raw_hex":"e8f83ce5ff","size":5,"soft_fail":false,"target_va":5382810688,"va":5384564547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18413455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384563488,"raw_hex":"e88ff71801","size":5,"soft_fail":false,"target_va":5402978144,"va":5384564684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-13257786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384563488,"raw_hex":"e8c6b335ff","size":5,"soft_fail":false,"target_va":5371306160,"va":5384563941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-7197934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384563488,"raw_hex":"e8122b92ff","size":5,"soft_fail":false,"target_va":5377366224,"va":5384564153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1365,"record_offset":11732,"parent_record_offset":11580,"ancestor_record_offset":11580,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1365,"record_offset":11884,"parent_record_offset":11580,"ancestor_record_offset":11580,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1365,"record_offset":11968,"parent_record_offset":11580,"ancestor_record_offset":11580,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1365,"record_offset":12964,"parent_record_offset":11580,"ancestor_record_offset":11580,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1365,"record_offset":12100,"parent_record_offset":11580,"ancestor_record_offset":11580,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1365,"record_offset":12440,"parent_record_offset":11580,"ancestor_record_offset":11580,"inlinee":24287,"source_line":24,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_indices
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1365,"record_offset":12936,"parent_record_offset":11580,"ancestor_record_offset":11580,"inlinee":24560,"source_line":48,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1365,"record_offset":11788,"parent_record_offset":11764,"ancestor_record_offset":11580,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1365,"record_offset":11812,"parent_record_offset":11788,"ancestor_record_offset":11580,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1365,"record_offset":13140,"parent_record_offset":13116,"ancestor_record_offset":11580,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1365,"record_offset":11764,"parent_record_offset":11732,"ancestor_record_offset":11580,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1365,"record_offset":13116,"parent_record_offset":13016,"ancestor_record_offset":11580,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1365,"record_offset":13048,"parent_record_offset":13016,"ancestor_record_offset":11580,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1365,"record_offset":12036,"parent_record_offset":12012,"ancestor_record_offset":11580,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1365,"record_offset":13016,"parent_record_offset":12988,"ancestor_record_offset":11580,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1365,"record_offset":12012,"parent_record_offset":11968,"ancestor_record_offset":11580,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1365,"record_offset":12764,"parent_record_offset":12736,"ancestor_record_offset":11580,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1365,"record_offset":12816,"parent_record_offset":12792,"ancestor_record_offset":11580,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1365,"record_offset":12792,"parent_record_offset":12736,"ancestor_record_offset":11580,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1365,"record_offset":12840,"parent_record_offset":12816,"ancestor_record_offset":11580,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1365,"record_offset":13080,"parent_record_offset":13048,"ancestor_record_offset":11580,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1365,"record_offset":12692,"parent_record_offset":12668,"ancestor_record_offset":11580,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1365,"record_offset":12988,"parent_record_offset":12964,"ancestor_record_offset":11580,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1365,"record_offset":12132,"parent_record_offset":12100,"ancestor_record_offset":11580,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1365,"record_offset":11840,"parent_record_offset":11812,"ancestor_record_offset":11580,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1365,"record_offset":12068,"parent_record_offset":11968,"ancestor_record_offset":11580,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1365,"record_offset":12464,"parent_record_offset":12440,"ancestor_record_offset":11580,"inlinee":22684,"source_line":11,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1365,"record_offset":12492,"parent_record_offset":12464,"ancestor_record_offset":11580,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1365,"record_offset":12540,"parent_record_offset":12516,"ancestor_record_offset":11580,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1365,"record_offset":12616,"parent_record_offset":12588,"ancestor_record_offset":11580,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1365,"record_offset":12736,"parent_record_offset":12588,"ancestor_record_offset":11580,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1365,"record_offset":12516,"parent_record_offset":12492,"ancestor_record_offset":11580,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1365,"record_offset":12640,"parent_record_offset":12616,"ancestor_record_offset":11580,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":1365,"record_offset":12668,"parent_record_offset":12640,"ancestor_record_offset":11580,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1365,"record_offset":12588,"parent_record_offset":12564,"ancestor_record_offset":11580,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1365,"record_offset":12564,"parent_record_offset":12540,"ancestor_record_offset":11580,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/01365.jsonl.gz"}
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
// 完整结构及原始类型记录: plane.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: plane.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: plane.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: country_core::geometry::plane::impl$1::from
// 信息 ID: 321da8c9279c53c0bff38314
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::startup_critters::startup_critters @ 0x66eb48 -> 0xf1eb20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
