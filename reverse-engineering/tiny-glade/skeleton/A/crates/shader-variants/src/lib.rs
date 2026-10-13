// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/shader-variants/src/lib.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: e7276181be8b5cf88d954854be5e608359d3fdc5013346d4fccfefdc13a8942b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1454
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use shader_pipeline; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: shader_variants::get_raster_shader_variants
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1454,"record_offset":804,"rva":17943200,"va":5386652320}
// 编译实例 RVAs: 0x111caa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..clone..Clone$GT$::clone
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t8181657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386652320,"raw_hex":"e899d77c00","size":5,"soft_fail":false,"target_va":5394834128,"va":5386652466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e16c0
//     owner=unresolved_namespace; count=4
//     target_aliases=4 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17019937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386652320,"raw_hex":"e8df4bfcfe","size":5,"soft_fail":false,"target_va":5369632448,"va":5386652380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b8730
//     owner=unresolved_namespace; count=2
//     target_aliases=26 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15092301","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386652320,"raw_hex":"e8b3b519ff","size":5,"soft_fail":false,"target_va":5371561776,"va":5386654072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13902855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386652320,"raw_hex":"e8f9db2bff","size":5,"soft_fail":false,"target_va":5372749696,"va":5386652546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13903612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386652320,"raw_hex":"e804d92bff","size":5,"soft_fail":false,"target_va":5372749712,"va":5386653319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e8c80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10699469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386652320,"raw_hex":"e833bd5cff","size":5,"soft_fail":false,"target_va":5375954048,"va":5386653512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t16324959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386652320,"raw_hex":"e85f19f900","size":5,"soft_fail":false,"target_va":5402978144,"va":5386653180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-15345013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386652320,"raw_hex":"e88bda15ff","size":5,"soft_fail":false,"target_va":5371307664,"va":5386652672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1454,"record_offset":1472,"parent_record_offset":804,"ancestor_record_offset":804,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1454,"record_offset":2464,"parent_record_offset":804,"ancestor_record_offset":804,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1454,"record_offset":4504,"parent_record_offset":804,"ancestor_record_offset":804,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::impl$14::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":1454,"record_offset":944,"parent_record_offset":804,"ancestor_record_offset":804,"inlinee":67035,"source_line":16,"archive":"_pdb/A/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1454,"record_offset":1756,"parent_record_offset":1732,"ancestor_record_offset":804,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1454,"record_offset":1780,"parent_record_offset":1756,"ancestor_record_offset":804,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1454,"record_offset":1224,"parent_record_offset":1200,"ancestor_record_offset":804,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1454,"record_offset":1732,"parent_record_offset":1704,"ancestor_record_offset":804,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1454,"record_offset":1200,"parent_record_offset":1144,"ancestor_record_offset":804,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1454,"record_offset":1172,"parent_record_offset":1144,"ancestor_record_offset":804,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1454,"record_offset":2540,"parent_record_offset":2508,"ancestor_record_offset":804,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1454,"record_offset":1704,"parent_record_offset":1676,"ancestor_record_offset":804,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1454,"record_offset":1676,"parent_record_offset":1648,"ancestor_record_offset":804,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1454,"record_offset":1852,"parent_record_offset":1676,"ancestor_record_offset":804,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1454,"record_offset":1144,"parent_record_offset":1116,"ancestor_record_offset":804,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1454,"record_offset":1612,"parent_record_offset":1584,"ancestor_record_offset":804,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1454,"record_offset":1584,"parent_record_offset":1556,"ancestor_record_offset":804,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1454,"record_offset":1528,"parent_record_offset":1500,"ancestor_record_offset":804,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1454,"record_offset":1556,"parent_record_offset":1528,"ancestor_record_offset":804,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1454,"record_offset":1500,"parent_record_offset":1472,"ancestor_record_offset":804,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1454,"record_offset":2508,"parent_record_offset":2464,"ancestor_record_offset":804,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1454,"record_offset":1648,"parent_record_offset":1612,"ancestor_record_offset":804,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1454,"record_offset":1912,"parent_record_offset":1888,"ancestor_record_offset":804,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1454,"record_offset":992,"parent_record_offset":944,"ancestor_record_offset":804,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1454,"record_offset":1888,"parent_record_offset":1612,"ancestor_record_offset":804,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1454,"record_offset":1064,"parent_record_offset":944,"ancestor_record_offset":804,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1454,"record_offset":1808,"parent_record_offset":1780,"ancestor_record_offset":804,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1454,"record_offset":2572,"parent_record_offset":2464,"ancestor_record_offset":804,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::shader_defines::impl$1::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/shader_defines.rs
//     evidence={"module":1454,"record_offset":1032,"parent_record_offset":944,"ancestor_record_offset":804,"inlinee":67037,"source_line":3,"archive":"_pdb/A/modules/01454.jsonl.gz"}
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
// 完整结构及原始类型记录: lib.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: lib.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: lib.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: shader_variants::get_raster_shader_variants
// 信息 ID: 3fb936dc1d4e4b8aacb5cf57
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::shader::compile_shader_variants @ 0x650498 -> 0x111caa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
