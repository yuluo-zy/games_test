// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/render/shadowed_buffer.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 5e095de03013beb71113afd2d775fbd4ef88650324bc833338e10ec93acf0e95
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: shadowed_buffer.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 451, 455, 1371, 1450
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::render::nani::buffers::GpuInstance>
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1450,"record_offset":9004,"rva":17882592,"va":5386591712}
// 编译实例 RVAs: 0x110dde0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13842164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386591712,"raw_hex":"e80cc92cff","size":5,"soft_fail":false,"target_va":5372749712,"va":5386591871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da7a0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13842096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386591712,"raw_hex":"e850c92cff","size":5,"soft_fail":false,"target_va":5372749728,"va":5386591819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10352081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386591712,"raw_hex":"e82f0a62ff","size":5,"soft_fail":false,"target_va":5376239920,"va":5386591996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f84b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1610479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386591712,"raw_hex":"e8116de7ff","size":5,"soft_fail":false,"target_va":5384981296,"va":5386591770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16386295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386591712,"raw_hex":"e8f708fa00","size":5,"soft_fail":false,"target_va":5402978144,"va":5386591844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1450,"record_offset":9200,"parent_record_offset":9004,"ancestor_record_offset":9004,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1450,"record_offset":9800,"parent_record_offset":9004,"ancestor_record_offset":9004,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":9576,"parent_record_offset":9552,"ancestor_record_offset":9004,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":9552,"parent_record_offset":9332,"ancestor_record_offset":9004,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":9468,"parent_record_offset":9332,"ancestor_record_offset":9004,"inlinee":8396,"source_line":283,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::realloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":9492,"parent_record_offset":9468,"ancestor_record_offset":9004,"inlinee":8397,"source_line":137,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":9376,"parent_record_offset":9332,"ancestor_record_offset":9004,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":9332,"parent_record_offset":9308,"ancestor_record_offset":9004,"inlinee":8395,"source_line":486,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":9308,"parent_record_offset":9284,"ancestor_record_offset":9004,"inlinee":8394,"source_line":406,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":9612,"parent_record_offset":9308,"ancestor_record_offset":9004,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":9696,"parent_record_offset":9672,"ancestor_record_offset":9004,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::into_boxed_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1450,"record_offset":9256,"parent_record_offset":9228,"ancestor_record_offset":9004,"inlinee":8392,"source_line":1139,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1450,"record_offset":9284,"parent_record_offset":9256,"ancestor_record_offset":9004,"inlinee":8393,"source_line":1077,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$38::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1450,"record_offset":9228,"parent_record_offset":9200,"ancestor_record_offset":9004,"inlinee":8407,"source_line":3414,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1450,"record_offset":9400,"parent_record_offset":9376,"ancestor_record_offset":9004,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1450,"record_offset":9648,"parent_record_offset":9256,"ancestor_record_offset":9004,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1450,"record_offset":9524,"parent_record_offset":9332,"ancestor_record_offset":9004,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::render::nani::instance_data::GothicWindowBricksInstanceData>
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1450,"record_offset":6700,"rva":17881808,"va":5386590928}
// 编译实例 RVAs: 0x110dad0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13841377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386590928,"raw_hex":"e81fcc2cff","size":5,"soft_fail":false,"target_va":5372749712,"va":5386591084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da7a0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13841309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386590928,"raw_hex":"e863cc2cff","size":5,"soft_fail":false,"target_va":5372749728,"va":5386591032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10351281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386590928,"raw_hex":"e84f0d62ff","size":5,"soft_fail":false,"target_va":5376239920,"va":5386591196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f85f20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1604591","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386590928,"raw_hex":"e81184e7ff","size":5,"soft_fail":false,"target_va":5384986400,"va":5386590986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16387082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386590928,"raw_hex":"e80a0cfa00","size":5,"soft_fail":false,"target_va":5402978144,"va":5386591057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1450,"record_offset":6924,"parent_record_offset":6700,"ancestor_record_offset":6700,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1450,"record_offset":7524,"parent_record_offset":6700,"ancestor_record_offset":6700,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":7300,"parent_record_offset":7276,"ancestor_record_offset":6700,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":7276,"parent_record_offset":7056,"ancestor_record_offset":6700,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":7192,"parent_record_offset":7056,"ancestor_record_offset":6700,"inlinee":8396,"source_line":283,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::realloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":7216,"parent_record_offset":7192,"ancestor_record_offset":6700,"inlinee":8397,"source_line":137,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":7100,"parent_record_offset":7056,"ancestor_record_offset":6700,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":7056,"parent_record_offset":7032,"ancestor_record_offset":6700,"inlinee":8395,"source_line":486,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":7032,"parent_record_offset":7008,"ancestor_record_offset":6700,"inlinee":8394,"source_line":406,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":7336,"parent_record_offset":7032,"ancestor_record_offset":6700,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":7420,"parent_record_offset":7396,"ancestor_record_offset":6700,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::into_boxed_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1450,"record_offset":6980,"parent_record_offset":6952,"ancestor_record_offset":6700,"inlinee":8392,"source_line":1139,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1450,"record_offset":7008,"parent_record_offset":6980,"ancestor_record_offset":6700,"inlinee":8393,"source_line":1077,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$38::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1450,"record_offset":6952,"parent_record_offset":6924,"ancestor_record_offset":6700,"inlinee":8407,"source_line":3414,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1450,"record_offset":7124,"parent_record_offset":7100,"ancestor_record_offset":6700,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1450,"record_offset":7372,"parent_record_offset":6980,"ancestor_record_offset":6700,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1450,"record_offset":7248,"parent_record_offset":7056,"ancestor_record_offset":6700,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::render::nani::instance_data::PlantInstanceData>
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1450,"record_offset":12408,"rva":17883792,"va":5386592912}
// 编译实例 RVAs: 0x110e290
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13843372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386592912,"raw_hex":"e854c42cff","size":5,"soft_fail":false,"target_va":5372749712,"va":5386593079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da7a0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13843304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386592912,"raw_hex":"e898c42cff","size":5,"soft_fail":false,"target_va":5372749728,"va":5386593027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10353281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386592912,"raw_hex":"e87f0562ff","size":5,"soft_fail":false,"target_va":5376239920,"va":5386593196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f84ef0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1610719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386592912,"raw_hex":"e8216ce7ff","size":5,"soft_fail":false,"target_va":5384982256,"va":5386592970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16385087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386592912,"raw_hex":"e83f04fa00","size":5,"soft_fail":false,"target_va":5402978144,"va":5386593052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1450,"record_offset":12616,"parent_record_offset":12408,"ancestor_record_offset":12408,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1450,"record_offset":13216,"parent_record_offset":12408,"ancestor_record_offset":12408,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":12992,"parent_record_offset":12968,"ancestor_record_offset":12408,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":12968,"parent_record_offset":12748,"ancestor_record_offset":12408,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":12884,"parent_record_offset":12748,"ancestor_record_offset":12408,"inlinee":8396,"source_line":283,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::realloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":12908,"parent_record_offset":12884,"ancestor_record_offset":12408,"inlinee":8397,"source_line":137,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":12792,"parent_record_offset":12748,"ancestor_record_offset":12408,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":12748,"parent_record_offset":12724,"ancestor_record_offset":12408,"inlinee":8395,"source_line":486,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":12724,"parent_record_offset":12700,"ancestor_record_offset":12408,"inlinee":8394,"source_line":406,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":13028,"parent_record_offset":12724,"ancestor_record_offset":12408,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":13112,"parent_record_offset":13088,"ancestor_record_offset":12408,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::into_boxed_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1450,"record_offset":12672,"parent_record_offset":12644,"ancestor_record_offset":12408,"inlinee":8392,"source_line":1139,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1450,"record_offset":12700,"parent_record_offset":12672,"ancestor_record_offset":12408,"inlinee":8393,"source_line":1077,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$38::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1450,"record_offset":12644,"parent_record_offset":12616,"ancestor_record_offset":12408,"inlinee":8407,"source_line":3414,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1450,"record_offset":12816,"parent_record_offset":12792,"ancestor_record_offset":12408,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1450,"record_offset":13064,"parent_record_offset":12672,"ancestor_record_offset":12408,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1450,"record_offset":12940,"parent_record_offset":12748,"ancestor_record_offset":12408,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::render::nani::instance_data::PreplacedFlowerInstanceData>
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1450,"record_offset":11264,"rva":17883392,"va":5386592512}
// 编译实例 RVAs: 0x110e100
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13842972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386592512,"raw_hex":"e8e4c52cff","size":5,"soft_fail":false,"target_va":5372749712,"va":5386592679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da7a0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13842904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386592512,"raw_hex":"e828c62cff","size":5,"soft_fail":false,"target_va":5372749728,"va":5386592627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10352881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386592512,"raw_hex":"e80f0762ff","size":5,"soft_fail":false,"target_va":5376239920,"va":5386592796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f84780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1612223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386592512,"raw_hex":"e84166e7ff","size":5,"soft_fail":false,"target_va":5384980352,"va":5386592570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16385487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386592512,"raw_hex":"e8cf05fa00","size":5,"soft_fail":false,"target_va":5402978144,"va":5386592652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1450,"record_offset":11484,"parent_record_offset":11264,"ancestor_record_offset":11264,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1450,"record_offset":12084,"parent_record_offset":11264,"ancestor_record_offset":11264,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":11860,"parent_record_offset":11836,"ancestor_record_offset":11264,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":11836,"parent_record_offset":11616,"ancestor_record_offset":11264,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":11752,"parent_record_offset":11616,"ancestor_record_offset":11264,"inlinee":8396,"source_line":283,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::realloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":11776,"parent_record_offset":11752,"ancestor_record_offset":11264,"inlinee":8397,"source_line":137,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":11660,"parent_record_offset":11616,"ancestor_record_offset":11264,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":11616,"parent_record_offset":11592,"ancestor_record_offset":11264,"inlinee":8395,"source_line":486,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":11592,"parent_record_offset":11568,"ancestor_record_offset":11264,"inlinee":8394,"source_line":406,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":11896,"parent_record_offset":11592,"ancestor_record_offset":11264,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":11980,"parent_record_offset":11956,"ancestor_record_offset":11264,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::into_boxed_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1450,"record_offset":11540,"parent_record_offset":11512,"ancestor_record_offset":11264,"inlinee":8392,"source_line":1139,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1450,"record_offset":11568,"parent_record_offset":11540,"ancestor_record_offset":11264,"inlinee":8393,"source_line":1077,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$38::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1450,"record_offset":11512,"parent_record_offset":11484,"ancestor_record_offset":11264,"inlinee":8407,"source_line":3414,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1450,"record_offset":11684,"parent_record_offset":11660,"ancestor_record_offset":11264,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1450,"record_offset":11932,"parent_record_offset":11540,"ancestor_record_offset":11264,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1450,"record_offset":11808,"parent_record_offset":11616,"ancestor_record_offset":11264,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::render::nani::instance_data::SolidAlbedoMapInstanceData>
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1450,"record_offset":15824,"rva":17884992,"va":5386594112}
// 编译实例 RVAs: 0x110e740
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13844564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386594112,"raw_hex":"e8acbf2cff","size":5,"soft_fail":false,"target_va":5372749712,"va":5386594271,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da7a0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13844496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386594112,"raw_hex":"e8f0bf2cff","size":5,"soft_fail":false,"target_va":5372749728,"va":5386594219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10354481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386594112,"raw_hex":"e8cf0062ff","size":5,"soft_fail":false,"target_va":5376239920,"va":5386594396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f862d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1606831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386594112,"raw_hex":"e8517be7ff","size":5,"soft_fail":false,"target_va":5384987344,"va":5386594170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16383895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386594112,"raw_hex":"e897fff900","size":5,"soft_fail":false,"target_va":5402978144,"va":5386594244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1450,"record_offset":16044,"parent_record_offset":15824,"ancestor_record_offset":15824,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1450,"record_offset":16644,"parent_record_offset":15824,"ancestor_record_offset":15824,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":16420,"parent_record_offset":16396,"ancestor_record_offset":15824,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":16396,"parent_record_offset":16176,"ancestor_record_offset":15824,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":16312,"parent_record_offset":16176,"ancestor_record_offset":15824,"inlinee":8396,"source_line":283,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::realloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":16336,"parent_record_offset":16312,"ancestor_record_offset":15824,"inlinee":8397,"source_line":137,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":16220,"parent_record_offset":16176,"ancestor_record_offset":15824,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":16176,"parent_record_offset":16152,"ancestor_record_offset":15824,"inlinee":8395,"source_line":486,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":16152,"parent_record_offset":16128,"ancestor_record_offset":15824,"inlinee":8394,"source_line":406,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":16456,"parent_record_offset":16152,"ancestor_record_offset":15824,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":16540,"parent_record_offset":16516,"ancestor_record_offset":15824,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::into_boxed_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1450,"record_offset":16100,"parent_record_offset":16072,"ancestor_record_offset":15824,"inlinee":8392,"source_line":1139,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1450,"record_offset":16128,"parent_record_offset":16100,"ancestor_record_offset":15824,"inlinee":8393,"source_line":1077,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$38::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1450,"record_offset":16072,"parent_record_offset":16044,"ancestor_record_offset":15824,"inlinee":8407,"source_line":3414,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1450,"record_offset":16244,"parent_record_offset":16220,"ancestor_record_offset":15824,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1450,"record_offset":16492,"parent_record_offset":16100,"ancestor_record_offset":15824,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1450,"record_offset":16368,"parent_record_offset":16176,"ancestor_record_offset":15824,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::render::nani::instance_data::SolidVertexColorBricksInstanceData>
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1450,"record_offset":5548,"rva":17881408,"va":5386590528}
// 编译实例 RVAs: 0x110d940
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13840980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386590528,"raw_hex":"e8accd2cff","size":5,"soft_fail":false,"target_va":5372749712,"va":5386590687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da7a0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13840912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386590528,"raw_hex":"e8f0cd2cff","size":5,"soft_fail":false,"target_va":5372749728,"va":5386590635,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10350897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386590528,"raw_hex":"e8cf0e62ff","size":5,"soft_fail":false,"target_va":5376239920,"va":5386590812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f824b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1619151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386590528,"raw_hex":"e8314be7ff","size":5,"soft_fail":false,"target_va":5384971440,"va":5386590586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16387479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386590528,"raw_hex":"e8970dfa00","size":5,"soft_fail":false,"target_va":5402978144,"va":5386590660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1450,"record_offset":5776,"parent_record_offset":5548,"ancestor_record_offset":5548,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1450,"record_offset":6376,"parent_record_offset":5548,"ancestor_record_offset":5548,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":6152,"parent_record_offset":6128,"ancestor_record_offset":5548,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":6128,"parent_record_offset":5908,"ancestor_record_offset":5548,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":6044,"parent_record_offset":5908,"ancestor_record_offset":5548,"inlinee":8396,"source_line":283,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::realloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":6068,"parent_record_offset":6044,"ancestor_record_offset":5548,"inlinee":8397,"source_line":137,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":5952,"parent_record_offset":5908,"ancestor_record_offset":5548,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":5908,"parent_record_offset":5884,"ancestor_record_offset":5548,"inlinee":8395,"source_line":486,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":5884,"parent_record_offset":5860,"ancestor_record_offset":5548,"inlinee":8394,"source_line":406,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":6188,"parent_record_offset":5884,"ancestor_record_offset":5548,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":6272,"parent_record_offset":6248,"ancestor_record_offset":5548,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::into_boxed_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1450,"record_offset":5832,"parent_record_offset":5804,"ancestor_record_offset":5548,"inlinee":8392,"source_line":1139,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1450,"record_offset":5860,"parent_record_offset":5832,"ancestor_record_offset":5548,"inlinee":8393,"source_line":1077,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$38::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1450,"record_offset":5804,"parent_record_offset":5776,"ancestor_record_offset":5548,"inlinee":8407,"source_line":3414,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1450,"record_offset":5976,"parent_record_offset":5952,"ancestor_record_offset":5548,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1450,"record_offset":6224,"parent_record_offset":5832,"ancestor_record_offset":5548,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1450,"record_offset":6100,"parent_record_offset":5908,"ancestor_record_offset":5548,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::render::nani::instance_data::SolidVertexColorInstanceData>
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1450,"record_offset":16968,"rva":17885392,"va":5386594512}
// 编译实例 RVAs: 0x110e8d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13844964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386594512,"raw_hex":"e81cbe2cff","size":5,"soft_fail":false,"target_va":5372749712,"va":5386594671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da7a0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13844896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386594512,"raw_hex":"e860be2cff","size":5,"soft_fail":false,"target_va":5372749728,"va":5386594619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10354881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386594512,"raw_hex":"e83fff61ff","size":5,"soft_fail":false,"target_va":5376239920,"va":5386594796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f84d10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1612799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386594512,"raw_hex":"e80164e7ff","size":5,"soft_fail":false,"target_va":5384981776,"va":5386594570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16383495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386594512,"raw_hex":"e807fef900","size":5,"soft_fail":false,"target_va":5402978144,"va":5386594644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1450,"record_offset":17188,"parent_record_offset":16968,"ancestor_record_offset":16968,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1450,"record_offset":17788,"parent_record_offset":16968,"ancestor_record_offset":16968,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":17564,"parent_record_offset":17540,"ancestor_record_offset":16968,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":17540,"parent_record_offset":17320,"ancestor_record_offset":16968,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":17456,"parent_record_offset":17320,"ancestor_record_offset":16968,"inlinee":8396,"source_line":283,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::realloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":17480,"parent_record_offset":17456,"ancestor_record_offset":16968,"inlinee":8397,"source_line":137,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":17364,"parent_record_offset":17320,"ancestor_record_offset":16968,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":17320,"parent_record_offset":17296,"ancestor_record_offset":16968,"inlinee":8395,"source_line":486,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":17296,"parent_record_offset":17272,"ancestor_record_offset":16968,"inlinee":8394,"source_line":406,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":17600,"parent_record_offset":17296,"ancestor_record_offset":16968,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":17684,"parent_record_offset":17660,"ancestor_record_offset":16968,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::into_boxed_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1450,"record_offset":17244,"parent_record_offset":17216,"ancestor_record_offset":16968,"inlinee":8392,"source_line":1139,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1450,"record_offset":17272,"parent_record_offset":17244,"ancestor_record_offset":16968,"inlinee":8393,"source_line":1077,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$38::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1450,"record_offset":17216,"parent_record_offset":17188,"ancestor_record_offset":16968,"inlinee":8407,"source_line":3414,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1450,"record_offset":17388,"parent_record_offset":17364,"ancestor_record_offset":16968,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1450,"record_offset":17636,"parent_record_offset":17244,"ancestor_record_offset":16968,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1450,"record_offset":17512,"parent_record_offset":17320,"ancestor_record_offset":16968,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData>
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1450,"record_offset":7848,"rva":17882192,"va":5386591312}
// 编译实例 RVAs: 0x110dc50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13841764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386591312,"raw_hex":"e89cca2cff","size":5,"soft_fail":false,"target_va":5372749712,"va":5386591471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da7a0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13841696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386591312,"raw_hex":"e8e0ca2cff","size":5,"soft_fail":false,"target_va":5372749728,"va":5386591419,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10351681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386591312,"raw_hex":"e8bf0b62ff","size":5,"soft_fail":false,"target_va":5376239920,"va":5386591596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f83aa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1614319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386591312,"raw_hex":"e8115ee7ff","size":5,"soft_fail":false,"target_va":5384977056,"va":5386591370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16386695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386591312,"raw_hex":"e8870afa00","size":5,"soft_fail":false,"target_va":5402978144,"va":5386591444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1450,"record_offset":8080,"parent_record_offset":7848,"ancestor_record_offset":7848,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1450,"record_offset":8680,"parent_record_offset":7848,"ancestor_record_offset":7848,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":8456,"parent_record_offset":8432,"ancestor_record_offset":7848,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":8432,"parent_record_offset":8212,"ancestor_record_offset":7848,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":8348,"parent_record_offset":8212,"ancestor_record_offset":7848,"inlinee":8396,"source_line":283,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::realloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":8372,"parent_record_offset":8348,"ancestor_record_offset":7848,"inlinee":8397,"source_line":137,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":8256,"parent_record_offset":8212,"ancestor_record_offset":7848,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":8212,"parent_record_offset":8188,"ancestor_record_offset":7848,"inlinee":8395,"source_line":486,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":8188,"parent_record_offset":8164,"ancestor_record_offset":7848,"inlinee":8394,"source_line":406,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":8492,"parent_record_offset":8188,"ancestor_record_offset":7848,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":8576,"parent_record_offset":8552,"ancestor_record_offset":7848,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::into_boxed_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1450,"record_offset":8136,"parent_record_offset":8108,"ancestor_record_offset":7848,"inlinee":8392,"source_line":1139,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1450,"record_offset":8164,"parent_record_offset":8136,"ancestor_record_offset":7848,"inlinee":8393,"source_line":1077,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$38::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1450,"record_offset":8108,"parent_record_offset":8080,"ancestor_record_offset":7848,"inlinee":8407,"source_line":3414,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1450,"record_offset":8280,"parent_record_offset":8256,"ancestor_record_offset":7848,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1450,"record_offset":8528,"parent_record_offset":8136,"ancestor_record_offset":7848,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1450,"record_offset":8404,"parent_record_offset":8212,"ancestor_record_offset":7848,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData>
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1450,"record_offset":14672,"rva":17884592,"va":5386593712}
// 编译实例 RVAs: 0x110e5b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13844164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386593712,"raw_hex":"e83cc12cff","size":5,"soft_fail":false,"target_va":5372749712,"va":5386593871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da7a0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13844096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386593712,"raw_hex":"e880c12cff","size":5,"soft_fail":false,"target_va":5372749728,"va":5386593819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10354081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386593712,"raw_hex":"e85f0262ff","size":5,"soft_fail":false,"target_va":5376239920,"va":5386593996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f850d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1611039","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386593712,"raw_hex":"e8e16ae7ff","size":5,"soft_fail":false,"target_va":5384982736,"va":5386593770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16384295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386593712,"raw_hex":"e82701fa00","size":5,"soft_fail":false,"target_va":5402978144,"va":5386593844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1450,"record_offset":14900,"parent_record_offset":14672,"ancestor_record_offset":14672,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1450,"record_offset":15500,"parent_record_offset":14672,"ancestor_record_offset":14672,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":15276,"parent_record_offset":15252,"ancestor_record_offset":14672,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":15252,"parent_record_offset":15032,"ancestor_record_offset":14672,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":15168,"parent_record_offset":15032,"ancestor_record_offset":14672,"inlinee":8396,"source_line":283,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::realloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":15192,"parent_record_offset":15168,"ancestor_record_offset":14672,"inlinee":8397,"source_line":137,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":15076,"parent_record_offset":15032,"ancestor_record_offset":14672,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":15032,"parent_record_offset":15008,"ancestor_record_offset":14672,"inlinee":8395,"source_line":486,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":15008,"parent_record_offset":14984,"ancestor_record_offset":14672,"inlinee":8394,"source_line":406,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":15312,"parent_record_offset":15008,"ancestor_record_offset":14672,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":15396,"parent_record_offset":15372,"ancestor_record_offset":14672,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::into_boxed_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1450,"record_offset":14956,"parent_record_offset":14928,"ancestor_record_offset":14672,"inlinee":8392,"source_line":1139,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1450,"record_offset":14984,"parent_record_offset":14956,"ancestor_record_offset":14672,"inlinee":8393,"source_line":1077,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$38::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1450,"record_offset":14928,"parent_record_offset":14900,"ancestor_record_offset":14672,"inlinee":8407,"source_line":3414,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1450,"record_offset":15100,"parent_record_offset":15076,"ancestor_record_offset":14672,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1450,"record_offset":15348,"parent_record_offset":14956,"ancestor_record_offset":14672,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1450,"record_offset":15224,"parent_record_offset":15032,"ancestor_record_offset":14672,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::render::nani::instance_data::StoneFloorInstanceData>
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1450,"record_offset":10124,"rva":17882992,"va":5386592112}
// 编译实例 RVAs: 0x110df70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13842572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386592112,"raw_hex":"e874c72cff","size":5,"soft_fail":false,"target_va":5372749712,"va":5386592279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da7a0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13842504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386592112,"raw_hex":"e8b8c72cff","size":5,"soft_fail":false,"target_va":5372749728,"va":5386592227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10352481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386592112,"raw_hex":"e89f0862ff","size":5,"soft_fail":false,"target_va":5376239920,"va":5386592396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f82860
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1619791","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386592112,"raw_hex":"e8b148e7ff","size":5,"soft_fail":false,"target_va":5384972384,"va":5386592170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16385887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386592112,"raw_hex":"e85f07fa00","size":5,"soft_fail":false,"target_va":5402978144,"va":5386592252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1450,"record_offset":10340,"parent_record_offset":10124,"ancestor_record_offset":10124,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1450,"record_offset":10940,"parent_record_offset":10124,"ancestor_record_offset":10124,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":10716,"parent_record_offset":10692,"ancestor_record_offset":10124,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":10692,"parent_record_offset":10472,"ancestor_record_offset":10124,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":10608,"parent_record_offset":10472,"ancestor_record_offset":10124,"inlinee":8396,"source_line":283,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::realloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":10632,"parent_record_offset":10608,"ancestor_record_offset":10124,"inlinee":8397,"source_line":137,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":10516,"parent_record_offset":10472,"ancestor_record_offset":10124,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":10472,"parent_record_offset":10448,"ancestor_record_offset":10124,"inlinee":8395,"source_line":486,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":10448,"parent_record_offset":10424,"ancestor_record_offset":10124,"inlinee":8394,"source_line":406,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":10752,"parent_record_offset":10448,"ancestor_record_offset":10124,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":10836,"parent_record_offset":10812,"ancestor_record_offset":10124,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::into_boxed_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1450,"record_offset":10396,"parent_record_offset":10368,"ancestor_record_offset":10124,"inlinee":8392,"source_line":1139,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1450,"record_offset":10424,"parent_record_offset":10396,"ancestor_record_offset":10124,"inlinee":8393,"source_line":1077,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$38::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1450,"record_offset":10368,"parent_record_offset":10340,"ancestor_record_offset":10124,"inlinee":8407,"source_line":3414,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1450,"record_offset":10540,"parent_record_offset":10516,"ancestor_record_offset":10124,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1450,"record_offset":10788,"parent_record_offset":10396,"ancestor_record_offset":10124,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1450,"record_offset":10664,"parent_record_offset":10472,"ancestor_record_offset":10124,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::render::nani::instance_data::WrenInstanceData>
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1450,"record_offset":18112,"rva":17885792,"va":5386594912}
// 编译实例 RVAs: 0x110ea60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13845364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386594912,"raw_hex":"e88cbc2cff","size":5,"soft_fail":false,"target_va":5372749712,"va":5386595071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da7a0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13845296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386594912,"raw_hex":"e8d0bc2cff","size":5,"soft_fail":false,"target_va":5372749728,"va":5386595019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10355281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386594912,"raw_hex":"e8affd61ff","size":5,"soft_fail":false,"target_va":5376239920,"va":5386595196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f843d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1615567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386594912,"raw_hex":"e83159e7ff","size":5,"soft_fail":false,"target_va":5384979408,"va":5386594970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16383095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386594912,"raw_hex":"e877fcf900","size":5,"soft_fail":false,"target_va":5402978144,"va":5386595044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1450,"record_offset":18320,"parent_record_offset":18112,"ancestor_record_offset":18112,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1450,"record_offset":18920,"parent_record_offset":18112,"ancestor_record_offset":18112,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":18696,"parent_record_offset":18672,"ancestor_record_offset":18112,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":18672,"parent_record_offset":18452,"ancestor_record_offset":18112,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":18588,"parent_record_offset":18452,"ancestor_record_offset":18112,"inlinee":8396,"source_line":283,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::realloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":18612,"parent_record_offset":18588,"ancestor_record_offset":18112,"inlinee":8397,"source_line":137,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":18496,"parent_record_offset":18452,"ancestor_record_offset":18112,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":18452,"parent_record_offset":18428,"ancestor_record_offset":18112,"inlinee":8395,"source_line":486,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":18428,"parent_record_offset":18404,"ancestor_record_offset":18112,"inlinee":8394,"source_line":406,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":18732,"parent_record_offset":18428,"ancestor_record_offset":18112,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":18816,"parent_record_offset":18792,"ancestor_record_offset":18112,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::into_boxed_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1450,"record_offset":18376,"parent_record_offset":18348,"ancestor_record_offset":18112,"inlinee":8392,"source_line":1139,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1450,"record_offset":18404,"parent_record_offset":18376,"ancestor_record_offset":18112,"inlinee":8393,"source_line":1077,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$38::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1450,"record_offset":18348,"parent_record_offset":18320,"ancestor_record_offset":18112,"inlinee":8407,"source_line":3414,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1450,"record_offset":18520,"parent_record_offset":18496,"ancestor_record_offset":18112,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1450,"record_offset":18768,"parent_record_offset":18376,"ancestor_record_offset":18112,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1450,"record_offset":18644,"parent_record_offset":18452,"ancestor_record_offset":18112,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::resources::placement_data::PlacementDataSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1450,"record_offset":13540,"rva":17884192,"va":5386593312}
// 编译实例 RVAs: 0x110e420
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13843772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386593312,"raw_hex":"e8c4c22cff","size":5,"soft_fail":false,"target_va":5372749712,"va":5386593479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da7a0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13843704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386593312,"raw_hex":"e808c32cff","size":5,"soft_fail":false,"target_va":5372749728,"va":5386593427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10353681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386593312,"raw_hex":"e8ef0362ff","size":5,"soft_fail":false,"target_va":5376239920,"va":5386593596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f83c80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1615839","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386593312,"raw_hex":"e82158e7ff","size":5,"soft_fail":false,"target_va":5384977536,"va":5386593370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16384687","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386593312,"raw_hex":"e8af02fa00","size":5,"soft_fail":false,"target_va":5402978144,"va":5386593452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1450,"record_offset":13748,"parent_record_offset":13540,"ancestor_record_offset":13540,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1450,"record_offset":14348,"parent_record_offset":13540,"ancestor_record_offset":13540,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":14124,"parent_record_offset":14100,"ancestor_record_offset":13540,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":14100,"parent_record_offset":13880,"ancestor_record_offset":13540,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":14016,"parent_record_offset":13880,"ancestor_record_offset":13540,"inlinee":8396,"source_line":283,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::realloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1450,"record_offset":14040,"parent_record_offset":14016,"ancestor_record_offset":13540,"inlinee":8397,"source_line":137,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":13924,"parent_record_offset":13880,"ancestor_record_offset":13540,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::shrink
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":13880,"parent_record_offset":13856,"ancestor_record_offset":13540,"inlinee":8395,"source_line":486,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":13856,"parent_record_offset":13832,"ancestor_record_offset":13540,"inlinee":8394,"source_line":406,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":14160,"parent_record_offset":13856,"ancestor_record_offset":13540,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1450,"record_offset":14244,"parent_record_offset":14220,"ancestor_record_offset":13540,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::into_boxed_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1450,"record_offset":13804,"parent_record_offset":13776,"ancestor_record_offset":13540,"inlinee":8392,"source_line":1139,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::shrink_to_fit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1450,"record_offset":13832,"parent_record_offset":13804,"ancestor_record_offset":13540,"inlinee":8393,"source_line":1077,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$38::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1450,"record_offset":13776,"parent_record_offset":13748,"ancestor_record_offset":13540,"inlinee":8407,"source_line":3414,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1450,"record_offset":13948,"parent_record_offset":13924,"ancestor_record_offset":13540,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1450,"record_offset":14196,"parent_record_offset":13804,"ancestor_record_offset":13540,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1450,"record_offset":14072,"parent_record_offset":13880,"ancestor_record_offset":13540,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::shadowed_buffer::ShadowedBuffer::buffer
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6503
// 首条位置: {"module":451,"inlinee":25859}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::shadowed_buffer::ShadowedBuffer::write
// 记录形式: inline_instance, inlinee_declaration; instances=27
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6609
// 首条位置: {"module":455,"inlinee":26121}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::dirty_index_range::DirtyIndexRange::add
//     owner=private_source_ownership_candidate; count=14
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/dirty_index_range.rs
//     evidence={"module":455,"record_offset":13996,"parent_record_offset":13960,"ancestor_record_offset":13960,"inlinee":26105,"source_line":13,"archive":"_pdb/A/modules/00455.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_max
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":455,"record_offset":14080,"parent_record_offset":13996,"ancestor_record_offset":13960,"inlinee":26108,"source_line":2932,"archive":"_pdb/A/modules/00455.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_min
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":455,"record_offset":14024,"parent_record_offset":13996,"ancestor_record_offset":13960,"inlinee":26106,"source_line":2978,"archive":"_pdb/A/modules/00455.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_umax
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":455,"record_offset":14104,"parent_record_offset":14080,"ancestor_record_offset":13960,"inlinee":26109,"source_line":3538,"archive":"_pdb/A/modules/00455.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_umin
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":455,"record_offset":14048,"parent_record_offset":14024,"ancestor_record_offset":13960,"inlinee":26107,"source_line":3555,"archive":"_pdb/A/modules/00455.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::shadowed_buffer::ShadowedBuffer::get_mut
// 记录形式: inline_instance, inlinee_declaration; instances=14
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x65F7
// 首条位置: {"module":455,"inlinee":26103}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::dirty_index_range::DirtyIndexRange::add
//     owner=private_source_ownership_candidate; count=12
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/dirty_index_range.rs
//     evidence={"module":455,"record_offset":11732,"parent_record_offset":11700,"ancestor_record_offset":11700,"inlinee":26105,"source_line":13,"archive":"_pdb/A/modules/00455.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_max
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":455,"record_offset":11816,"parent_record_offset":11732,"ancestor_record_offset":11700,"inlinee":26108,"source_line":2932,"archive":"_pdb/A/modules/00455.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_min
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":455,"record_offset":11760,"parent_record_offset":11732,"ancestor_record_offset":11700,"inlinee":26106,"source_line":2978,"archive":"_pdb/A/modules/00455.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_umax
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":455,"record_offset":11840,"parent_record_offset":11816,"ancestor_record_offset":11700,"inlinee":26109,"source_line":3538,"archive":"_pdb/A/modules/00455.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_umin
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":455,"record_offset":11784,"parent_record_offset":11760,"ancestor_record_offset":11700,"inlinee":26107,"source_line":3555,"archive":"_pdb/A/modules/00455.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::shadowed_buffer::ShadowedBuffer::copy
// 记录形式: inline_instance, inlinee_declaration; instances=18
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x65FF
// 首条位置: {"module":455,"inlinee":26111}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::dirty_index_range::DirtyIndexRange::add
//     owner=private_source_ownership_candidate; count=9
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/dirty_index_range.rs
//     evidence={"module":455,"record_offset":12408,"parent_record_offset":12372,"ancestor_record_offset":12372,"inlinee":26105,"source_line":13,"archive":"_pdb/A/modules/00455.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_max
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":455,"record_offset":12492,"parent_record_offset":12408,"ancestor_record_offset":12372,"inlinee":26108,"source_line":2932,"archive":"_pdb/A/modules/00455.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_min
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":455,"record_offset":12436,"parent_record_offset":12408,"ancestor_record_offset":12372,"inlinee":26106,"source_line":2978,"archive":"_pdb/A/modules/00455.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_umax
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":455,"record_offset":12516,"parent_record_offset":12492,"ancestor_record_offset":12372,"inlinee":26109,"source_line":3538,"archive":"_pdb/A/modules/00455.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_umin
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":455,"record_offset":12460,"parent_record_offset":12436,"ancestor_record_offset":12372,"inlinee":26107,"source_line":3555,"archive":"_pdb/A/modules/00455.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::buffers::GpuInstance>
// 记录形式: compiled_procedure; instances=1
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1450,"record_offset":21764,"rva":17887408,"va":5386596528}
// 编译实例 RVAs: 0x110f0b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bd730
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13966789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386596528,"raw_hex":"e83be22aff","size":5,"soft_fail":false,"target_va":5372630832,"va":5386597616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140739e50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10310999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386596528,"raw_hex":"e8a9aa62ff","size":5,"soft_fail":false,"target_va":5376286288,"va":5386597282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9067229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386596528,"raw_hex":"e823a575ff","size":5,"soft_fail":false,"target_va":5377529888,"va":5386597112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-8474","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386596528,"raw_hex":"e8e6deffff","size":5,"soft_fail":false,"target_va":5386588864,"va":5386597333,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16382561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386596528,"raw_hex":"e861faf900","size":5,"soft_fail":false,"target_va":5402980128,"va":5386597562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16382879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386596528,"raw_hex":"e89ffbf900","size":5,"soft_fail":false,"target_va":5402980304,"va":5386597420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16382929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386596528,"raw_hex":"e8d1fbf900","size":5,"soft_fail":false,"target_va":5402980320,"va":5386597386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-7564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386596528,"raw_hex":"e874e2ffff","size":5,"soft_fail":false,"target_va":5386589056,"va":5386596615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t16383959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386596528,"raw_hex":"e8d7fff900","size":5,"soft_fail":false,"target_va":5402980656,"va":5386596692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ProfilerScope::new<alloc::string::String>
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386596528,"raw_hex":"e8a9e4ffff","size":5,"soft_fail":false,"target_va":5386590144,"va":5386597138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10240456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386596528,"raw_hex":"e838be63ff","size":5,"soft_fail":false,"target_va":5376356144,"va":5386596595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1450,"record_offset":23860,"parent_record_offset":21764,"ancestor_record_offset":21764,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1450,"record_offset":24228,"parent_record_offset":21764,"ancestor_record_offset":21764,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1450,"record_offset":23944,"parent_record_offset":21764,"ancestor_record_offset":21764,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::dirty_index_range::DirtyIndexRange::clear
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/dirty_index_range.rs
//     evidence={"module":1450,"record_offset":24200,"parent_record_offset":21764,"ancestor_record_offset":21764,"inlinee":27271,"source_line":22,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::dirty_index_range::DirtyIndexRange::to_range
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/dirty_index_range.rs
//     evidence={"module":1450,"record_offset":21992,"parent_record_offset":21764,"ancestor_record_offset":21764,"inlinee":27272,"source_line":26,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1450,"record_offset":24024,"parent_record_offset":21764,"ancestor_record_offset":21764,"inlinee":7514,"source_line":155,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1450,"record_offset":22276,"parent_record_offset":21764,"ancestor_record_offset":21764,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1450,"record_offset":22852,"parent_record_offset":21764,"ancestor_record_offset":21764,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1450,"record_offset":23908,"parent_record_offset":23884,"ancestor_record_offset":21764,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":1450,"record_offset":22488,"parent_record_offset":22392,"ancestor_record_offset":21764,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1450,"record_offset":23444,"parent_record_offset":22984,"ancestor_record_offset":21764,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1450,"record_offset":23884,"parent_record_offset":23860,"ancestor_record_offset":21764,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1450,"record_offset":22800,"parent_record_offset":22772,"ancestor_record_offset":21764,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1450,"record_offset":23220,"parent_record_offset":23196,"ancestor_record_offset":21764,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1450,"record_offset":23196,"parent_record_offset":23172,"ancestor_record_offset":21764,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1450,"record_offset":23404,"parent_record_offset":23372,"ancestor_record_offset":21764,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1450,"record_offset":24156,"parent_record_offset":24056,"ancestor_record_offset":21764,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1450,"record_offset":22424,"parent_record_offset":22392,"ancestor_record_offset":21764,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1450,"record_offset":24100,"parent_record_offset":24056,"ancestor_record_offset":21764,"inlinee":7501,"source_line":157,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1450,"record_offset":22452,"parent_record_offset":22424,"ancestor_record_offset":21764,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1450,"record_offset":23976,"parent_record_offset":23944,"ancestor_record_offset":21764,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1450,"record_offset":23172,"parent_record_offset":23148,"ancestor_record_offset":21764,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1450,"record_offset":23500,"parent_record_offset":23476,"ancestor_record_offset":21764,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1450,"record_offset":23148,"parent_record_offset":23040,"ancestor_record_offset":21764,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1450,"record_offset":22392,"parent_record_offset":22364,"ancestor_record_offset":21764,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1450,"record_offset":22880,"parent_record_offset":22852,"ancestor_record_offset":21764,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1450,"record_offset":23012,"parent_record_offset":22984,"ancestor_record_offset":21764,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1450,"record_offset":22984,"parent_record_offset":22952,"ancestor_record_offset":21764,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1450,"record_offset":22928,"parent_record_offset":22904,"ancestor_record_offset":21764,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1450,"record_offset":22952,"parent_record_offset":22928,"ancestor_record_offset":21764,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1450,"record_offset":22904,"parent_record_offset":22880,"ancestor_record_offset":21764,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1450,"record_offset":23476,"parent_record_offset":22952,"ancestor_record_offset":21764,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":22588,"parent_record_offset":22556,"ancestor_record_offset":21764,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":22748,"parent_record_offset":22588,"ancestor_record_offset":21764,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":22556,"parent_record_offset":22532,"ancestor_record_offset":21764,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":22308,"parent_record_offset":22276,"ancestor_record_offset":21764,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":22772,"parent_record_offset":22748,"ancestor_record_offset":21764,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":22364,"parent_record_offset":22332,"ancestor_record_offset":21764,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":22332,"parent_record_offset":22308,"ancestor_record_offset":21764,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1450,"record_offset":23040,"parent_record_offset":23012,"ancestor_record_offset":21764,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1450,"record_offset":23288,"parent_record_offset":23040,"ancestor_record_offset":21764,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1450,"record_offset":23260,"parent_record_offset":23040,"ancestor_record_offset":21764,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1450,"record_offset":23092,"parent_record_offset":23040,"ancestor_record_offset":21764,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1450,"record_offset":22044,"parent_record_offset":22020,"ancestor_record_offset":21764,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1450,"record_offset":22068,"parent_record_offset":22044,"ancestor_record_offset":21764,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::dirty_index_range::DirtyIndexRange::is_empty
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/dirty_index_range.rs
//     evidence={"module":1450,"record_offset":22020,"parent_record_offset":21992,"ancestor_record_offset":21764,"inlinee":27263,"source_line":18,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1450,"record_offset":24056,"parent_record_offset":24024,"ancestor_record_offset":21764,"inlinee":7500,"source_line":207,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::bytes::slice_as_bytes
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":1450,"record_offset":24128,"parent_record_offset":24056,"ancestor_record_offset":21764,"inlinee":7323,"source_line":8,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::GothicWindowBricksInstanceData>
// 记录形式: compiled_procedure; instances=1
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1450,"record_offset":39368,"rva":17889808,"va":5386598928}
// 编译实例 RVAs: 0x110fa10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bd730
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13969269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386598928,"raw_hex":"e88bd82aff","size":5,"soft_fail":false,"target_va":5372630832,"va":5386600096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140739e50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10313405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386598928,"raw_hex":"e843a162ff","size":5,"soft_fail":false,"target_va":5376286288,"va":5386599688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9069635","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386598928,"raw_hex":"e8bd9b75ff","size":5,"soft_fail":false,"target_va":5377529888,"va":5386599518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386598928,"raw_hex":"e880d5ffff","size":5,"soft_fail":false,"target_va":5386588864,"va":5386599739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16380149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386598928,"raw_hex":"e8f5f0f900","size":5,"soft_fail":false,"target_va":5402980128,"va":5386599974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16380473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386598928,"raw_hex":"e839f2f900","size":5,"soft_fail":false,"target_va":5402980304,"va":5386599826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16380523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386598928,"raw_hex":"e86bf2f900","size":5,"soft_fail":false,"target_va":5402980320,"va":5386599792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-9964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386598928,"raw_hex":"e814d9ffff","size":5,"soft_fail":false,"target_va":5386589056,"va":5386599015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t16381559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386598928,"raw_hex":"e877f6f900","size":5,"soft_fail":false,"target_va":5402980656,"va":5386599092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ProfilerScope::new<alloc::string::String>
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386598928,"raw_hex":"e847dbffff","size":5,"soft_fail":false,"target_va":5386590144,"va":5386599540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10242856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386598928,"raw_hex":"e8d8b463ff","size":5,"soft_fail":false,"target_va":5376356144,"va":5386598995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1450,"record_offset":41488,"parent_record_offset":39368,"ancestor_record_offset":39368,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1450,"record_offset":41856,"parent_record_offset":39368,"ancestor_record_offset":39368,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1450,"record_offset":41572,"parent_record_offset":39368,"ancestor_record_offset":39368,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::dirty_index_range::DirtyIndexRange::clear
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/dirty_index_range.rs
//     evidence={"module":1450,"record_offset":41828,"parent_record_offset":39368,"ancestor_record_offset":39368,"inlinee":27271,"source_line":22,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::dirty_index_range::DirtyIndexRange::to_range
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/dirty_index_range.rs
//     evidence={"module":1450,"record_offset":39620,"parent_record_offset":39368,"ancestor_record_offset":39368,"inlinee":27272,"source_line":26,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1450,"record_offset":41652,"parent_record_offset":39368,"ancestor_record_offset":39368,"inlinee":7514,"source_line":155,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1450,"record_offset":39904,"parent_record_offset":39368,"ancestor_record_offset":39368,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1450,"record_offset":40480,"parent_record_offset":39368,"ancestor_record_offset":39368,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1450,"record_offset":41536,"parent_record_offset":41512,"ancestor_record_offset":39368,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":1450,"record_offset":40116,"parent_record_offset":40020,"ancestor_record_offset":39368,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1450,"record_offset":41072,"parent_record_offset":40612,"ancestor_record_offset":39368,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1450,"record_offset":41512,"parent_record_offset":41488,"ancestor_record_offset":39368,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1450,"record_offset":40428,"parent_record_offset":40400,"ancestor_record_offset":39368,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1450,"record_offset":40848,"parent_record_offset":40824,"ancestor_record_offset":39368,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1450,"record_offset":40824,"parent_record_offset":40800,"ancestor_record_offset":39368,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1450,"record_offset":41032,"parent_record_offset":41000,"ancestor_record_offset":39368,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1450,"record_offset":41784,"parent_record_offset":41684,"ancestor_record_offset":39368,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1450,"record_offset":40052,"parent_record_offset":40020,"ancestor_record_offset":39368,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1450,"record_offset":41728,"parent_record_offset":41684,"ancestor_record_offset":39368,"inlinee":7501,"source_line":157,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1450,"record_offset":40080,"parent_record_offset":40052,"ancestor_record_offset":39368,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1450,"record_offset":41604,"parent_record_offset":41572,"ancestor_record_offset":39368,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1450,"record_offset":40800,"parent_record_offset":40776,"ancestor_record_offset":39368,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1450,"record_offset":41128,"parent_record_offset":41104,"ancestor_record_offset":39368,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1450,"record_offset":40776,"parent_record_offset":40668,"ancestor_record_offset":39368,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1450,"record_offset":40020,"parent_record_offset":39992,"ancestor_record_offset":39368,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1450,"record_offset":40508,"parent_record_offset":40480,"ancestor_record_offset":39368,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1450,"record_offset":40640,"parent_record_offset":40612,"ancestor_record_offset":39368,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1450,"record_offset":40612,"parent_record_offset":40580,"ancestor_record_offset":39368,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1450,"record_offset":40556,"parent_record_offset":40532,"ancestor_record_offset":39368,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1450,"record_offset":40580,"parent_record_offset":40556,"ancestor_record_offset":39368,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1450,"record_offset":40532,"parent_record_offset":40508,"ancestor_record_offset":39368,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1450,"record_offset":41104,"parent_record_offset":40580,"ancestor_record_offset":39368,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":40216,"parent_record_offset":40184,"ancestor_record_offset":39368,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":40376,"parent_record_offset":40216,"ancestor_record_offset":39368,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":40184,"parent_record_offset":40160,"ancestor_record_offset":39368,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":39936,"parent_record_offset":39904,"ancestor_record_offset":39368,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":40400,"parent_record_offset":40376,"ancestor_record_offset":39368,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":39992,"parent_record_offset":39960,"ancestor_record_offset":39368,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":39960,"parent_record_offset":39936,"ancestor_record_offset":39368,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1450,"record_offset":40668,"parent_record_offset":40640,"ancestor_record_offset":39368,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1450,"record_offset":40916,"parent_record_offset":40668,"ancestor_record_offset":39368,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1450,"record_offset":40888,"parent_record_offset":40668,"ancestor_record_offset":39368,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1450,"record_offset":40720,"parent_record_offset":40668,"ancestor_record_offset":39368,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1450,"record_offset":39672,"parent_record_offset":39648,"ancestor_record_offset":39368,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1450,"record_offset":39696,"parent_record_offset":39672,"ancestor_record_offset":39368,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::dirty_index_range::DirtyIndexRange::is_empty
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/dirty_index_range.rs
//     evidence={"module":1450,"record_offset":39648,"parent_record_offset":39620,"ancestor_record_offset":39368,"inlinee":27263,"source_line":18,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1450,"record_offset":41684,"parent_record_offset":41652,"ancestor_record_offset":39368,"inlinee":7500,"source_line":207,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::bytes::slice_as_bytes
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":1450,"record_offset":41756,"parent_record_offset":41684,"ancestor_record_offset":39368,"inlinee":7323,"source_line":8,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::PreplacedFlowerInstanceData>
// 记录形式: compiled_procedure; instances=1
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1450,"record_offset":19244,"rva":17886192,"va":5386595312}
// 编译实例 RVAs: 0x110ebf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bd730
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13965589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386595312,"raw_hex":"e8ebe62aff","size":5,"soft_fail":false,"target_va":5372630832,"va":5386596416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140739e50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10309804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386595312,"raw_hex":"e854af62ff","size":5,"soft_fail":false,"target_va":5376286288,"va":5386596087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9066013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386595312,"raw_hex":"e8e3a975ff","size":5,"soft_fail":false,"target_va":5377529888,"va":5386595896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-7279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386595312,"raw_hex":"e891e3ffff","size":5,"soft_fail":false,"target_va":5386588864,"va":5386596138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16383756","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386595312,"raw_hex":"e80cfff900","size":5,"soft_fail":false,"target_va":5402980128,"va":5386596367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16384074","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386595312,"raw_hex":"e84a00fa00","size":5,"soft_fail":false,"target_va":5402980304,"va":5386596225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16384124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386595312,"raw_hex":"e87c00fa00","size":5,"soft_fail":false,"target_va":5402980320,"va":5386596191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386595312,"raw_hex":"e834e7ffff","size":5,"soft_fail":false,"target_va":5386589056,"va":5386595399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t16385175","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386595312,"raw_hex":"e89704fa00","size":5,"soft_fail":false,"target_va":5402980656,"va":5386595476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ProfilerScope::new<alloc::string::String>
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386595312,"raw_hex":"e869e9ffff","size":5,"soft_fail":false,"target_va":5386590144,"va":5386595922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10239240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386595312,"raw_hex":"e8f8c263ff","size":5,"soft_fail":false,"target_va":5376356144,"va":5386595379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1450,"record_offset":21360,"parent_record_offset":19244,"ancestor_record_offset":19244,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1450,"record_offset":21732,"parent_record_offset":19244,"ancestor_record_offset":19244,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1450,"record_offset":21444,"parent_record_offset":19244,"ancestor_record_offset":19244,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::dirty_index_range::DirtyIndexRange::clear
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/dirty_index_range.rs
//     evidence={"module":1450,"record_offset":21704,"parent_record_offset":19244,"ancestor_record_offset":19244,"inlinee":27271,"source_line":22,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::dirty_index_range::DirtyIndexRange::to_range
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/dirty_index_range.rs
//     evidence={"module":1450,"record_offset":19492,"parent_record_offset":19244,"ancestor_record_offset":19244,"inlinee":27272,"source_line":26,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1450,"record_offset":21524,"parent_record_offset":19244,"ancestor_record_offset":19244,"inlinee":7514,"source_line":155,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1450,"record_offset":19776,"parent_record_offset":19244,"ancestor_record_offset":19244,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1450,"record_offset":20352,"parent_record_offset":19244,"ancestor_record_offset":19244,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1450,"record_offset":21408,"parent_record_offset":21384,"ancestor_record_offset":19244,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":1450,"record_offset":19988,"parent_record_offset":19892,"ancestor_record_offset":19244,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1450,"record_offset":20944,"parent_record_offset":20484,"ancestor_record_offset":19244,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1450,"record_offset":21384,"parent_record_offset":21360,"ancestor_record_offset":19244,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1450,"record_offset":20300,"parent_record_offset":20272,"ancestor_record_offset":19244,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1450,"record_offset":20720,"parent_record_offset":20696,"ancestor_record_offset":19244,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1450,"record_offset":20696,"parent_record_offset":20672,"ancestor_record_offset":19244,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1450,"record_offset":20904,"parent_record_offset":20872,"ancestor_record_offset":19244,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1450,"record_offset":21660,"parent_record_offset":21556,"ancestor_record_offset":19244,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1450,"record_offset":19924,"parent_record_offset":19892,"ancestor_record_offset":19244,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1450,"record_offset":21604,"parent_record_offset":21556,"ancestor_record_offset":19244,"inlinee":7501,"source_line":157,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1450,"record_offset":19952,"parent_record_offset":19924,"ancestor_record_offset":19244,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1450,"record_offset":21476,"parent_record_offset":21444,"ancestor_record_offset":19244,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1450,"record_offset":20672,"parent_record_offset":20648,"ancestor_record_offset":19244,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1450,"record_offset":21000,"parent_record_offset":20976,"ancestor_record_offset":19244,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1450,"record_offset":20648,"parent_record_offset":20540,"ancestor_record_offset":19244,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1450,"record_offset":19892,"parent_record_offset":19864,"ancestor_record_offset":19244,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1450,"record_offset":20380,"parent_record_offset":20352,"ancestor_record_offset":19244,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1450,"record_offset":20512,"parent_record_offset":20484,"ancestor_record_offset":19244,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1450,"record_offset":20484,"parent_record_offset":20452,"ancestor_record_offset":19244,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1450,"record_offset":20428,"parent_record_offset":20404,"ancestor_record_offset":19244,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1450,"record_offset":20452,"parent_record_offset":20428,"ancestor_record_offset":19244,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1450,"record_offset":20404,"parent_record_offset":20380,"ancestor_record_offset":19244,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1450,"record_offset":20976,"parent_record_offset":20452,"ancestor_record_offset":19244,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":20088,"parent_record_offset":20056,"ancestor_record_offset":19244,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":20248,"parent_record_offset":20088,"ancestor_record_offset":19244,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":20056,"parent_record_offset":20032,"ancestor_record_offset":19244,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":19808,"parent_record_offset":19776,"ancestor_record_offset":19244,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":20272,"parent_record_offset":20248,"ancestor_record_offset":19244,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":19864,"parent_record_offset":19832,"ancestor_record_offset":19244,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":19832,"parent_record_offset":19808,"ancestor_record_offset":19244,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1450,"record_offset":20540,"parent_record_offset":20512,"ancestor_record_offset":19244,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1450,"record_offset":20788,"parent_record_offset":20540,"ancestor_record_offset":19244,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1450,"record_offset":20760,"parent_record_offset":20540,"ancestor_record_offset":19244,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1450,"record_offset":20592,"parent_record_offset":20540,"ancestor_record_offset":19244,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1450,"record_offset":19544,"parent_record_offset":19520,"ancestor_record_offset":19244,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1450,"record_offset":19568,"parent_record_offset":19544,"ancestor_record_offset":19244,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::dirty_index_range::DirtyIndexRange::is_empty
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/dirty_index_range.rs
//     evidence={"module":1450,"record_offset":19520,"parent_record_offset":19492,"ancestor_record_offset":19244,"inlinee":27263,"source_line":18,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1450,"record_offset":21556,"parent_record_offset":21524,"ancestor_record_offset":19244,"inlinee":7500,"source_line":207,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::bytes::slice_as_bytes
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":1450,"record_offset":21632,"parent_record_offset":21556,"ancestor_record_offset":19244,"inlinee":7323,"source_line":8,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::SolidVertexColorBricksInstanceData>
// 记录形式: compiled_procedure; instances=1
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1450,"record_offset":34336,"rva":17888608,"va":5386597728}
// 编译实例 RVAs: 0x110f560
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bd730
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13967989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e88bdd2aff","size":5,"soft_fail":false,"target_va":5372630832,"va":5386598816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140739e50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10312199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e8f9a562ff","size":5,"soft_fail":false,"target_va":5376286288,"va":5386598482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9068429","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e873a075ff","size":5,"soft_fail":false,"target_va":5377529888,"va":5386598312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-9674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e836daffff","size":5,"soft_fail":false,"target_va":5386588864,"va":5386598533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16381361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e8b1f5f900","size":5,"soft_fail":false,"target_va":5402980128,"va":5386598762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16381679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e8eff6f900","size":5,"soft_fail":false,"target_va":5402980304,"va":5386598620,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16381729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e821f7f900","size":5,"soft_fail":false,"target_va":5402980320,"va":5386598586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-8764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e8c4ddffff","size":5,"soft_fail":false,"target_va":5386589056,"va":5386597815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t16382759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e827fbf900","size":5,"soft_fail":false,"target_va":5402980656,"va":5386597892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ProfilerScope::new<alloc::string::String>
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-8199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e8f9dfffff","size":5,"soft_fail":false,"target_va":5386590144,"va":5386598338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10241656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e888b963ff","size":5,"soft_fail":false,"target_va":5376356144,"va":5386597795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1450,"record_offset":36460,"parent_record_offset":34336,"ancestor_record_offset":34336,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1450,"record_offset":36828,"parent_record_offset":34336,"ancestor_record_offset":34336,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1450,"record_offset":36544,"parent_record_offset":34336,"ancestor_record_offset":34336,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::dirty_index_range::DirtyIndexRange::clear
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/dirty_index_range.rs
//     evidence={"module":1450,"record_offset":36800,"parent_record_offset":34336,"ancestor_record_offset":34336,"inlinee":27271,"source_line":22,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::dirty_index_range::DirtyIndexRange::to_range
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/dirty_index_range.rs
//     evidence={"module":1450,"record_offset":34592,"parent_record_offset":34336,"ancestor_record_offset":34336,"inlinee":27272,"source_line":26,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1450,"record_offset":36624,"parent_record_offset":34336,"ancestor_record_offset":34336,"inlinee":7514,"source_line":155,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1450,"record_offset":34876,"parent_record_offset":34336,"ancestor_record_offset":34336,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1450,"record_offset":35452,"parent_record_offset":34336,"ancestor_record_offset":34336,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1450,"record_offset":36508,"parent_record_offset":36484,"ancestor_record_offset":34336,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":1450,"record_offset":35088,"parent_record_offset":34992,"ancestor_record_offset":34336,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1450,"record_offset":36044,"parent_record_offset":35584,"ancestor_record_offset":34336,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1450,"record_offset":36484,"parent_record_offset":36460,"ancestor_record_offset":34336,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1450,"record_offset":35400,"parent_record_offset":35372,"ancestor_record_offset":34336,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1450,"record_offset":35820,"parent_record_offset":35796,"ancestor_record_offset":34336,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1450,"record_offset":35796,"parent_record_offset":35772,"ancestor_record_offset":34336,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1450,"record_offset":36004,"parent_record_offset":35972,"ancestor_record_offset":34336,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1450,"record_offset":36756,"parent_record_offset":36656,"ancestor_record_offset":34336,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1450,"record_offset":35024,"parent_record_offset":34992,"ancestor_record_offset":34336,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1450,"record_offset":36700,"parent_record_offset":36656,"ancestor_record_offset":34336,"inlinee":7501,"source_line":157,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1450,"record_offset":35052,"parent_record_offset":35024,"ancestor_record_offset":34336,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1450,"record_offset":36576,"parent_record_offset":36544,"ancestor_record_offset":34336,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1450,"record_offset":35772,"parent_record_offset":35748,"ancestor_record_offset":34336,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1450,"record_offset":36100,"parent_record_offset":36076,"ancestor_record_offset":34336,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1450,"record_offset":35748,"parent_record_offset":35640,"ancestor_record_offset":34336,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1450,"record_offset":34992,"parent_record_offset":34964,"ancestor_record_offset":34336,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1450,"record_offset":35480,"parent_record_offset":35452,"ancestor_record_offset":34336,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1450,"record_offset":35612,"parent_record_offset":35584,"ancestor_record_offset":34336,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1450,"record_offset":35584,"parent_record_offset":35552,"ancestor_record_offset":34336,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1450,"record_offset":35528,"parent_record_offset":35504,"ancestor_record_offset":34336,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1450,"record_offset":35552,"parent_record_offset":35528,"ancestor_record_offset":34336,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1450,"record_offset":35504,"parent_record_offset":35480,"ancestor_record_offset":34336,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1450,"record_offset":36076,"parent_record_offset":35552,"ancestor_record_offset":34336,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":35188,"parent_record_offset":35156,"ancestor_record_offset":34336,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":35348,"parent_record_offset":35188,"ancestor_record_offset":34336,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":35156,"parent_record_offset":35132,"ancestor_record_offset":34336,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":34908,"parent_record_offset":34876,"ancestor_record_offset":34336,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":35372,"parent_record_offset":35348,"ancestor_record_offset":34336,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":34964,"parent_record_offset":34932,"ancestor_record_offset":34336,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":34932,"parent_record_offset":34908,"ancestor_record_offset":34336,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1450,"record_offset":35640,"parent_record_offset":35612,"ancestor_record_offset":34336,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1450,"record_offset":35888,"parent_record_offset":35640,"ancestor_record_offset":34336,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1450,"record_offset":35860,"parent_record_offset":35640,"ancestor_record_offset":34336,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1450,"record_offset":35692,"parent_record_offset":35640,"ancestor_record_offset":34336,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1450,"record_offset":34644,"parent_record_offset":34620,"ancestor_record_offset":34336,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1450,"record_offset":34668,"parent_record_offset":34644,"ancestor_record_offset":34336,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::dirty_index_range::DirtyIndexRange::is_empty
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/dirty_index_range.rs
//     evidence={"module":1450,"record_offset":34620,"parent_record_offset":34592,"ancestor_record_offset":34336,"inlinee":27263,"source_line":18,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1450,"record_offset":36656,"parent_record_offset":36624,"ancestor_record_offset":34336,"inlinee":7500,"source_line":207,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::bytes::slice_as_bytes
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":1450,"record_offset":36728,"parent_record_offset":36656,"ancestor_record_offset":34336,"inlinee":7323,"source_line":8,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData>
// 记录形式: compiled_procedure; instances=1
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1450,"record_offset":24260,"rva":17888608,"va":5386597728}
// 编译实例 RVAs: 0x110f560
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bd730
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13967989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e88bdd2aff","size":5,"soft_fail":false,"target_va":5372630832,"va":5386598816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140739e50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10312199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e8f9a562ff","size":5,"soft_fail":false,"target_va":5376286288,"va":5386598482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9068429","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e873a075ff","size":5,"soft_fail":false,"target_va":5377529888,"va":5386598312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-9674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e836daffff","size":5,"soft_fail":false,"target_va":5386588864,"va":5386598533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16381361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e8b1f5f900","size":5,"soft_fail":false,"target_va":5402980128,"va":5386598762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16381679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e8eff6f900","size":5,"soft_fail":false,"target_va":5402980304,"va":5386598620,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16381729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e821f7f900","size":5,"soft_fail":false,"target_va":5402980320,"va":5386598586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-8764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e8c4ddffff","size":5,"soft_fail":false,"target_va":5386589056,"va":5386597815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t16382759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e827fbf900","size":5,"soft_fail":false,"target_va":5402980656,"va":5386597892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ProfilerScope::new<alloc::string::String>
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-8199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e8f9dfffff","size":5,"soft_fail":false,"target_va":5386590144,"va":5386598338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10241656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e888b963ff","size":5,"soft_fail":false,"target_va":5376356144,"va":5386597795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1450,"record_offset":26388,"parent_record_offset":24260,"ancestor_record_offset":24260,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1450,"record_offset":26756,"parent_record_offset":24260,"ancestor_record_offset":24260,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1450,"record_offset":26472,"parent_record_offset":24260,"ancestor_record_offset":24260,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::dirty_index_range::DirtyIndexRange::clear
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/dirty_index_range.rs
//     evidence={"module":1450,"record_offset":26728,"parent_record_offset":24260,"ancestor_record_offset":24260,"inlinee":27271,"source_line":22,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::dirty_index_range::DirtyIndexRange::to_range
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/dirty_index_range.rs
//     evidence={"module":1450,"record_offset":24520,"parent_record_offset":24260,"ancestor_record_offset":24260,"inlinee":27272,"source_line":26,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1450,"record_offset":26552,"parent_record_offset":24260,"ancestor_record_offset":24260,"inlinee":7514,"source_line":155,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1450,"record_offset":24804,"parent_record_offset":24260,"ancestor_record_offset":24260,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1450,"record_offset":25380,"parent_record_offset":24260,"ancestor_record_offset":24260,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1450,"record_offset":26436,"parent_record_offset":26412,"ancestor_record_offset":24260,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":1450,"record_offset":25016,"parent_record_offset":24920,"ancestor_record_offset":24260,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1450,"record_offset":25972,"parent_record_offset":25512,"ancestor_record_offset":24260,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1450,"record_offset":26412,"parent_record_offset":26388,"ancestor_record_offset":24260,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1450,"record_offset":25328,"parent_record_offset":25300,"ancestor_record_offset":24260,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1450,"record_offset":25748,"parent_record_offset":25724,"ancestor_record_offset":24260,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1450,"record_offset":25724,"parent_record_offset":25700,"ancestor_record_offset":24260,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1450,"record_offset":25932,"parent_record_offset":25900,"ancestor_record_offset":24260,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1450,"record_offset":26684,"parent_record_offset":26584,"ancestor_record_offset":24260,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1450,"record_offset":24952,"parent_record_offset":24920,"ancestor_record_offset":24260,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1450,"record_offset":26628,"parent_record_offset":26584,"ancestor_record_offset":24260,"inlinee":7501,"source_line":157,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1450,"record_offset":24980,"parent_record_offset":24952,"ancestor_record_offset":24260,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1450,"record_offset":26504,"parent_record_offset":26472,"ancestor_record_offset":24260,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1450,"record_offset":25700,"parent_record_offset":25676,"ancestor_record_offset":24260,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1450,"record_offset":26028,"parent_record_offset":26004,"ancestor_record_offset":24260,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1450,"record_offset":25676,"parent_record_offset":25568,"ancestor_record_offset":24260,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1450,"record_offset":24920,"parent_record_offset":24892,"ancestor_record_offset":24260,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1450,"record_offset":25408,"parent_record_offset":25380,"ancestor_record_offset":24260,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1450,"record_offset":25540,"parent_record_offset":25512,"ancestor_record_offset":24260,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1450,"record_offset":25512,"parent_record_offset":25480,"ancestor_record_offset":24260,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1450,"record_offset":25456,"parent_record_offset":25432,"ancestor_record_offset":24260,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1450,"record_offset":25480,"parent_record_offset":25456,"ancestor_record_offset":24260,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1450,"record_offset":25432,"parent_record_offset":25408,"ancestor_record_offset":24260,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1450,"record_offset":26004,"parent_record_offset":25480,"ancestor_record_offset":24260,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":25116,"parent_record_offset":25084,"ancestor_record_offset":24260,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":25276,"parent_record_offset":25116,"ancestor_record_offset":24260,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":25084,"parent_record_offset":25060,"ancestor_record_offset":24260,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":24836,"parent_record_offset":24804,"ancestor_record_offset":24260,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":25300,"parent_record_offset":25276,"ancestor_record_offset":24260,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":24892,"parent_record_offset":24860,"ancestor_record_offset":24260,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":24860,"parent_record_offset":24836,"ancestor_record_offset":24260,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1450,"record_offset":25568,"parent_record_offset":25540,"ancestor_record_offset":24260,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1450,"record_offset":25816,"parent_record_offset":25568,"ancestor_record_offset":24260,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1450,"record_offset":25788,"parent_record_offset":25568,"ancestor_record_offset":24260,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1450,"record_offset":25620,"parent_record_offset":25568,"ancestor_record_offset":24260,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1450,"record_offset":24572,"parent_record_offset":24548,"ancestor_record_offset":24260,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1450,"record_offset":24596,"parent_record_offset":24572,"ancestor_record_offset":24260,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::dirty_index_range::DirtyIndexRange::is_empty
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/dirty_index_range.rs
//     evidence={"module":1450,"record_offset":24548,"parent_record_offset":24520,"ancestor_record_offset":24260,"inlinee":27263,"source_line":18,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1450,"record_offset":26584,"parent_record_offset":26552,"ancestor_record_offset":24260,"inlinee":7500,"source_line":207,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::bytes::slice_as_bytes
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":1450,"record_offset":26656,"parent_record_offset":26584,"ancestor_record_offset":24260,"inlinee":7323,"source_line":8,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData>
// 记录形式: compiled_procedure; instances=1
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1450,"record_offset":26788,"rva":17888608,"va":5386597728}
// 编译实例 RVAs: 0x110f560
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bd730
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13967989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e88bdd2aff","size":5,"soft_fail":false,"target_va":5372630832,"va":5386598816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140739e50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10312199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e8f9a562ff","size":5,"soft_fail":false,"target_va":5376286288,"va":5386598482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9068429","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e873a075ff","size":5,"soft_fail":false,"target_va":5377529888,"va":5386598312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-9674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e836daffff","size":5,"soft_fail":false,"target_va":5386588864,"va":5386598533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16381361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e8b1f5f900","size":5,"soft_fail":false,"target_va":5402980128,"va":5386598762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16381679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e8eff6f900","size":5,"soft_fail":false,"target_va":5402980304,"va":5386598620,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16381729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e821f7f900","size":5,"soft_fail":false,"target_va":5402980320,"va":5386598586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-8764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e8c4ddffff","size":5,"soft_fail":false,"target_va":5386589056,"va":5386597815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t16382759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e827fbf900","size":5,"soft_fail":false,"target_va":5402980656,"va":5386597892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ProfilerScope::new<alloc::string::String>
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-8199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e8f9dfffff","size":5,"soft_fail":false,"target_va":5386590144,"va":5386598338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10241656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e888b963ff","size":5,"soft_fail":false,"target_va":5376356144,"va":5386597795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1450,"record_offset":28912,"parent_record_offset":26788,"ancestor_record_offset":26788,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1450,"record_offset":29280,"parent_record_offset":26788,"ancestor_record_offset":26788,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1450,"record_offset":28996,"parent_record_offset":26788,"ancestor_record_offset":26788,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::dirty_index_range::DirtyIndexRange::clear
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/dirty_index_range.rs
//     evidence={"module":1450,"record_offset":29252,"parent_record_offset":26788,"ancestor_record_offset":26788,"inlinee":27271,"source_line":22,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::dirty_index_range::DirtyIndexRange::to_range
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/dirty_index_range.rs
//     evidence={"module":1450,"record_offset":27044,"parent_record_offset":26788,"ancestor_record_offset":26788,"inlinee":27272,"source_line":26,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1450,"record_offset":29076,"parent_record_offset":26788,"ancestor_record_offset":26788,"inlinee":7514,"source_line":155,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1450,"record_offset":27328,"parent_record_offset":26788,"ancestor_record_offset":26788,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1450,"record_offset":27904,"parent_record_offset":26788,"ancestor_record_offset":26788,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1450,"record_offset":28960,"parent_record_offset":28936,"ancestor_record_offset":26788,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":1450,"record_offset":27540,"parent_record_offset":27444,"ancestor_record_offset":26788,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1450,"record_offset":28496,"parent_record_offset":28036,"ancestor_record_offset":26788,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1450,"record_offset":28936,"parent_record_offset":28912,"ancestor_record_offset":26788,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1450,"record_offset":27852,"parent_record_offset":27824,"ancestor_record_offset":26788,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1450,"record_offset":28272,"parent_record_offset":28248,"ancestor_record_offset":26788,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1450,"record_offset":28248,"parent_record_offset":28224,"ancestor_record_offset":26788,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1450,"record_offset":28456,"parent_record_offset":28424,"ancestor_record_offset":26788,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1450,"record_offset":29208,"parent_record_offset":29108,"ancestor_record_offset":26788,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1450,"record_offset":27476,"parent_record_offset":27444,"ancestor_record_offset":26788,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1450,"record_offset":29152,"parent_record_offset":29108,"ancestor_record_offset":26788,"inlinee":7501,"source_line":157,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1450,"record_offset":27504,"parent_record_offset":27476,"ancestor_record_offset":26788,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1450,"record_offset":29028,"parent_record_offset":28996,"ancestor_record_offset":26788,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1450,"record_offset":28224,"parent_record_offset":28200,"ancestor_record_offset":26788,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1450,"record_offset":28552,"parent_record_offset":28528,"ancestor_record_offset":26788,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1450,"record_offset":28200,"parent_record_offset":28092,"ancestor_record_offset":26788,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1450,"record_offset":27444,"parent_record_offset":27416,"ancestor_record_offset":26788,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1450,"record_offset":27932,"parent_record_offset":27904,"ancestor_record_offset":26788,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1450,"record_offset":28064,"parent_record_offset":28036,"ancestor_record_offset":26788,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1450,"record_offset":28036,"parent_record_offset":28004,"ancestor_record_offset":26788,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1450,"record_offset":27980,"parent_record_offset":27956,"ancestor_record_offset":26788,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1450,"record_offset":28004,"parent_record_offset":27980,"ancestor_record_offset":26788,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1450,"record_offset":27956,"parent_record_offset":27932,"ancestor_record_offset":26788,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1450,"record_offset":28528,"parent_record_offset":28004,"ancestor_record_offset":26788,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":27640,"parent_record_offset":27608,"ancestor_record_offset":26788,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":27800,"parent_record_offset":27640,"ancestor_record_offset":26788,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":27608,"parent_record_offset":27584,"ancestor_record_offset":26788,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":27360,"parent_record_offset":27328,"ancestor_record_offset":26788,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":27824,"parent_record_offset":27800,"ancestor_record_offset":26788,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":27416,"parent_record_offset":27384,"ancestor_record_offset":26788,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":27384,"parent_record_offset":27360,"ancestor_record_offset":26788,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1450,"record_offset":28092,"parent_record_offset":28064,"ancestor_record_offset":26788,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1450,"record_offset":28340,"parent_record_offset":28092,"ancestor_record_offset":26788,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1450,"record_offset":28312,"parent_record_offset":28092,"ancestor_record_offset":26788,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1450,"record_offset":28144,"parent_record_offset":28092,"ancestor_record_offset":26788,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1450,"record_offset":27096,"parent_record_offset":27072,"ancestor_record_offset":26788,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1450,"record_offset":27120,"parent_record_offset":27096,"ancestor_record_offset":26788,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::dirty_index_range::DirtyIndexRange::is_empty
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/dirty_index_range.rs
//     evidence={"module":1450,"record_offset":27072,"parent_record_offset":27044,"ancestor_record_offset":26788,"inlinee":27263,"source_line":18,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1450,"record_offset":29108,"parent_record_offset":29076,"ancestor_record_offset":26788,"inlinee":7500,"source_line":207,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::bytes::slice_as_bytes
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":1450,"record_offset":29180,"parent_record_offset":29108,"ancestor_record_offset":26788,"inlinee":7323,"source_line":8,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::StoneFloorInstanceData>
// 记录形式: compiled_procedure; instances=1
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1450,"record_offset":31820,"rva":17886192,"va":5386595312}
// 编译实例 RVAs: 0x110ebf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bd730
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13965589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386595312,"raw_hex":"e8ebe62aff","size":5,"soft_fail":false,"target_va":5372630832,"va":5386596416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140739e50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10309804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386595312,"raw_hex":"e854af62ff","size":5,"soft_fail":false,"target_va":5376286288,"va":5386596087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9066013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386595312,"raw_hex":"e8e3a975ff","size":5,"soft_fail":false,"target_va":5377529888,"va":5386595896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-7279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386595312,"raw_hex":"e891e3ffff","size":5,"soft_fail":false,"target_va":5386588864,"va":5386596138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16383756","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386595312,"raw_hex":"e80cfff900","size":5,"soft_fail":false,"target_va":5402980128,"va":5386596367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16384074","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386595312,"raw_hex":"e84a00fa00","size":5,"soft_fail":false,"target_va":5402980304,"va":5386596225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16384124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386595312,"raw_hex":"e87c00fa00","size":5,"soft_fail":false,"target_va":5402980320,"va":5386596191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386595312,"raw_hex":"e834e7ffff","size":5,"soft_fail":false,"target_va":5386589056,"va":5386595399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t16385175","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386595312,"raw_hex":"e89704fa00","size":5,"soft_fail":false,"target_va":5402980656,"va":5386595476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ProfilerScope::new<alloc::string::String>
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386595312,"raw_hex":"e869e9ffff","size":5,"soft_fail":false,"target_va":5386590144,"va":5386595922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10239240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386595312,"raw_hex":"e8f8c263ff","size":5,"soft_fail":false,"target_va":5376356144,"va":5386595379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1450,"record_offset":33932,"parent_record_offset":31820,"ancestor_record_offset":31820,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1450,"record_offset":34304,"parent_record_offset":31820,"ancestor_record_offset":31820,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1450,"record_offset":34016,"parent_record_offset":31820,"ancestor_record_offset":31820,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::dirty_index_range::DirtyIndexRange::clear
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/dirty_index_range.rs
//     evidence={"module":1450,"record_offset":34276,"parent_record_offset":31820,"ancestor_record_offset":31820,"inlinee":27271,"source_line":22,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::dirty_index_range::DirtyIndexRange::to_range
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/dirty_index_range.rs
//     evidence={"module":1450,"record_offset":32064,"parent_record_offset":31820,"ancestor_record_offset":31820,"inlinee":27272,"source_line":26,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1450,"record_offset":34096,"parent_record_offset":31820,"ancestor_record_offset":31820,"inlinee":7514,"source_line":155,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1450,"record_offset":32348,"parent_record_offset":31820,"ancestor_record_offset":31820,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1450,"record_offset":32924,"parent_record_offset":31820,"ancestor_record_offset":31820,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1450,"record_offset":33980,"parent_record_offset":33956,"ancestor_record_offset":31820,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":1450,"record_offset":32560,"parent_record_offset":32464,"ancestor_record_offset":31820,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1450,"record_offset":33516,"parent_record_offset":33056,"ancestor_record_offset":31820,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1450,"record_offset":33956,"parent_record_offset":33932,"ancestor_record_offset":31820,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1450,"record_offset":32872,"parent_record_offset":32844,"ancestor_record_offset":31820,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1450,"record_offset":33292,"parent_record_offset":33268,"ancestor_record_offset":31820,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1450,"record_offset":33268,"parent_record_offset":33244,"ancestor_record_offset":31820,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1450,"record_offset":33476,"parent_record_offset":33444,"ancestor_record_offset":31820,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1450,"record_offset":34232,"parent_record_offset":34128,"ancestor_record_offset":31820,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1450,"record_offset":32496,"parent_record_offset":32464,"ancestor_record_offset":31820,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1450,"record_offset":34176,"parent_record_offset":34128,"ancestor_record_offset":31820,"inlinee":7501,"source_line":157,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1450,"record_offset":32524,"parent_record_offset":32496,"ancestor_record_offset":31820,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1450,"record_offset":34048,"parent_record_offset":34016,"ancestor_record_offset":31820,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1450,"record_offset":33244,"parent_record_offset":33220,"ancestor_record_offset":31820,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1450,"record_offset":33572,"parent_record_offset":33548,"ancestor_record_offset":31820,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1450,"record_offset":33220,"parent_record_offset":33112,"ancestor_record_offset":31820,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1450,"record_offset":32464,"parent_record_offset":32436,"ancestor_record_offset":31820,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1450,"record_offset":32952,"parent_record_offset":32924,"ancestor_record_offset":31820,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1450,"record_offset":33084,"parent_record_offset":33056,"ancestor_record_offset":31820,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1450,"record_offset":33056,"parent_record_offset":33024,"ancestor_record_offset":31820,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1450,"record_offset":33000,"parent_record_offset":32976,"ancestor_record_offset":31820,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1450,"record_offset":33024,"parent_record_offset":33000,"ancestor_record_offset":31820,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1450,"record_offset":32976,"parent_record_offset":32952,"ancestor_record_offset":31820,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1450,"record_offset":33548,"parent_record_offset":33024,"ancestor_record_offset":31820,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":32660,"parent_record_offset":32628,"ancestor_record_offset":31820,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":32820,"parent_record_offset":32660,"ancestor_record_offset":31820,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":32628,"parent_record_offset":32604,"ancestor_record_offset":31820,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":32380,"parent_record_offset":32348,"ancestor_record_offset":31820,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":32844,"parent_record_offset":32820,"ancestor_record_offset":31820,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":32436,"parent_record_offset":32404,"ancestor_record_offset":31820,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":32404,"parent_record_offset":32380,"ancestor_record_offset":31820,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1450,"record_offset":33112,"parent_record_offset":33084,"ancestor_record_offset":31820,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1450,"record_offset":33360,"parent_record_offset":33112,"ancestor_record_offset":31820,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1450,"record_offset":33332,"parent_record_offset":33112,"ancestor_record_offset":31820,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1450,"record_offset":33164,"parent_record_offset":33112,"ancestor_record_offset":31820,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1450,"record_offset":32116,"parent_record_offset":32092,"ancestor_record_offset":31820,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1450,"record_offset":32140,"parent_record_offset":32116,"ancestor_record_offset":31820,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::dirty_index_range::DirtyIndexRange::is_empty
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/dirty_index_range.rs
//     evidence={"module":1450,"record_offset":32092,"parent_record_offset":32064,"ancestor_record_offset":31820,"inlinee":27263,"source_line":18,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1450,"record_offset":34128,"parent_record_offset":34096,"ancestor_record_offset":31820,"inlinee":7500,"source_line":207,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::bytes::slice_as_bytes
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":1450,"record_offset":34204,"parent_record_offset":34128,"ancestor_record_offset":31820,"inlinee":7323,"source_line":8,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::WrenInstanceData>
// 记录形式: compiled_procedure; instances=1
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1450,"record_offset":36860,"rva":17888608,"va":5386597728}
// 编译实例 RVAs: 0x110f560
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bd730
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13967989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e88bdd2aff","size":5,"soft_fail":false,"target_va":5372630832,"va":5386598816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140739e50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10312199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e8f9a562ff","size":5,"soft_fail":false,"target_va":5376286288,"va":5386598482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9068429","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e873a075ff","size":5,"soft_fail":false,"target_va":5377529888,"va":5386598312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-9674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e836daffff","size":5,"soft_fail":false,"target_va":5386588864,"va":5386598533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16381361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e8b1f5f900","size":5,"soft_fail":false,"target_va":5402980128,"va":5386598762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16381679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e8eff6f900","size":5,"soft_fail":false,"target_va":5402980304,"va":5386598620,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16381729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e821f7f900","size":5,"soft_fail":false,"target_va":5402980320,"va":5386598586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-8764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e8c4ddffff","size":5,"soft_fail":false,"target_va":5386589056,"va":5386597815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t16382759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e827fbf900","size":5,"soft_fail":false,"target_va":5402980656,"va":5386597892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ProfilerScope::new<alloc::string::String>
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-8199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e8f9dfffff","size":5,"soft_fail":false,"target_va":5386590144,"va":5386598338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10241656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386597728,"raw_hex":"e888b963ff","size":5,"soft_fail":false,"target_va":5376356144,"va":5386597795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1450,"record_offset":38968,"parent_record_offset":36860,"ancestor_record_offset":36860,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1450,"record_offset":39336,"parent_record_offset":36860,"ancestor_record_offset":36860,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1450,"record_offset":39052,"parent_record_offset":36860,"ancestor_record_offset":36860,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::dirty_index_range::DirtyIndexRange::clear
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/dirty_index_range.rs
//     evidence={"module":1450,"record_offset":39308,"parent_record_offset":36860,"ancestor_record_offset":36860,"inlinee":27271,"source_line":22,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::dirty_index_range::DirtyIndexRange::to_range
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/dirty_index_range.rs
//     evidence={"module":1450,"record_offset":37100,"parent_record_offset":36860,"ancestor_record_offset":36860,"inlinee":27272,"source_line":26,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1450,"record_offset":39132,"parent_record_offset":36860,"ancestor_record_offset":36860,"inlinee":7514,"source_line":155,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1450,"record_offset":37384,"parent_record_offset":36860,"ancestor_record_offset":36860,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1450,"record_offset":37960,"parent_record_offset":36860,"ancestor_record_offset":36860,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1450,"record_offset":39016,"parent_record_offset":38992,"ancestor_record_offset":36860,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":1450,"record_offset":37596,"parent_record_offset":37500,"ancestor_record_offset":36860,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1450,"record_offset":38552,"parent_record_offset":38092,"ancestor_record_offset":36860,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1450,"record_offset":38992,"parent_record_offset":38968,"ancestor_record_offset":36860,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1450,"record_offset":37908,"parent_record_offset":37880,"ancestor_record_offset":36860,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1450,"record_offset":38328,"parent_record_offset":38304,"ancestor_record_offset":36860,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1450,"record_offset":38304,"parent_record_offset":38280,"ancestor_record_offset":36860,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1450,"record_offset":38512,"parent_record_offset":38480,"ancestor_record_offset":36860,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1450,"record_offset":39264,"parent_record_offset":39164,"ancestor_record_offset":36860,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1450,"record_offset":37532,"parent_record_offset":37500,"ancestor_record_offset":36860,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1450,"record_offset":39208,"parent_record_offset":39164,"ancestor_record_offset":36860,"inlinee":7501,"source_line":157,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1450,"record_offset":37560,"parent_record_offset":37532,"ancestor_record_offset":36860,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1450,"record_offset":39084,"parent_record_offset":39052,"ancestor_record_offset":36860,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1450,"record_offset":38280,"parent_record_offset":38256,"ancestor_record_offset":36860,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1450,"record_offset":38608,"parent_record_offset":38584,"ancestor_record_offset":36860,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1450,"record_offset":38256,"parent_record_offset":38148,"ancestor_record_offset":36860,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1450,"record_offset":37500,"parent_record_offset":37472,"ancestor_record_offset":36860,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1450,"record_offset":37988,"parent_record_offset":37960,"ancestor_record_offset":36860,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1450,"record_offset":38120,"parent_record_offset":38092,"ancestor_record_offset":36860,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1450,"record_offset":38092,"parent_record_offset":38060,"ancestor_record_offset":36860,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1450,"record_offset":38036,"parent_record_offset":38012,"ancestor_record_offset":36860,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1450,"record_offset":38060,"parent_record_offset":38036,"ancestor_record_offset":36860,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1450,"record_offset":38012,"parent_record_offset":37988,"ancestor_record_offset":36860,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1450,"record_offset":38584,"parent_record_offset":38060,"ancestor_record_offset":36860,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":37696,"parent_record_offset":37664,"ancestor_record_offset":36860,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":37856,"parent_record_offset":37696,"ancestor_record_offset":36860,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":37664,"parent_record_offset":37640,"ancestor_record_offset":36860,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":37416,"parent_record_offset":37384,"ancestor_record_offset":36860,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":37880,"parent_record_offset":37856,"ancestor_record_offset":36860,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":37472,"parent_record_offset":37440,"ancestor_record_offset":36860,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":37440,"parent_record_offset":37416,"ancestor_record_offset":36860,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1450,"record_offset":38148,"parent_record_offset":38120,"ancestor_record_offset":36860,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1450,"record_offset":38396,"parent_record_offset":38148,"ancestor_record_offset":36860,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1450,"record_offset":38368,"parent_record_offset":38148,"ancestor_record_offset":36860,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1450,"record_offset":38200,"parent_record_offset":38148,"ancestor_record_offset":36860,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1450,"record_offset":37152,"parent_record_offset":37128,"ancestor_record_offset":36860,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1450,"record_offset":37176,"parent_record_offset":37152,"ancestor_record_offset":36860,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::dirty_index_range::DirtyIndexRange::is_empty
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/dirty_index_range.rs
//     evidence={"module":1450,"record_offset":37128,"parent_record_offset":37100,"ancestor_record_offset":36860,"inlinee":27263,"source_line":18,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1450,"record_offset":39164,"parent_record_offset":39132,"ancestor_record_offset":36860,"inlinee":7500,"source_line":207,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::bytes::slice_as_bytes
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":1450,"record_offset":39236,"parent_record_offset":39164,"ancestor_record_offset":36860,"inlinee":7323,"source_line":8,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::resources::placement_data::PlacementDataSsbo>
// 记录形式: compiled_procedure; instances=1
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1450,"record_offset":29312,"rva":17886192,"va":5386595312}
// 编译实例 RVAs: 0x110ebf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bd730
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13965589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386595312,"raw_hex":"e8ebe62aff","size":5,"soft_fail":false,"target_va":5372630832,"va":5386596416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140739e50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadowed_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10309804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386595312,"raw_hex":"e854af62ff","size":5,"soft_fail":false,"target_va":5376286288,"va":5386596087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9066013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386595312,"raw_hex":"e8e3a975ff","size":5,"soft_fail":false,"target_va":5377529888,"va":5386595896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-7279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386595312,"raw_hex":"e891e3ffff","size":5,"soft_fail":false,"target_va":5386588864,"va":5386596138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16383756","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386595312,"raw_hex":"e80cfff900","size":5,"soft_fail":false,"target_va":5402980128,"va":5386596367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16384074","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386595312,"raw_hex":"e84a00fa00","size":5,"soft_fail":false,"target_va":5402980304,"va":5386596225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16384124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386595312,"raw_hex":"e87c00fa00","size":5,"soft_fail":false,"target_va":5402980320,"va":5386596191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386595312,"raw_hex":"e834e7ffff","size":5,"soft_fail":false,"target_va":5386589056,"va":5386595399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t16385175","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386595312,"raw_hex":"e89704fa00","size":5,"soft_fail":false,"target_va":5402980656,"va":5386595476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ProfilerScope::new<alloc::string::String>
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386595312,"raw_hex":"e869e9ffff","size":5,"soft_fail":false,"target_va":5386590144,"va":5386595922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10239240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386595312,"raw_hex":"e8f8c263ff","size":5,"soft_fail":false,"target_va":5376356144,"va":5386595379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1450,"record_offset":31416,"parent_record_offset":29312,"ancestor_record_offset":29312,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1450,"record_offset":31788,"parent_record_offset":29312,"ancestor_record_offset":29312,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1450,"record_offset":31500,"parent_record_offset":29312,"ancestor_record_offset":29312,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::dirty_index_range::DirtyIndexRange::clear
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/dirty_index_range.rs
//     evidence={"module":1450,"record_offset":31760,"parent_record_offset":29312,"ancestor_record_offset":29312,"inlinee":27271,"source_line":22,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::dirty_index_range::DirtyIndexRange::to_range
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/dirty_index_range.rs
//     evidence={"module":1450,"record_offset":29548,"parent_record_offset":29312,"ancestor_record_offset":29312,"inlinee":27272,"source_line":26,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1450,"record_offset":31580,"parent_record_offset":29312,"ancestor_record_offset":29312,"inlinee":7514,"source_line":155,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1450,"record_offset":29832,"parent_record_offset":29312,"ancestor_record_offset":29312,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1450,"record_offset":30408,"parent_record_offset":29312,"ancestor_record_offset":29312,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1450,"record_offset":31464,"parent_record_offset":31440,"ancestor_record_offset":29312,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":1450,"record_offset":30044,"parent_record_offset":29948,"ancestor_record_offset":29312,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1450,"record_offset":31000,"parent_record_offset":30540,"ancestor_record_offset":29312,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1450,"record_offset":31440,"parent_record_offset":31416,"ancestor_record_offset":29312,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1450,"record_offset":30356,"parent_record_offset":30328,"ancestor_record_offset":29312,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1450,"record_offset":30776,"parent_record_offset":30752,"ancestor_record_offset":29312,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1450,"record_offset":30752,"parent_record_offset":30728,"ancestor_record_offset":29312,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1450,"record_offset":30960,"parent_record_offset":30928,"ancestor_record_offset":29312,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1450,"record_offset":31716,"parent_record_offset":31612,"ancestor_record_offset":29312,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1450,"record_offset":29980,"parent_record_offset":29948,"ancestor_record_offset":29312,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1450,"record_offset":31660,"parent_record_offset":31612,"ancestor_record_offset":29312,"inlinee":7501,"source_line":157,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1450,"record_offset":30008,"parent_record_offset":29980,"ancestor_record_offset":29312,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1450,"record_offset":31532,"parent_record_offset":31500,"ancestor_record_offset":29312,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1450,"record_offset":30728,"parent_record_offset":30704,"ancestor_record_offset":29312,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1450,"record_offset":31056,"parent_record_offset":31032,"ancestor_record_offset":29312,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1450,"record_offset":30704,"parent_record_offset":30596,"ancestor_record_offset":29312,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1450,"record_offset":29948,"parent_record_offset":29920,"ancestor_record_offset":29312,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1450,"record_offset":30436,"parent_record_offset":30408,"ancestor_record_offset":29312,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1450,"record_offset":30568,"parent_record_offset":30540,"ancestor_record_offset":29312,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1450,"record_offset":30540,"parent_record_offset":30508,"ancestor_record_offset":29312,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1450,"record_offset":30484,"parent_record_offset":30460,"ancestor_record_offset":29312,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1450,"record_offset":30508,"parent_record_offset":30484,"ancestor_record_offset":29312,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1450,"record_offset":30460,"parent_record_offset":30436,"ancestor_record_offset":29312,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1450,"record_offset":31032,"parent_record_offset":30508,"ancestor_record_offset":29312,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":30144,"parent_record_offset":30112,"ancestor_record_offset":29312,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":30304,"parent_record_offset":30144,"ancestor_record_offset":29312,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":30112,"parent_record_offset":30088,"ancestor_record_offset":29312,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":29864,"parent_record_offset":29832,"ancestor_record_offset":29312,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":30328,"parent_record_offset":30304,"ancestor_record_offset":29312,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":29920,"parent_record_offset":29888,"ancestor_record_offset":29312,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1450,"record_offset":29888,"parent_record_offset":29864,"ancestor_record_offset":29312,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1450,"record_offset":30596,"parent_record_offset":30568,"ancestor_record_offset":29312,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1450,"record_offset":30844,"parent_record_offset":30596,"ancestor_record_offset":29312,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1450,"record_offset":30816,"parent_record_offset":30596,"ancestor_record_offset":29312,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1450,"record_offset":30648,"parent_record_offset":30596,"ancestor_record_offset":29312,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1450,"record_offset":29600,"parent_record_offset":29576,"ancestor_record_offset":29312,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1450,"record_offset":29624,"parent_record_offset":29600,"ancestor_record_offset":29312,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::dirty_index_range::DirtyIndexRange::is_empty
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/dirty_index_range.rs
//     evidence={"module":1450,"record_offset":29576,"parent_record_offset":29548,"ancestor_record_offset":29312,"inlinee":27263,"source_line":18,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":1450,"record_offset":31612,"parent_record_offset":31580,"ancestor_record_offset":29312,"inlinee":7500,"source_line":207,"archive":"_pdb/A/modules/01450.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::bytes::slice_as_bytes
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":1450,"record_offset":31688,"parent_record_offset":31612,"ancestor_record_offset":29312,"inlinee":7323,"source_line":8,"archive":"_pdb/A/modules/01450.jsonl.gz"}
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
// 完整结构及原始类型记录: shadowed_buffer.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::shadowed_buffer::ShadowedBuffer
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 7; 过程引用数: 22
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::shadowed_buffer::ShadowedBuffer<country_core::render::nani::buffers::GpuInstance>", "constructor": "country_core::render::shadowed_buffer::ShadowedBuffer", "arguments": ["country_core::render::nani::buffers::GpuInstance"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::render::shadowed_buffer::ShadowedBuffer<country_core::render::nani::buffers::GpuInstance> >", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::render::shadowed_buffer::ShadowedBuffer<country_core::render::nani::buffers::GpuInstance>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::render::nani::instance_data::SolidVertexColorBricksInstanceData>", "constructor": "country_core::render::shadowed_buffer::ShadowedBuffer::new", "arguments": ["country_core::render::nani::instance_data::SolidVertexColorBricksInstanceData"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::render::nani::instance_data::GothicWindowBricksInstanceData>", "constructor": "country_core::render::shadowed_buffer::ShadowedBuffer::new", "arguments": ["country_core::render::nani::instance_data::GothicWindowBricksInstanceData"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData>", "constructor": "country_core::render::shadowed_buffer::ShadowedBuffer::new", "arguments": ["country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::render::nani::buffers::GpuInstance>", "constructor": "country_core::render::shadowed_buffer::ShadowedBuffer::new", "arguments": ["country_core::render::nani::buffers::GpuInstance"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::render::nani::instance_data::StoneFloorInstanceData>", "constructor": "country_core::render::shadowed_buffer::ShadowedBuffer::new", "arguments": ["country_core::render::nani::instance_data::StoneFloorInstanceData"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::render::nani::instance_data::PreplacedFlowerInstanceData>", "constructor": "country_core::render::shadowed_buffer::ShadowedBuffer::new", "arguments": ["country_core::render::nani::instance_data::PreplacedFlowerInstanceData"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::render::nani::instance_data::PlantInstanceData>", "constructor": "country_core::render::shadowed_buffer::ShadowedBuffer::new", "arguments": ["country_core::render::nani::instance_data::PlantInstanceData"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::resources::placement_data::PlacementDataSsbo>", "constructor": "country_core::render::shadowed_buffer::ShadowedBuffer::new", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData>", "constructor": "country_core::render::shadowed_buffer::ShadowedBuffer::new", "arguments": ["country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::render::nani::instance_data::SolidAlbedoMapInstanceData>", "constructor": "country_core::render::shadowed_buffer::ShadowedBuffer::new", "arguments": ["country_core::render::nani::instance_data::SolidAlbedoMapInstanceData"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::render::nani::instance_data::SolidVertexColorInstanceData>", "constructor": "country_core::render::shadowed_buffer::ShadowedBuffer::new", "arguments": ["country_core::render::nani::instance_data::SolidVertexColorInstanceData"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::render::nani::instance_data::WrenInstanceData>", "constructor": "country_core::render::shadowed_buffer::ShadowedBuffer::new", "arguments": ["country_core::render::nani::instance_data::WrenInstanceData"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::PreplacedFlowerInstanceData>", "constructor": "country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush", "arguments": ["country_core::render::nani::instance_data::PreplacedFlowerInstanceData"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::buffers::GpuInstance>", "constructor": "country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush", "arguments": ["country_core::render::nani::buffers::GpuInstance"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData>", "constructor": "country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush", "arguments": ["country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData>", "constructor": "country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush", "arguments": ["country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::resources::placement_data::PlacementDataSsbo>", "constructor": "country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush", "arguments": ["country_core::resources::placement_data::PlacementDataSsbo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::StoneFloorInstanceData>", "constructor": "country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush", "arguments": ["country_core::render::nani::instance_data::StoneFloorInstanceData"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::SolidVertexColorBricksInstanceData>", "constructor": "country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush", "arguments": ["country_core::render::nani::instance_data::SolidVertexColorBricksInstanceData"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::WrenInstanceData>", "constructor": "country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush", "arguments": ["country_core::render::nani::instance_data::WrenInstanceData"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::GothicWindowBricksInstanceData>", "constructor": "country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush", "arguments": ["country_core::render::nani::instance_data::GothicWindowBricksInstanceData"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: shadowed_buffer.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: shadowed_buffer.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::render::shadowed_buffer::ShadowedBuffer::buffer
// 信息 ID: 79aa229453a398e594224f8c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists; module=451, inline_record=14168; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::impl$14::import_to_rg<country_core::render::nani::instance_data::SolidVertexColorInstanceData>; module=1371, inline_record=74812; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::shadowed_buffer::ShadowedBuffer::copy
// 信息 ID: 67f132b26f2b59903bc42955
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::remove_instance; module=455, inline_record=12372; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::impl$14::copy<country_core::render::nani::instance_data::PreplacedFlowerInstanceData>; module=1371, inline_record=75452; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::impl$14::copy<country_core::render::nani::instance_data::GothicWindowBricksInstanceData>; module=1371, inline_record=75848; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::impl$14::copy<country_core::resources::placement_data::PlacementDataSsbo>; module=1371, inline_record=76228; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::impl$14::copy<country_core::render::nani::instance_data::StoneFloorInstanceData>; module=1371, inline_record=76616; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::impl$14::copy<country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData>; module=1371, inline_record=77016; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::impl$14::copy<country_core::render::nani::instance_data::WrenInstanceData>; module=1371, inline_record=77400; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::impl$14::copy<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData>; module=1371, inline_record=77804; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::shadowed_buffer::ShadowedBuffer::get_mut
// 信息 ID: 88b0f7c113121e9b344a7def
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 12 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::update_instance_mesh; module=455, inline_record=11700; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::update_instance_data<country_core::render::nani::instance_data::SolidVertexColorInstanceData>; module=1371, inline_record=16736; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::update_instance_data<country_core::render::nani::instance_data::SolidVertexColorBricksInstanceData>; module=1371, inline_record=19148; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::update_instance_data<country_core::resources::placement_data::PlacementDataSsbo>; module=1371, inline_record=21544; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::update_instance_data<country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData>; module=1371, inline_record=23956; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::update_instance_data<country_core::render::nani::instance_data::PlantInstanceData>; module=1371, inline_record=26352; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::update_instance_data<country_core::render::nani::instance_data::WrenInstanceData>; module=1371, inline_record=28748; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::update_instance_data<country_core::render::nani::instance_data::SolidAlbedoMapInstanceData>; module=1371, inline_record=31152; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::buffers::GpuInstance>
// 信息 ID: 6620afec43e78bfe7d556399
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush -> ["country_core::render::nani::buffers::GpuInstance"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 17 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::nani::systems::nani_flush_instance_updates @ 0x181e61f -> 0x110f0b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::systems::nani_flush_instance_updates @ 0x181e64b -> 0x110f0b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::systems::nani_flush_instance_updates @ 0x181e677 -> 0x110f0b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::systems::nani_flush_instance_updates @ 0x181e6a3 -> 0x110f0b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::systems::nani_flush_instance_updates @ 0x181e6cf -> 0x110f0b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::systems::nani_flush_instance_updates @ 0x181e6fb -> 0x110f0b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::systems::nani_flush_instance_updates @ 0x181e727 -> 0x110f0b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::systems::nani_flush_instance_updates @ 0x181e753 -> 0x110f0b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::systems::nani_flush_instance_updates @ 0x181e77f -> 0x110f0b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::systems::nani_flush_instance_updates @ 0x181e7ab -> 0x110f0b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::systems::nani_flush_instance_updates @ 0x181e7d7 -> 0x110f0b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::systems::nani_flush_instance_updates @ 0x181e803 -> 0x110f0b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::systems::nani_flush_instance_updates @ 0x181e82f -> 0x110f0b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::systems::nani_flush_instance_updates @ 0x181e85b -> 0x110f0b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::systems::nani_flush_instance_updates @ 0x181e887 -> 0x110f0b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::systems::nani_flush_instance_updates @ 0x181e8b3 -> 0x110f0b0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::GothicWindowBricksInstanceData>
// 信息 ID: 88aaf1fbeb10201c4119a5a8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush -> ["country_core::render::nani::instance_data::GothicWindowBricksInstanceData"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::nani::gpu_world::impl$14::gpu_flush<country_core::render::nani::instance_data::GothicWindowBricksInstanceData> @ 0xf3f044 -> 0x110fa10 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::PreplacedFlowerInstanceData>
// 信息 ID: 2794ec073d0cb0df056b645e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush -> ["country_core::render::nani::instance_data::PreplacedFlowerInstanceData"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::nani::gpu_world::impl$14::gpu_flush<country_core::render::nani::instance_data::PlantInstanceData> | country_core::render::nani::gpu_world::impl$14::gpu_flush<country_core::render::nani::instance_data::StoneFloorInstanceData> | country_core::render::nani::gpu_world::impl$14::gpu_flush<country_core::render::nani::instance_data::PreplacedFlowerInstanceData> | country_core::render::nani::gpu_world::impl$14::gpu_flush<country_core::resources::placement_data::PlacementDataSsbo> @ 0xf3f034 -> 0x110ebf0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::SolidVertexColorBricksInstanceData>
// 信息 ID: 739970284e1bf1db33803ee9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush -> ["country_core::render::nani::instance_data::SolidVertexColorBricksInstanceData"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::nani::gpu_world::impl$14::gpu_flush<country_core::render::nani::instance_data::SolidVertexColorInstanceData> | country_core::render::nani::gpu_world::impl$14::gpu_flush<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData> | country_core::render::nani::gpu_world::impl$14::gpu_flush<country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData> | country_core::render::nani::gpu_world::impl$14::gpu_flush<country_core::render::nani::instance_data::WrenInstanceData> | 其余 2 个同地址别名见共享索引 @ 0xf3f024 -> 0x110f560 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData>
// 信息 ID: c16178abbb90e7498c8d9d2f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush -> ["country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::nani::gpu_world::impl$14::gpu_flush<country_core::render::nani::instance_data::SolidVertexColorInstanceData> | country_core::render::nani::gpu_world::impl$14::gpu_flush<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData> | country_core::render::nani::gpu_world::impl$14::gpu_flush<country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData> | country_core::render::nani::gpu_world::impl$14::gpu_flush<country_core::render::nani::instance_data::WrenInstanceData> | 其余 2 个同地址别名见共享索引 @ 0xf3f024 -> 0x110f560 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData>
// 信息 ID: 3435e956176de63712bbc5fe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush -> ["country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::nani::gpu_world::impl$14::gpu_flush<country_core::render::nani::instance_data::SolidVertexColorInstanceData> | country_core::render::nani::gpu_world::impl$14::gpu_flush<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData> | country_core::render::nani::gpu_world::impl$14::gpu_flush<country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData> | country_core::render::nani::gpu_world::impl$14::gpu_flush<country_core::render::nani::instance_data::WrenInstanceData> | 其余 2 个同地址别名见共享索引 @ 0xf3f024 -> 0x110f560 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::StoneFloorInstanceData>
// 信息 ID: 1e359985c4828a039cf56603
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush -> ["country_core::render::nani::instance_data::StoneFloorInstanceData"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::nani::gpu_world::impl$14::gpu_flush<country_core::render::nani::instance_data::PlantInstanceData> | country_core::render::nani::gpu_world::impl$14::gpu_flush<country_core::render::nani::instance_data::StoneFloorInstanceData> | country_core::render::nani::gpu_world::impl$14::gpu_flush<country_core::render::nani::instance_data::PreplacedFlowerInstanceData> | country_core::render::nani::gpu_world::impl$14::gpu_flush<country_core::resources::placement_data::PlacementDataSsbo> @ 0xf3f034 -> 0x110ebf0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::render::nani::instance_data::WrenInstanceData>
// 信息 ID: 8d18c6266dc40c79ef405723
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush -> ["country_core::render::nani::instance_data::WrenInstanceData"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::nani::gpu_world::impl$14::gpu_flush<country_core::render::nani::instance_data::SolidVertexColorInstanceData> | country_core::render::nani::gpu_world::impl$14::gpu_flush<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData> | country_core::render::nani::gpu_world::impl$14::gpu_flush<country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData> | country_core::render::nani::gpu_world::impl$14::gpu_flush<country_core::render::nani::instance_data::WrenInstanceData> | 其余 2 个同地址别名见共享索引 @ 0xf3f024 -> 0x110f560 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush<country_core::resources::placement_data::PlacementDataSsbo>
// 信息 ID: da9a145a684e14f8b9863115
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::shadowed_buffer::ShadowedBuffer::gpu_flush -> ["country_core::resources::placement_data::PlacementDataSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::nani::gpu_world::impl$14::gpu_flush<country_core::render::nani::instance_data::PlantInstanceData> | country_core::render::nani::gpu_world::impl$14::gpu_flush<country_core::render::nani::instance_data::StoneFloorInstanceData> | country_core::render::nani::gpu_world::impl$14::gpu_flush<country_core::render::nani::instance_data::PreplacedFlowerInstanceData> | country_core::render::nani::gpu_world::impl$14::gpu_flush<country_core::resources::placement_data::PlacementDataSsbo> @ 0xf3f034 -> 0x110ebf0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::render::nani::buffers::GpuInstance>
// 信息 ID: f95e0588180c49982832c105
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::shadowed_buffer::ShadowedBuffer::new -> ["country_core::render::nani::buffers::GpuInstance"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 11 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::new<country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData> @ 0xf3bac0 -> 0x110dde0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::new<country_core::render::nani::instance_data::WrenInstanceData> @ 0xf3bf80 -> 0x110dde0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::new<country_core::render::nani::instance_data::StoneFloorInstanceData> @ 0xf3c440 -> 0x110dde0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::new<country_core::render::nani::instance_data::SolidVertexColorInstanceData> @ 0xf3c8c0 -> 0x110dde0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::new<country_core::render::nani::instance_data::PlantInstanceData> @ 0xf3cd80 -> 0x110dde0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::new<country_core::render::nani::instance_data::SolidVertexColorBricksInstanceData> @ 0xf3d200 -> 0x110dde0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::new<country_core::resources::placement_data::PlacementDataSsbo> @ 0xf3d6c0 -> 0x110dde0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::new<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData> @ 0xf3dbf0 -> 0x110dde0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::new<country_core::render::nani::instance_data::SolidAlbedoMapInstanceData> @ 0xf3e0b0 -> 0x110dde0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::new<country_core::render::nani::instance_data::GothicWindowBricksInstanceData> @ 0xf3e570 -> 0x110dde0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::new<country_core::render::nani::instance_data::PreplacedFlowerInstanceData> @ 0xf3eaf0 -> 0x110dde0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::render::nani::instance_data::GothicWindowBricksInstanceData>
// 信息 ID: f5149a5202ae409999fee903
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::shadowed_buffer::ShadowedBuffer::new -> ["country_core::render::nani::instance_data::GothicWindowBricksInstanceData"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::new<country_core::render::nani::instance_data::GothicWindowBricksInstanceData> @ 0xf3e7d9 -> 0x110dad0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::render::nani::instance_data::PlantInstanceData>
// 信息 ID: 456e4e5671d845c8118fd959
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::shadowed_buffer::ShadowedBuffer::new -> ["country_core::render::nani::instance_data::PlantInstanceData"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::new<country_core::render::nani::instance_data::PlantInstanceData> @ 0xf3cee9 -> 0x110e290 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::render::nani::instance_data::PreplacedFlowerInstanceData>
// 信息 ID: f62dee4378ea4a9dcd58267e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::shadowed_buffer::ShadowedBuffer::new -> ["country_core::render::nani::instance_data::PreplacedFlowerInstanceData"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::new<country_core::render::nani::instance_data::PreplacedFlowerInstanceData> @ 0xf3ec59 -> 0x110e100 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::render::nani::instance_data::SolidAlbedoMapInstanceData>
// 信息 ID: 3577e3e97306536974ebe860
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::shadowed_buffer::ShadowedBuffer::new -> ["country_core::render::nani::instance_data::SolidAlbedoMapInstanceData"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::new<country_core::render::nani::instance_data::SolidAlbedoMapInstanceData> @ 0xf3e251 -> 0x110e740 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::render::nani::instance_data::SolidVertexColorBricksInstanceData>
// 信息 ID: 5fa11e87e070899a9f755507
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::shadowed_buffer::ShadowedBuffer::new -> ["country_core::render::nani::instance_data::SolidVertexColorBricksInstanceData"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::new<country_core::render::nani::instance_data::SolidVertexColorBricksInstanceData> @ 0xf3d3a1 -> 0x110d940 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::render::nani::instance_data::SolidVertexColorInstanceData>
// 信息 ID: 298ce10a4480177b4c834204
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::shadowed_buffer::ShadowedBuffer::new -> ["country_core::render::nani::instance_data::SolidVertexColorInstanceData"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::new<country_core::render::nani::instance_data::SolidVertexColorInstanceData> @ 0xf3ca61 -> 0x110e8d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData>
// 信息 ID: 2062e42f6b3ffc476a06cfde
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::shadowed_buffer::ShadowedBuffer::new -> ["country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::new<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData> @ 0xf3dd91 -> 0x110dc50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData>
// 信息 ID: d750f6f9333dfb8f2861c608
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::shadowed_buffer::ShadowedBuffer::new -> ["country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::new<country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData> @ 0xf3bc61 -> 0x110e5b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::render::nani::instance_data::StoneFloorInstanceData>
// 信息 ID: 6add0f7ec0c28937d4a1a327
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::shadowed_buffer::ShadowedBuffer::new -> ["country_core::render::nani::instance_data::StoneFloorInstanceData"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::new<country_core::render::nani::instance_data::StoneFloorInstanceData> @ 0xf3c5a9 -> 0x110df70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::render::nani::instance_data::WrenInstanceData>
// 信息 ID: 4ce9fd4d9fed6e35cdc3d42c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::shadowed_buffer::ShadowedBuffer::new -> ["country_core::render::nani::instance_data::WrenInstanceData"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::new<country_core::render::nani::instance_data::WrenInstanceData> @ 0xf3c121 -> 0x110ea60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::shadowed_buffer::ShadowedBuffer::new<country_core::resources::placement_data::PlacementDataSsbo>
// 信息 ID: 7a5f26bca6617a9dcb0da440
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::shadowed_buffer::ShadowedBuffer::new -> ["country_core::resources::placement_data::PlacementDataSsbo"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::new<country_core::resources::placement_data::PlacementDataSsbo> @ 0xf3d8c8 -> 0x110e420 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::shadowed_buffer::ShadowedBuffer::write
// 信息 ID: 530435efad74a5b65cc7bb99
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 14 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::nani::gpu_world::OwnedGpuWorldSubset::extend_no_data; module=455, inline_record=13960; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::create_instances<country_core::resources::placement_data::PlacementDataSsbo,country_core::systems::prefab::create_prefab_gpu_instances::closure_env$0>; module=1371, inline_record=11844; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::create_instances<country_core::resources::placement_data::PlacementDataSsbo,country_core::systems::prefab::create_prefab_gpu_instances::closure_env$0>; module=1371, inline_record=12172; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::update_instance_data<country_core::render::nani::instance_data::SolidVertexColorInstanceData>; module=1371, inline_record=17440; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::update_instance_data<country_core::render::nani::instance_data::SolidVertexColorBricksInstanceData>; module=1371, inline_record=19852; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::update_instance_data<country_core::resources::placement_data::PlacementDataSsbo>; module=1371, inline_record=22248; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::update_instance_data<country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData>; module=1371, inline_record=24660; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::gpu_world::GpuWorld::update_instance_data<country_core::render::nani::instance_data::PlantInstanceData>; module=1371, inline_record=27056; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
