// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/restore_archived_decorators.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 71d8477ef7999750cfcd47fbba911a6c0d20097652459b6dcb01a5b8af33b5a4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: restore_archived_decorators.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 381, 1276
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::restore_archived_decorators::overwrite_decorator_state_from_archivist
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":381,"record_offset":1320,"rva":5233952,"va":5373943072}
// 编译实例 RVAs: 0x4fdd20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140012e10
//     owner=unresolved_namespace; count=1
//     target_aliases=359 candidates; complete list: restore_archived_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5158382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373943072,"raw_hex":"e8124ab1ff","size":5,"soft_fail":false,"target_va":5368786448,"va":5373944825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400507f0
//     owner=unresolved_namespace; count=1
//     target_aliases=36 candidates; complete list: restore_archived_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4905102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373943072,"raw_hex":"e87227b5ff","size":5,"soft_fail":false,"target_va":5369038832,"va":5373943929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=2
//     target_aliases=30 candidates; complete list: restore_archived_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4326005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373943072,"raw_hex":"e88bfdbdff","size":5,"soft_fail":false,"target_va":5369618032,"va":5373944032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bab90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: restore_archived_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t773358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373943072,"raw_hex":"e8eecc0b00","size":5,"soft_fail":false,"target_va":5374716816,"va":5373943453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cf9e80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: restore_archived_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8371853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373943072,"raw_hex":"e88dbe7f00","size":5,"soft_fail":false,"target_va":5382315648,"va":5373943790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cfaeb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: restore_archived_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8375249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373943072,"raw_hex":"e8d1cb7f00","size":5,"soft_fail":false,"target_va":5382319792,"va":5373944538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420c8eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: restore_archived_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29141145","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373943072,"raw_hex":"e899a8bc01","size":5,"soft_fail":false,"target_va":5403086512,"va":5373945362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: restore_archived_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29649749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373943072,"raw_hex":"e8556bc401","size":5,"soft_fail":false,"target_va":5403593392,"va":5373943638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_archivist::DecoratorArchivist::all_decorators
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t769368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373943072,"raw_hex":"e858bd0b00","size":5,"soft_fail":false,"target_va":5374712544,"va":5373943171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::get_or_insert_default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t773974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373943072,"raw_hex":"e856cf0b00","size":5,"soft_fail":false,"target_va":5374717584,"va":5373943605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::get
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1342459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373943072,"raw_hex":"e8fb7b1400","size":5,"soft_fail":false,"target_va":5375285936,"va":5373943472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":381,"record_offset":1520,"parent_record_offset":1320,"ancestor_record_offset":1320,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":381,"record_offset":2772,"parent_record_offset":1320,"ancestor_record_offset":1320,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::impl$111::clone
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":381,"record_offset":7144,"parent_record_offset":1320,"ancestor_record_offset":1320,"inlinee":21686,"source_line":1157,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":381,"record_offset":2828,"parent_record_offset":1320,"ancestor_record_offset":1320,"inlinee":17320,"source_line":710,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":381,"record_offset":4728,"parent_record_offset":1320,"ancestor_record_offset":1320,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":381,"record_offset":3484,"parent_record_offset":1320,"ancestor_record_offset":1320,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":381,"record_offset":4088,"parent_record_offset":1320,"ancestor_record_offset":1320,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$80::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":381,"record_offset":3188,"parent_record_offset":1320,"ancestor_record_offset":1320,"inlinee":18503,"source_line":3143,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":381,"record_offset":7172,"parent_record_offset":7144,"ancestor_record_offset":1320,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":381,"record_offset":6928,"parent_record_offset":6904,"ancestor_record_offset":1320,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":381,"record_offset":2372,"parent_record_offset":2348,"ancestor_record_offset":1320,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":381,"record_offset":1988,"parent_record_offset":1956,"ancestor_record_offset":1320,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":381,"record_offset":6396,"parent_record_offset":6372,"ancestor_record_offset":1320,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":381,"record_offset":2052,"parent_record_offset":2016,"ancestor_record_offset":1320,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::BuildHasher::hash_one
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":381,"record_offset":4832,"parent_record_offset":4808,"ancestor_record_offset":1320,"inlinee":8489,"source_line":696,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":381,"record_offset":5060,"parent_record_offset":5036,"ancestor_record_offset":1320,"inlinee":8461,"source_line":395,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$13::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":381,"record_offset":5036,"parent_record_offset":5012,"ancestor_record_offset":1320,"inlinee":6472,"source_line":806,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":381,"record_offset":4988,"parent_record_offset":4832,"ancestor_record_offset":1320,"inlinee":5832,"source_line":941,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::Hasher::new_with_keys
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":381,"record_offset":4916,"parent_record_offset":4888,"ancestor_record_offset":1320,"inlinee":8426,"source_line":190,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::Hasher::reset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":381,"record_offset":4944,"parent_record_offset":4916,"ancestor_record_offset":1320,"inlinee":8427,"source_line":205,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::SipHasher13::new_with_keys
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":381,"record_offset":4888,"parent_record_offset":4864,"ancestor_record_offset":1320,"inlinee":8424,"source_line":183,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":381,"record_offset":5212,"parent_record_offset":5188,"ancestor_record_offset":1320,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":381,"record_offset":5140,"parent_record_offset":5116,"ancestor_record_offset":1320,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":381,"record_offset":5236,"parent_record_offset":5212,"ancestor_record_offset":1320,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":381,"record_offset":5284,"parent_record_offset":5236,"ancestor_record_offset":1320,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":381,"record_offset":5676,"parent_record_offset":5236,"ancestor_record_offset":1320,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":381,"record_offset":6420,"parent_record_offset":6396,"ancestor_record_offset":1320,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":381,"record_offset":3096,"parent_record_offset":3060,"ancestor_record_offset":1320,"inlinee":8557,"source_line":3145,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":381,"record_offset":1548,"parent_record_offset":1520,"ancestor_record_offset":1320,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":381,"record_offset":2512,"parent_record_offset":2488,"ancestor_record_offset":1320,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":381,"record_offset":3360,"parent_record_offset":3336,"ancestor_record_offset":1320,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":381,"record_offset":1636,"parent_record_offset":1612,"ancestor_record_offset":1320,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":381,"record_offset":3336,"parent_record_offset":3220,"ancestor_record_offset":1320,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":381,"record_offset":2660,"parent_record_offset":2636,"ancestor_record_offset":1320,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":381,"record_offset":5084,"parent_record_offset":5060,"ancestor_record_offset":1320,"inlinee":8462,"source_line":2859,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":381,"record_offset":5384,"parent_record_offset":5284,"ancestor_record_offset":1320,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":381,"record_offset":5352,"parent_record_offset":5284,"ancestor_record_offset":1320,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":381,"record_offset":1888,"parent_record_offset":1864,"ancestor_record_offset":1320,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":381,"record_offset":1612,"parent_record_offset":1548,"ancestor_record_offset":1320,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":381,"record_offset":2324,"parent_record_offset":2300,"ancestor_record_offset":1320,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":381,"record_offset":1584,"parent_record_offset":1548,"ancestor_record_offset":1320,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":381,"record_offset":2420,"parent_record_offset":1548,"ancestor_record_offset":1320,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":381,"record_offset":2448,"parent_record_offset":2420,"ancestor_record_offset":1320,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":381,"record_offset":2300,"parent_record_offset":1636,"ancestor_record_offset":1320,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":381,"record_offset":2488,"parent_record_offset":1520,"ancestor_record_offset":1320,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":381,"record_offset":2636,"parent_record_offset":2612,"ancestor_record_offset":1320,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$18::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":381,"record_offset":2612,"parent_record_offset":2588,"ancestor_record_offset":1320,"inlinee":8605,"source_line":2242,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$23::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":381,"record_offset":2588,"parent_record_offset":2564,"ancestor_record_offset":1320,"inlinee":21682,"source_line":2282,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":381,"record_offset":2172,"parent_record_offset":1740,"ancestor_record_offset":1320,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":381,"record_offset":2904,"parent_record_offset":2876,"ancestor_record_offset":1320,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":381,"record_offset":2136,"parent_record_offset":2112,"ancestor_record_offset":1320,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":381,"record_offset":2112,"parent_record_offset":2088,"ancestor_record_offset":1320,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":381,"record_offset":2684,"parent_record_offset":2660,"ancestor_record_offset":1320,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":381,"record_offset":7200,"parent_record_offset":7172,"ancestor_record_offset":1320,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":381,"record_offset":7224,"parent_record_offset":7200,"ancestor_record_offset":1320,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$22::hash
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":381,"record_offset":5012,"parent_record_offset":4988,"ancestor_record_offset":1320,"inlinee":21684,"source_line":84,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$25::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":381,"record_offset":6952,"parent_record_offset":6928,"ancestor_record_offset":1320,"inlinee":21651,"source_line":84,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_archivist::impl$0::all_decorators::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_archivist.rs
//     evidence={"module":381,"record_offset":2348,"parent_record_offset":2324,"ancestor_record_offset":1320,"inlinee":21680,"source_line":217,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":381,"record_offset":6904,"parent_record_offset":6880,"ancestor_record_offset":1320,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::clear
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":381,"record_offset":2852,"parent_record_offset":2828,"ancestor_record_offset":1320,"inlinee":14997,"source_line":1011,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":381,"record_offset":4752,"parent_record_offset":4728,"ancestor_record_offset":1320,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":381,"record_offset":4776,"parent_record_offset":4752,"ancestor_record_offset":1320,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":381,"record_offset":3508,"parent_record_offset":3484,"ancestor_record_offset":1320,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":381,"record_offset":3256,"parent_record_offset":3220,"ancestor_record_offset":1320,"inlinee":6552,"source_line":1105,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":381,"record_offset":6880,"parent_record_offset":6660,"ancestor_record_offset":1320,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::fold
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":381,"record_offset":3408,"parent_record_offset":3384,"ancestor_record_offset":1320,"inlinee":16170,"source_line":4742,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":381,"record_offset":1684,"parent_record_offset":1660,"ancestor_record_offset":1320,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$94::extend
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":381,"record_offset":3220,"parent_record_offset":3188,"ancestor_record_offset":1320,"inlinee":18288,"source_line":6499,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":381,"record_offset":4808,"parent_record_offset":4776,"ancestor_record_offset":1320,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":381,"record_offset":6792,"parent_record_offset":6768,"ancestor_record_offset":1320,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":381,"record_offset":6768,"parent_record_offset":6660,"ancestor_record_offset":1320,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":381,"record_offset":6708,"parent_record_offset":6684,"ancestor_record_offset":1320,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":381,"record_offset":2088,"parent_record_offset":1740,"ancestor_record_offset":1320,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":381,"record_offset":7088,"parent_record_offset":6300,"ancestor_record_offset":1320,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":381,"record_offset":3640,"parent_record_offset":3556,"ancestor_record_offset":1320,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":381,"record_offset":1740,"parent_record_offset":1708,"ancestor_record_offset":1320,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":381,"record_offset":6684,"parent_record_offset":6660,"ancestor_record_offset":1320,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::clear
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":381,"record_offset":2876,"parent_record_offset":2852,"ancestor_record_offset":1320,"inlinee":14998,"source_line":1092,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::clear_no_drop
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":381,"record_offset":2976,"parent_record_offset":2952,"ancestor_record_offset":1320,"inlinee":15001,"source_line":1086,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":381,"record_offset":6276,"parent_record_offset":6252,"ancestor_record_offset":1320,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":381,"record_offset":6252,"parent_record_offset":4776,"ancestor_record_offset":1320,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":381,"record_offset":3532,"parent_record_offset":3508,"ancestor_record_offset":1320,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":381,"record_offset":3292,"parent_record_offset":3256,"ancestor_record_offset":1320,"inlinee":6553,"source_line":1175,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":381,"record_offset":3612,"parent_record_offset":3556,"ancestor_record_offset":1320,"inlinee":6423,"source_line":2806,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::clear_no_drop
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":381,"record_offset":3000,"parent_record_offset":2976,"ancestor_record_offset":1320,"inlinee":15002,"source_line":3337,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::ctrl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":381,"record_offset":3060,"parent_record_offset":3000,"ancestor_record_offset":1320,"inlinee":11753,"source_line":2799,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":381,"record_offset":3584,"parent_record_offset":3556,"ancestor_record_offset":1320,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":381,"record_offset":6300,"parent_record_offset":6276,"ancestor_record_offset":1320,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":381,"record_offset":3032,"parent_record_offset":3000,"ancestor_record_offset":1320,"inlinee":6230,"source_line":2827,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":381,"record_offset":3556,"parent_record_offset":3532,"ancestor_record_offset":1320,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":381,"record_offset":1828,"parent_record_offset":1792,"ancestor_record_offset":1320,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":381,"record_offset":1864,"parent_record_offset":1828,"ancestor_record_offset":1320,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":381,"record_offset":1924,"parent_record_offset":1792,"ancestor_record_offset":1320,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":381,"record_offset":1792,"parent_record_offset":1740,"ancestor_record_offset":1320,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bucket_mask_to_capacity
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":381,"record_offset":3128,"parent_record_offset":3000,"ancestor_record_offset":1320,"inlinee":11754,"source_line":222,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":381,"record_offset":6344,"parent_record_offset":6300,"ancestor_record_offset":1320,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::fold
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":381,"record_offset":3432,"parent_record_offset":3408,"ancestor_record_offset":1320,"inlinee":16171,"source_line":4151,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":381,"record_offset":1708,"parent_record_offset":1684,"ancestor_record_offset":1320,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::clear::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":381,"record_offset":2952,"parent_record_offset":2928,"ancestor_record_offset":1320,"inlinee":15000,"source_line":1098,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":381,"record_offset":6660,"parent_record_offset":6300,"ancestor_record_offset":1320,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":381,"record_offset":6372,"parent_record_offset":6300,"ancestor_record_offset":1320,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":381,"record_offset":1956,"parent_record_offset":1740,"ancestor_record_offset":1320,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":381,"record_offset":6456,"parent_record_offset":6300,"ancestor_record_offset":1320,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":381,"record_offset":6996,"parent_record_offset":6300,"ancestor_record_offset":1320,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":381,"record_offset":2016,"parent_record_offset":1956,"ancestor_record_offset":1320,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_full
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":381,"record_offset":7432,"parent_record_offset":7376,"ancestor_record_offset":1320,"inlinee":11826,"source_line":120,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::scopeguard::impl$3::drop
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\scopeguard.rs
//     evidence={"module":381,"record_offset":2928,"parent_record_offset":2904,"ancestor_record_offset":1320,"inlinee":8805,"source_line":69,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":381,"record_offset":3384,"parent_record_offset":3360,"ancestor_record_offset":1320,"inlinee":16169,"source_line":2245,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":381,"record_offset":1660,"parent_record_offset":1636,"ancestor_record_offset":1320,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$1::build_hasher
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":381,"record_offset":4864,"parent_record_offset":4832,"ancestor_record_offset":1320,"inlinee":8422,"source_line":84,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":381,"record_offset":5188,"parent_record_offset":4832,"ancestor_record_offset":1320,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":381,"record_offset":5116,"parent_record_offset":5060,"ancestor_record_offset":1320,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::restore_archived_decorators::restore_archived_decorators
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":381,"record_offset":256,"rva":5233680,"va":5373942800}
// 编译实例 RVAs: 0x4fdc10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402bce80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: restore_archived_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2363021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373942800,"raw_hex":"e873f1dbff","size":5,"soft_fail":false,"target_va":5371580032,"va":5373943048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_archivist::DecoratorArchivist::extract_diff
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t773004","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373942800,"raw_hex":"e88ccb0b00","size":5,"soft_fail":false,"target_va":5374716032,"va":5373943023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::restore_archived_decorators::overwrite_decorator_state_from_archivist
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373942800,"raw_hex":"e8a2000000","size":5,"soft_fail":false,"target_va":5373943072,"va":5373942905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":381,"record_offset":428,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":381,"record_offset":600,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::last
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":381,"record_offset":900,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":7511,"source_line":782,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":381,"record_offset":1172,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":381,"record_offset":456,"parent_record_offset":428,"ancestor_record_offset":256,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":381,"record_offset":720,"parent_record_offset":684,"ancestor_record_offset":256,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":381,"record_offset":684,"parent_record_offset":636,"ancestor_record_offset":256,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::last
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":381,"record_offset":940,"parent_record_offset":900,"ancestor_record_offset":256,"inlinee":7512,"source_line":867,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":381,"record_offset":636,"parent_record_offset":600,"ancestor_record_offset":256,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":381,"record_offset":980,"parent_record_offset":940,"ancestor_record_offset":256,"inlinee":8099,"source_line":143,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":381,"record_offset":764,"parent_record_offset":720,"ancestor_record_offset":256,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":381,"record_offset":1008,"parent_record_offset":980,"ancestor_record_offset":256,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":381,"record_offset":828,"parent_record_offset":720,"ancestor_record_offset":256,"inlinee":4156,"source_line":1003,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":381,"record_offset":1068,"parent_record_offset":940,"ancestor_record_offset":256,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":381,"record_offset":1196,"parent_record_offset":1172,"ancestor_record_offset":256,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":381,"record_offset":1268,"parent_record_offset":1244,"ancestor_record_offset":256,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/00381.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::restore_archived_decorators::overwrite_decorator_state_from_archivist::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 103
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEB22
// 首条位置: {"module":1276,"inlinee":60194}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$52::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1276,"record_offset":91996,"parent_record_offset":91976,"ancestor_record_offset":91976,"inlinee":59537,"source_line":188,"archive":"_pdb/A/modules/01276.jsonl.gz"}
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
// 完整结构及原始类型记录: restore_archived_decorators.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: restore_archived_decorators.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: restore_archived_decorators.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] country_core::resources::walls::WallId (*)(ref$<country_core::resources::walls::WallId>)
// 候选参数类型: ["ref$<country_core::resources::walls::WallId>"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>", "bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>

// ------------------------------------------------------------------
// 函数: system_decorator::restore_archived_decorators::overwrite_decorator_state_from_archivist
// 信息 ID: 70b3ecf7e15e32482e96d1cf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::restore_archived_decorators::restore_archived_decorators @ 0x4fdc79 -> 0x4fdd20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::deferred_decorator_history::decorator_record_history @ 0x17f2e06 -> 0x4fdd20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::restore_archived_decorators::overwrite_decorator_state_from_archivist::closure$0
// 信息 ID: 1c13bec8fbb327541863836c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::restore_archived_decorators::overwrite_decorator_state_from_archivist", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::restore_archived_decorators::overwrite_decorator_state_from_archivist::closure_env$0 <- hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,hashbrown::map::impl$60::fold::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,tuple$<>,system_decorator::restore_archived_decorators::overwrite_decorator_state_from_archivist::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,system_decorator::restore_archived_decorators::overwrite_decorator_state_from_archivist::closure_env$0> > > > >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::restore_archived_decorators::overwrite_decorator_state_from_archivist::closure_env$0 <- hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,hashbrown::map::impl$60::fold::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,tuple$<>,system_decorator::restore_archived_decorators::overwrite_decorator_state_from_archivist::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,system_decorator::restore_archived_decorators::overwrite_decorator_state_from_archivist::closure_env$0> > > > >,tuple$<> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] hashbrown::raw::RawIterRange::fold_impl<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,hashbrown::map::impl$60::fold::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> >,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,tuple$<>,system_decorator::restore_archived_decorators::overwrite_decorator_state_from_archivist::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,hashbrown::map::impl$94::extend::closure_env$0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorInfo>,system_decorator::restore_archived_decorators::overwrite_decorator_state_from_archivist::closure_env$0> > > > >,tuple$<> >; module=1276, inline_record=91976; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::restore_archived_decorators::restore_archived_decorators
// 信息 ID: cc59477e6b9eade7cca8f468
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 919f6612252861062d0099bd: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>)> @ 0x76558 -> 0x4fdc10 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>)> > @ 0xa6d8c -> 0x4fdc10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
