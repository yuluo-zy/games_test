// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/render/pass_builder/shader_uri.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: c7b8c6cff3e4d0519b9c0eb758e889aafa6e541ee45c1d9a8f9e1afd7a273aea
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: shader_uri.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 56, 115, 136, 390, 400, 413, 418, 451, 462, 489, 495, 1318, 1370
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use shader_pipeline; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::shader_uri::ShaderUri::parse
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":418,"record_offset":10896,"rva":5850768,"va":5374559888}
// 编译实例 RVAs: 0x594690
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28418676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374559888,"raw_hex":"e874a2b101","size":5,"soft_fail":false,"target_va":5402978880,"va":5374560199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::memchr::memrchr
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t3029870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374559888,"raw_hex":"e86e3b2e00","size":5,"soft_fail":false,"target_va":5377589808,"va":5374559933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28420426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374559888,"raw_hex":"e84aa9b101","size":5,"soft_fail":false,"target_va":5402980656,"va":5374560225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":418,"record_offset":12492,"parent_record_offset":10896,"ancestor_record_offset":10896,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::rfind
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":418,"record_offset":11052,"parent_record_offset":10896,"ancestor_record_offset":10896,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":418,"record_offset":11320,"parent_record_offset":10896,"ancestor_record_offset":10896,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":418,"record_offset":11228,"parent_record_offset":11076,"ancestor_record_offset":10896,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":418,"record_offset":11504,"parent_record_offset":11420,"ancestor_record_offset":10896,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":418,"record_offset":11588,"parent_record_offset":11560,"ancestor_record_offset":10896,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":418,"record_offset":11252,"parent_record_offset":11228,"ancestor_record_offset":10896,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":418,"record_offset":11276,"parent_record_offset":11252,"ancestor_record_offset":10896,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":418,"record_offset":11108,"parent_record_offset":11076,"ancestor_record_offset":10896,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":418,"record_offset":11472,"parent_record_offset":11448,"ancestor_record_offset":10896,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":418,"record_offset":11140,"parent_record_offset":11108,"ancestor_record_offset":10896,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":418,"record_offset":11420,"parent_record_offset":11388,"ancestor_record_offset":10896,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$2::next_match_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":418,"record_offset":11076,"parent_record_offset":11052,"ancestor_record_offset":10896,"inlinee":16738,"source_line":485,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":418,"record_offset":11388,"parent_record_offset":11352,"ancestor_record_offset":10896,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":418,"record_offset":11536,"parent_record_offset":11388,"ancestor_record_offset":10896,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":418,"record_offset":11352,"parent_record_offset":11320,"ancestor_record_offset":10896,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":418,"record_offset":11560,"parent_record_offset":11536,"ancestor_record_offset":10896,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00418.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::shader_uri::impl$1::source_hash
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1F12
// 首条位置: {"module":115,"inlinee":7954}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":115,"record_offset":3656,"parent_record_offset":3632,"ancestor_record_offset":3632,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash::compute
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":115,"record_offset":604,"parent_record_offset":580,"ancestor_record_offset":580,"inlinee":7935,"source_line":220,"archive":"_pdb/A/modules/00115.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::shader_uri::impl$1::source_hash<ref$<str$> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":390,"record_offset":10988,"rva":5321056,"va":5374030176}
// 编译实例 RVAs: 0x513160
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ddcf0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4412562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374030176,"raw_hex":"e86eabbcff","size":5,"soft_fail":false,"target_va":5369617648,"va":5374030205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash::compute
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":390,"record_offset":11152,"parent_record_offset":10988,"ancestor_record_offset":10988,"inlinee":7935,"source_line":220,"archive":"_pdb/A/modules/00390.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::shader_uri::impl$1::as_compute_pipeline<ref$<alloc::string::String> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":56,"record_offset":8456,"rva":504848,"va":5369213968}
// 编译实例 RVAs: 0x7b410
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402775e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2080987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369213968,"raw_hex":"e8dbc01f00","size":5,"soft_fail":false,"target_va":5371295200,"va":5369214208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3535637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369213968,"raw_hex":"e815f33500","size":5,"soft_fail":false,"target_va":5372749696,"va":5369214054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3535376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369213968,"raw_hex":"e810f23500","size":5,"soft_fail":false,"target_va":5372749712,"va":5369214331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34379267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369213968,"raw_hex":"e803960c02","size":5,"soft_fail":false,"target_va":5403593360,"va":5369214088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33763891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369213968,"raw_hex":"e833320302","size":5,"soft_fail":false,"target_va":5402978144,"va":5369214248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8312860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369213968,"raw_hex":"e81cd87e00","size":5,"soft_fail":false,"target_va":5377527120,"va":5369214255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::shader_uri::ShaderUri::parse
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5345868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369213968,"raw_hex":"e84c925100","size":5,"soft_fail":false,"target_va":5374559888,"va":5369214015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":56,"record_offset":8684,"parent_record_offset":8456,"ancestor_record_offset":8456,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":56,"record_offset":8656,"parent_record_offset":8456,"ancestor_record_offset":8456,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":56,"record_offset":9196,"parent_record_offset":8456,"ancestor_record_offset":8456,"inlinee":7937,"source_line":55,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":56,"record_offset":8984,"parent_record_offset":8960,"ancestor_record_offset":8456,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":56,"record_offset":9008,"parent_record_offset":8984,"ancestor_record_offset":8456,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":56,"record_offset":9880,"parent_record_offset":9856,"ancestor_record_offset":8456,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":56,"record_offset":8960,"parent_record_offset":8916,"ancestor_record_offset":8456,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":56,"record_offset":9856,"parent_record_offset":9800,"ancestor_record_offset":8456,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":56,"record_offset":9828,"parent_record_offset":9800,"ancestor_record_offset":8456,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":56,"record_offset":8916,"parent_record_offset":8888,"ancestor_record_offset":8456,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":56,"record_offset":8888,"parent_record_offset":8860,"ancestor_record_offset":8456,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":56,"record_offset":9076,"parent_record_offset":8888,"ancestor_record_offset":8456,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":56,"record_offset":9800,"parent_record_offset":9776,"ancestor_record_offset":8456,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":56,"record_offset":8824,"parent_record_offset":8796,"ancestor_record_offset":8456,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":56,"record_offset":8796,"parent_record_offset":8768,"ancestor_record_offset":8456,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":56,"record_offset":8740,"parent_record_offset":8712,"ancestor_record_offset":8456,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":56,"record_offset":8768,"parent_record_offset":8740,"ancestor_record_offset":8456,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":56,"record_offset":8712,"parent_record_offset":8684,"ancestor_record_offset":8456,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":56,"record_offset":9240,"parent_record_offset":9196,"ancestor_record_offset":8456,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":56,"record_offset":8860,"parent_record_offset":8824,"ancestor_record_offset":8456,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":56,"record_offset":9140,"parent_record_offset":9116,"ancestor_record_offset":8456,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":56,"record_offset":9116,"parent_record_offset":8824,"ancestor_record_offset":8456,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":56,"record_offset":9728,"parent_record_offset":9196,"ancestor_record_offset":8456,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":56,"record_offset":9032,"parent_record_offset":9008,"ancestor_record_offset":8456,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::shader_uri::impl$1::as_compute_pipeline<ref$<str$> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":136,"record_offset":14724,"rva":2520400,"va":5371229520}
// 编译实例 RVAs: 0x267550, 0x513190
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402775e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t65436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371229520,"raw_hex":"e89cff0000","size":5,"soft_fail":false,"target_va":5371295200,"va":5371229759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1520086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371229520,"raw_hex":"e8d6311700","size":5,"soft_fail":false,"target_va":5372749696,"va":5371229605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1519824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371229520,"raw_hex":"e8d0301700","size":5,"soft_fail":false,"target_va":5372749712,"va":5371229883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32363716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371229520,"raw_hex":"e8c4d4ed01","size":5,"soft_fail":false,"target_va":5403593360,"va":5371229639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t31748340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371229520,"raw_hex":"e8f470e401","size":5,"soft_fail":false,"target_va":5402978144,"va":5371229799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6297309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371229520,"raw_hex":"e8dd166000","size":5,"soft_fail":false,"target_va":5377527120,"va":5371229806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::shader_uri::ShaderUri::parse
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t3330317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371229520,"raw_hex":"e80dd13200","size":5,"soft_fail":false,"target_va":5374559888,"va":5371229566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":136,"record_offset":14900,"parent_record_offset":14724,"ancestor_record_offset":14724,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":136,"record_offset":15412,"parent_record_offset":14724,"ancestor_record_offset":14724,"inlinee":7937,"source_line":55,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":136,"record_offset":15200,"parent_record_offset":15176,"ancestor_record_offset":14724,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":136,"record_offset":15224,"parent_record_offset":15200,"ancestor_record_offset":14724,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":136,"record_offset":16096,"parent_record_offset":16072,"ancestor_record_offset":14724,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":136,"record_offset":15176,"parent_record_offset":15132,"ancestor_record_offset":14724,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":136,"record_offset":16072,"parent_record_offset":16016,"ancestor_record_offset":14724,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":136,"record_offset":16044,"parent_record_offset":16016,"ancestor_record_offset":14724,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":136,"record_offset":15132,"parent_record_offset":15104,"ancestor_record_offset":14724,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":136,"record_offset":15104,"parent_record_offset":15076,"ancestor_record_offset":14724,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":136,"record_offset":15292,"parent_record_offset":15104,"ancestor_record_offset":14724,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":136,"record_offset":16016,"parent_record_offset":15992,"ancestor_record_offset":14724,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":136,"record_offset":15040,"parent_record_offset":15012,"ancestor_record_offset":14724,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":136,"record_offset":15012,"parent_record_offset":14984,"ancestor_record_offset":14724,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":136,"record_offset":14956,"parent_record_offset":14928,"ancestor_record_offset":14724,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":136,"record_offset":14984,"parent_record_offset":14956,"ancestor_record_offset":14724,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":136,"record_offset":14928,"parent_record_offset":14900,"ancestor_record_offset":14724,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":136,"record_offset":15456,"parent_record_offset":15412,"ancestor_record_offset":14724,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":136,"record_offset":15076,"parent_record_offset":15040,"ancestor_record_offset":14724,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":136,"record_offset":15356,"parent_record_offset":15332,"ancestor_record_offset":14724,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":136,"record_offset":15332,"parent_record_offset":15040,"ancestor_record_offset":14724,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":136,"record_offset":15944,"parent_record_offset":15412,"ancestor_record_offset":14724,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":136,"record_offset":15248,"parent_record_offset":15224,"ancestor_record_offset":14724,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00136.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::shader_uri::impl$2::source_hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5E43
// 首条位置: {"module":1318,"inlinee":24131}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::shader_uri::impl$2::as_compute_pipeline
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5E44
// 首条位置: {"module":1318,"inlinee":24132}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::impl$15::hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 100
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6C90
// 首条位置: {"module":495,"inlinee":27792}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::hash::impls::impl$16::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":3564,"parent_record_offset":3540,"ancestor_record_offset":3540,"inlinee":8468,"source_line":806,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_i32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":3588,"parent_record_offset":3564,"ancestor_record_offset":3540,"inlinee":8469,"source_line":414,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":3612,"parent_record_offset":3588,"ancestor_record_offset":3540,"inlinee":8446,"source_line":377,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":3692,"parent_record_offset":3668,"ancestor_record_offset":3540,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":3636,"parent_record_offset":3612,"ancestor_record_offset":3540,"inlinee":8447,"source_line":2859,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":3668,"parent_record_offset":3612,"ancestor_record_offset":3540,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::dof::impl$11::source_hash
// 记录形式: compiled_procedure; instances=1
// 源码行: 113
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":400,"record_offset":69800,"rva":5565856,"va":5374274976}
// 编译实例 RVAs: 0x54eda0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1406c0f90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1515976","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374274976,"raw_hex":"e8c8211700","size":5,"soft_fail":false,"target_va":5375790992,"va":5374275011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash::compute
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":400,"record_offset":69964,"parent_record_offset":69800,"ancestor_record_offset":69800,"inlinee":7935,"source_line":220,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::dof::impl$15::source_hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 113
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5A0D
// 首条位置: {"module":1318,"inlinee":23053}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash::compute
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":1318,"record_offset":61924,"parent_record_offset":61900,"ancestor_record_offset":61900,"inlinee":7935,"source_line":220,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::dof::impl$19::source_hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 113
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5A10
// 首条位置: {"module":1318,"inlinee":23056}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash::compute
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":1318,"record_offset":49224,"parent_record_offset":49200,"ancestor_record_offset":49200,"inlinee":7935,"source_line":220,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::dof::impl$3::source_hash
// 记录形式: compiled_procedure; instances=1
// 源码行: 113
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":400,"record_offset":62304,"rva":5563552,"va":5374272672}
// 编译实例 RVAs: 0x54e4a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1406c0f90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1518280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374272672,"raw_hex":"e8c82a1700","size":5,"soft_fail":false,"target_va":5375790992,"va":5374272707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash::compute
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":400,"record_offset":62464,"parent_record_offset":62304,"ancestor_record_offset":62304,"inlinee":7935,"source_line":220,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::dof::impl$7::source_hash
// 记录形式: compiled_procedure; instances=1
// 源码行: 113
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":400,"record_offset":66052,"rva":5564704,"va":5374273824}
// 编译实例 RVAs: 0x54e920
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1406c0f90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1517128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374273824,"raw_hex":"e848261700","size":5,"soft_fail":false,"target_va":5375790992,"va":5374273859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash::compute
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":400,"record_offset":66212,"parent_record_offset":66052,"ancestor_record_offset":66052,"inlinee":7935,"source_line":220,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::impl$10::source_hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 113
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6529
// 首条位置: {"module":1318,"inlinee":25897}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash::compute
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":1318,"record_offset":33388,"parent_record_offset":33364,"ancestor_record_offset":33364,"inlinee":7935,"source_line":220,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::impl$14::source_hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 113
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x652C
// 首条位置: {"module":1318,"inlinee":25900}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash::compute
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":1318,"record_offset":39780,"parent_record_offset":39756,"ancestor_record_offset":39756,"inlinee":7935,"source_line":220,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::impl$18::source_hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 113
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x652F
// 首条位置: {"module":1318,"inlinee":25903}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash::compute
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":1318,"record_offset":36588,"parent_record_offset":36564,"ancestor_record_offset":36564,"inlinee":7935,"source_line":220,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::impl$2::source_hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 113
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6523
// 首条位置: {"module":1318,"inlinee":25891}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash::compute
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":1318,"record_offset":58764,"parent_record_offset":58740,"ancestor_record_offset":58740,"inlinee":7935,"source_line":220,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::impl$6::source_hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 113
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6526
// 首条位置: {"module":1318,"inlinee":25894}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash::compute
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":1318,"record_offset":55572,"parent_record_offset":55548,"ancestor_record_offset":55548,"inlinee":7935,"source_line":220,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::vermeer::impl$7::source_hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 113
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6734
// 首条位置: {"module":1318,"inlinee":26420}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash::compute
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":1318,"record_offset":52376,"parent_record_offset":52352,"ancestor_record_offset":52352,"inlinee":7935,"source_line":220,"archive":"_pdb/A/modules/01318.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::dof::impl$11::as_compute_pipeline
// 记录形式: compiled_procedure; instances=1
// 源码行: 120
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":400,"record_offset":69996,"rva":5565904,"va":5374275024}
// 编译实例 RVAs: 0x54edd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b8730
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2714335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374275024,"raw_hex":"e82195d6ff","size":5,"soft_fail":false,"target_va":5371561776,"va":5374276106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140387ca0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1864879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374275024,"raw_hex":"e8518be3ff","size":5,"soft_fail":false,"target_va":5372411040,"va":5374275914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1525388","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374275024,"raw_hex":"e874b9e8ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374275079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1525588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374275024,"raw_hex":"e8acb8e8ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374275295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t28702431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374275024,"raw_hex":"e8dff6b501","size":5,"soft_fail":false,"target_va":5402978144,"va":5374275708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2967929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374275024,"raw_hex":"e887b6d2ff","size":5,"soft_fail":false,"target_va":5371307664,"va":5374275588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$::fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3334797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374275024,"raw_hex":"e88de23200","size":5,"soft_fail":false,"target_va":5377610320,"va":5374275518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28704317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374275024,"raw_hex":"e83dfeb501","size":5,"soft_fail":false,"target_va":5402980128,"va":5374275806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":400,"record_offset":72300,"parent_record_offset":69996,"ancestor_record_offset":69996,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":400,"record_offset":72892,"parent_record_offset":69996,"ancestor_record_offset":69996,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":73244,"parent_record_offset":69996,"ancestor_record_offset":69996,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":73384,"parent_record_offset":69996,"ancestor_record_offset":69996,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":400,"record_offset":70188,"parent_record_offset":69996,"ancestor_record_offset":69996,"inlinee":7937,"source_line":55,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":400,"record_offset":71460,"parent_record_offset":69996,"ancestor_record_offset":69996,"inlinee":11369,"source_line":63,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":70580,"parent_record_offset":70556,"ancestor_record_offset":69996,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":70604,"parent_record_offset":70580,"ancestor_record_offset":69996,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":71408,"parent_record_offset":71384,"ancestor_record_offset":69996,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":70556,"parent_record_offset":70528,"ancestor_record_offset":69996,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":71384,"parent_record_offset":71360,"ancestor_record_offset":69996,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":73140,"parent_record_offset":73112,"ancestor_record_offset":69996,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":73320,"parent_record_offset":73288,"ancestor_record_offset":69996,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":70528,"parent_record_offset":70500,"ancestor_record_offset":69996,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":70500,"parent_record_offset":70472,"ancestor_record_offset":69996,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":70672,"parent_record_offset":70500,"ancestor_record_offset":69996,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":71360,"parent_record_offset":71336,"ancestor_record_offset":69996,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":70436,"parent_record_offset":70408,"ancestor_record_offset":69996,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":70408,"parent_record_offset":70380,"ancestor_record_offset":69996,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":70352,"parent_record_offset":70324,"ancestor_record_offset":69996,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":70380,"parent_record_offset":70352,"ancestor_record_offset":69996,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":70324,"parent_record_offset":70296,"ancestor_record_offset":69996,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":400,"record_offset":70296,"parent_record_offset":70268,"ancestor_record_offset":69996,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":400,"record_offset":72944,"parent_record_offset":72892,"ancestor_record_offset":69996,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":400,"record_offset":70268,"parent_record_offset":70240,"ancestor_record_offset":69996,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":73288,"parent_record_offset":73244,"ancestor_record_offset":69996,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":70472,"parent_record_offset":70436,"ancestor_record_offset":69996,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":400,"record_offset":70240,"parent_record_offset":70188,"ancestor_record_offset":69996,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":400,"record_offset":72972,"parent_record_offset":72892,"ancestor_record_offset":69996,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":400,"record_offset":70732,"parent_record_offset":70708,"ancestor_record_offset":69996,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":400,"record_offset":71532,"parent_record_offset":71504,"ancestor_record_offset":69996,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":400,"record_offset":71504,"parent_record_offset":71460,"ancestor_record_offset":69996,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":400,"record_offset":70708,"parent_record_offset":70436,"ancestor_record_offset":69996,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":71288,"parent_record_offset":70188,"ancestor_record_offset":69996,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":70628,"parent_record_offset":70604,"ancestor_record_offset":69996,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":73352,"parent_record_offset":73244,"ancestor_record_offset":69996,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":400,"record_offset":73000,"parent_record_offset":72892,"ancestor_record_offset":69996,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::dof::impl$15::as_compute_pipeline
// 记录形式: compiled_procedure; instances=1
// 源码行: 120
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":400,"record_offset":73556,"rva":5567008,"va":5374276128}
// 编译实例 RVAs: 0x54f220
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b8730
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2715439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374276128,"raw_hex":"e8d190d6ff","size":5,"soft_fail":false,"target_va":5371561776,"va":5374277210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140387ca0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1865983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374276128,"raw_hex":"e80187e3ff","size":5,"soft_fail":false,"target_va":5372411040,"va":5374277018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1526492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374276128,"raw_hex":"e824b5e8ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374276183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1526699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374276128,"raw_hex":"e855b4e8ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374276406,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t28701320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374276128,"raw_hex":"e888f2b501","size":5,"soft_fail":false,"target_va":5402978144,"va":5374276819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2969040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374276128,"raw_hex":"e830b2d2ff","size":5,"soft_fail":false,"target_va":5371307664,"va":5374276699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$::fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3333686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374276128,"raw_hex":"e836de3200","size":5,"soft_fail":false,"target_va":5377610320,"va":5374276629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28703206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374276128,"raw_hex":"e8e6f9b501","size":5,"soft_fail":false,"target_va":5402980128,"va":5374276917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":400,"record_offset":75860,"parent_record_offset":73556,"ancestor_record_offset":73556,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":400,"record_offset":76452,"parent_record_offset":73556,"ancestor_record_offset":73556,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":76804,"parent_record_offset":73556,"ancestor_record_offset":73556,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":76944,"parent_record_offset":73556,"ancestor_record_offset":73556,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":400,"record_offset":73748,"parent_record_offset":73556,"ancestor_record_offset":73556,"inlinee":7937,"source_line":55,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":400,"record_offset":75020,"parent_record_offset":73556,"ancestor_record_offset":73556,"inlinee":11369,"source_line":63,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":74140,"parent_record_offset":74116,"ancestor_record_offset":73556,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":74164,"parent_record_offset":74140,"ancestor_record_offset":73556,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":74968,"parent_record_offset":74944,"ancestor_record_offset":73556,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":74116,"parent_record_offset":74088,"ancestor_record_offset":73556,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":74944,"parent_record_offset":74920,"ancestor_record_offset":73556,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":76700,"parent_record_offset":76672,"ancestor_record_offset":73556,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":76880,"parent_record_offset":76848,"ancestor_record_offset":73556,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":74088,"parent_record_offset":74060,"ancestor_record_offset":73556,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":74060,"parent_record_offset":74032,"ancestor_record_offset":73556,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":74232,"parent_record_offset":74060,"ancestor_record_offset":73556,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":74920,"parent_record_offset":74896,"ancestor_record_offset":73556,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":73996,"parent_record_offset":73968,"ancestor_record_offset":73556,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":73968,"parent_record_offset":73940,"ancestor_record_offset":73556,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":73912,"parent_record_offset":73884,"ancestor_record_offset":73556,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":73940,"parent_record_offset":73912,"ancestor_record_offset":73556,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":73884,"parent_record_offset":73856,"ancestor_record_offset":73556,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":400,"record_offset":73856,"parent_record_offset":73828,"ancestor_record_offset":73556,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":400,"record_offset":76504,"parent_record_offset":76452,"ancestor_record_offset":73556,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":400,"record_offset":73828,"parent_record_offset":73800,"ancestor_record_offset":73556,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":76848,"parent_record_offset":76804,"ancestor_record_offset":73556,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":74032,"parent_record_offset":73996,"ancestor_record_offset":73556,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":400,"record_offset":73800,"parent_record_offset":73748,"ancestor_record_offset":73556,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":400,"record_offset":76532,"parent_record_offset":76452,"ancestor_record_offset":73556,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":400,"record_offset":74292,"parent_record_offset":74268,"ancestor_record_offset":73556,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":400,"record_offset":75092,"parent_record_offset":75064,"ancestor_record_offset":73556,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":400,"record_offset":75064,"parent_record_offset":75020,"ancestor_record_offset":73556,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":400,"record_offset":74268,"parent_record_offset":73996,"ancestor_record_offset":73556,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":74848,"parent_record_offset":73748,"ancestor_record_offset":73556,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":74188,"parent_record_offset":74164,"ancestor_record_offset":73556,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":76912,"parent_record_offset":76804,"ancestor_record_offset":73556,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":400,"record_offset":76560,"parent_record_offset":76452,"ancestor_record_offset":73556,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::dof::impl$19::as_compute_pipeline
// 记录形式: compiled_procedure; instances=1
// 源码行: 120
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":400,"record_offset":77116,"rva":5568112,"va":5374277232}
// 编译实例 RVAs: 0x54f670
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b8730
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2716559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374277232,"raw_hex":"e8718cd6ff","size":5,"soft_fail":false,"target_va":5371561776,"va":5374278330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140387ca0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1867103","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374277232,"raw_hex":"e8a182e3ff","size":5,"soft_fail":false,"target_va":5372411040,"va":5374278138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1527596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374277232,"raw_hex":"e8d4b0e8ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374277287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1527810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374277232,"raw_hex":"e8feafe8ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374277517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t28700209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374277232,"raw_hex":"e831eeb501","size":5,"soft_fail":false,"target_va":5402978144,"va":5374277930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2970151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374277232,"raw_hex":"e8d9add2ff","size":5,"soft_fail":false,"target_va":5371307664,"va":5374277810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$::fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3332575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374277232,"raw_hex":"e8dfd93200","size":5,"soft_fail":false,"target_va":5377610320,"va":5374277740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28702095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374277232,"raw_hex":"e88ff5b501","size":5,"soft_fail":false,"target_va":5402980128,"va":5374278028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":400,"record_offset":79420,"parent_record_offset":77116,"ancestor_record_offset":77116,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":400,"record_offset":80012,"parent_record_offset":77116,"ancestor_record_offset":77116,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":80364,"parent_record_offset":77116,"ancestor_record_offset":77116,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":80504,"parent_record_offset":77116,"ancestor_record_offset":77116,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":400,"record_offset":77308,"parent_record_offset":77116,"ancestor_record_offset":77116,"inlinee":7937,"source_line":55,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":400,"record_offset":78580,"parent_record_offset":77116,"ancestor_record_offset":77116,"inlinee":11369,"source_line":63,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":77700,"parent_record_offset":77676,"ancestor_record_offset":77116,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":77724,"parent_record_offset":77700,"ancestor_record_offset":77116,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":78528,"parent_record_offset":78504,"ancestor_record_offset":77116,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":77676,"parent_record_offset":77648,"ancestor_record_offset":77116,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":78504,"parent_record_offset":78480,"ancestor_record_offset":77116,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":80260,"parent_record_offset":80232,"ancestor_record_offset":77116,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":80440,"parent_record_offset":80408,"ancestor_record_offset":77116,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":77648,"parent_record_offset":77620,"ancestor_record_offset":77116,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":77620,"parent_record_offset":77592,"ancestor_record_offset":77116,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":77792,"parent_record_offset":77620,"ancestor_record_offset":77116,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":78480,"parent_record_offset":78456,"ancestor_record_offset":77116,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":77556,"parent_record_offset":77528,"ancestor_record_offset":77116,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":77528,"parent_record_offset":77500,"ancestor_record_offset":77116,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":77472,"parent_record_offset":77444,"ancestor_record_offset":77116,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":77500,"parent_record_offset":77472,"ancestor_record_offset":77116,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":77444,"parent_record_offset":77416,"ancestor_record_offset":77116,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":400,"record_offset":77416,"parent_record_offset":77388,"ancestor_record_offset":77116,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":400,"record_offset":80064,"parent_record_offset":80012,"ancestor_record_offset":77116,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":400,"record_offset":77388,"parent_record_offset":77360,"ancestor_record_offset":77116,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":80408,"parent_record_offset":80364,"ancestor_record_offset":77116,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":77592,"parent_record_offset":77556,"ancestor_record_offset":77116,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":400,"record_offset":77360,"parent_record_offset":77308,"ancestor_record_offset":77116,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":400,"record_offset":80092,"parent_record_offset":80012,"ancestor_record_offset":77116,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":400,"record_offset":77852,"parent_record_offset":77828,"ancestor_record_offset":77116,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":400,"record_offset":78652,"parent_record_offset":78624,"ancestor_record_offset":77116,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":400,"record_offset":78624,"parent_record_offset":78580,"ancestor_record_offset":77116,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":400,"record_offset":77828,"parent_record_offset":77556,"ancestor_record_offset":77116,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":78408,"parent_record_offset":77308,"ancestor_record_offset":77116,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":77748,"parent_record_offset":77724,"ancestor_record_offset":77116,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":80472,"parent_record_offset":80364,"ancestor_record_offset":77116,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":400,"record_offset":80120,"parent_record_offset":80012,"ancestor_record_offset":77116,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::dof::impl$3::as_compute_pipeline
// 记录形式: compiled_procedure; instances=1
// 源码行: 120
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":400,"record_offset":62496,"rva":5563600,"va":5374272720}
// 编译实例 RVAs: 0x54e4d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b8730
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2712031","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374272720,"raw_hex":"e8219ed6ff","size":5,"soft_fail":false,"target_va":5371561776,"va":5374273802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140387ca0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1862575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374272720,"raw_hex":"e85194e3ff","size":5,"soft_fail":false,"target_va":5372411040,"va":5374273610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1523084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374272720,"raw_hex":"e874c2e8ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374272775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1523288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374272720,"raw_hex":"e8a8c1e8ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374272995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t28704731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374272720,"raw_hex":"e8dbffb501","size":5,"soft_fail":false,"target_va":5402978144,"va":5374273408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2965629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374272720,"raw_hex":"e883bfd2ff","size":5,"soft_fail":false,"target_va":5371307664,"va":5374273288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$::fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3337097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374272720,"raw_hex":"e889eb3200","size":5,"soft_fail":false,"target_va":5377610320,"va":5374273218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28706617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374272720,"raw_hex":"e83907b601","size":5,"soft_fail":false,"target_va":5402980128,"va":5374273506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":400,"record_offset":64796,"parent_record_offset":62496,"ancestor_record_offset":62496,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":400,"record_offset":65388,"parent_record_offset":62496,"ancestor_record_offset":62496,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":65740,"parent_record_offset":62496,"ancestor_record_offset":62496,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":65880,"parent_record_offset":62496,"ancestor_record_offset":62496,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":400,"record_offset":62684,"parent_record_offset":62496,"ancestor_record_offset":62496,"inlinee":7937,"source_line":55,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":400,"record_offset":63956,"parent_record_offset":62496,"ancestor_record_offset":62496,"inlinee":11369,"source_line":63,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":63076,"parent_record_offset":63052,"ancestor_record_offset":62496,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":63100,"parent_record_offset":63076,"ancestor_record_offset":62496,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":63904,"parent_record_offset":63880,"ancestor_record_offset":62496,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":63052,"parent_record_offset":63024,"ancestor_record_offset":62496,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":63880,"parent_record_offset":63856,"ancestor_record_offset":62496,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":65636,"parent_record_offset":65608,"ancestor_record_offset":62496,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":65816,"parent_record_offset":65784,"ancestor_record_offset":62496,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":63024,"parent_record_offset":62996,"ancestor_record_offset":62496,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":62996,"parent_record_offset":62968,"ancestor_record_offset":62496,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":63168,"parent_record_offset":62996,"ancestor_record_offset":62496,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":63856,"parent_record_offset":63832,"ancestor_record_offset":62496,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":62932,"parent_record_offset":62904,"ancestor_record_offset":62496,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":62904,"parent_record_offset":62876,"ancestor_record_offset":62496,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":62848,"parent_record_offset":62820,"ancestor_record_offset":62496,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":62876,"parent_record_offset":62848,"ancestor_record_offset":62496,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":62820,"parent_record_offset":62792,"ancestor_record_offset":62496,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":400,"record_offset":62792,"parent_record_offset":62764,"ancestor_record_offset":62496,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":400,"record_offset":65440,"parent_record_offset":65388,"ancestor_record_offset":62496,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":400,"record_offset":62764,"parent_record_offset":62736,"ancestor_record_offset":62496,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":65784,"parent_record_offset":65740,"ancestor_record_offset":62496,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":62968,"parent_record_offset":62932,"ancestor_record_offset":62496,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":400,"record_offset":62736,"parent_record_offset":62684,"ancestor_record_offset":62496,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":400,"record_offset":65468,"parent_record_offset":65388,"ancestor_record_offset":62496,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":400,"record_offset":63228,"parent_record_offset":63204,"ancestor_record_offset":62496,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":400,"record_offset":64028,"parent_record_offset":64000,"ancestor_record_offset":62496,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":400,"record_offset":64000,"parent_record_offset":63956,"ancestor_record_offset":62496,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":400,"record_offset":63204,"parent_record_offset":62932,"ancestor_record_offset":62496,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":63784,"parent_record_offset":62684,"ancestor_record_offset":62496,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":63124,"parent_record_offset":63100,"ancestor_record_offset":62496,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":65848,"parent_record_offset":65740,"ancestor_record_offset":62496,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":400,"record_offset":65496,"parent_record_offset":65388,"ancestor_record_offset":62496,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::dof::impl$7::as_compute_pipeline
// 记录形式: compiled_procedure; instances=1
// 源码行: 120
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":400,"record_offset":66244,"rva":5564752,"va":5374273872}
// 编译实例 RVAs: 0x54e950
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b8730
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2713183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374273872,"raw_hex":"e8a199d6ff","size":5,"soft_fail":false,"target_va":5371561776,"va":5374274954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140387ca0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1863727","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374273872,"raw_hex":"e8d18fe3ff","size":5,"soft_fail":false,"target_va":5372411040,"va":5374274762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1524236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374273872,"raw_hex":"e8f4bde8ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374273927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1524440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374273872,"raw_hex":"e828bde8ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374274147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t28703579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374273872,"raw_hex":"e85bfbb501","size":5,"soft_fail":false,"target_va":5402978144,"va":5374274560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2966781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374273872,"raw_hex":"e803bbd2ff","size":5,"soft_fail":false,"target_va":5371307664,"va":5374274440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$::fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3335945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374273872,"raw_hex":"e809e73200","size":5,"soft_fail":false,"target_va":5377610320,"va":5374274370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28705465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374273872,"raw_hex":"e8b902b601","size":5,"soft_fail":false,"target_va":5402980128,"va":5374274658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":400,"record_offset":68544,"parent_record_offset":66244,"ancestor_record_offset":66244,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":400,"record_offset":69136,"parent_record_offset":66244,"ancestor_record_offset":66244,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":69488,"parent_record_offset":66244,"ancestor_record_offset":66244,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":69628,"parent_record_offset":66244,"ancestor_record_offset":66244,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":400,"record_offset":66432,"parent_record_offset":66244,"ancestor_record_offset":66244,"inlinee":7937,"source_line":55,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":400,"record_offset":67704,"parent_record_offset":66244,"ancestor_record_offset":66244,"inlinee":11369,"source_line":63,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":66824,"parent_record_offset":66800,"ancestor_record_offset":66244,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":66848,"parent_record_offset":66824,"ancestor_record_offset":66244,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":67652,"parent_record_offset":67628,"ancestor_record_offset":66244,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":66800,"parent_record_offset":66772,"ancestor_record_offset":66244,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":67628,"parent_record_offset":67604,"ancestor_record_offset":66244,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":69384,"parent_record_offset":69356,"ancestor_record_offset":66244,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":69564,"parent_record_offset":69532,"ancestor_record_offset":66244,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":66772,"parent_record_offset":66744,"ancestor_record_offset":66244,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":66744,"parent_record_offset":66716,"ancestor_record_offset":66244,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":66916,"parent_record_offset":66744,"ancestor_record_offset":66244,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":67604,"parent_record_offset":67580,"ancestor_record_offset":66244,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":66680,"parent_record_offset":66652,"ancestor_record_offset":66244,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":66652,"parent_record_offset":66624,"ancestor_record_offset":66244,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":66596,"parent_record_offset":66568,"ancestor_record_offset":66244,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":66624,"parent_record_offset":66596,"ancestor_record_offset":66244,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":66568,"parent_record_offset":66540,"ancestor_record_offset":66244,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":400,"record_offset":66540,"parent_record_offset":66512,"ancestor_record_offset":66244,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":400,"record_offset":69188,"parent_record_offset":69136,"ancestor_record_offset":66244,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":400,"record_offset":66512,"parent_record_offset":66484,"ancestor_record_offset":66244,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":69532,"parent_record_offset":69488,"ancestor_record_offset":66244,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":66716,"parent_record_offset":66680,"ancestor_record_offset":66244,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":400,"record_offset":66484,"parent_record_offset":66432,"ancestor_record_offset":66244,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":400,"record_offset":69216,"parent_record_offset":69136,"ancestor_record_offset":66244,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":400,"record_offset":66976,"parent_record_offset":66952,"ancestor_record_offset":66244,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":400,"record_offset":67776,"parent_record_offset":67748,"ancestor_record_offset":66244,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":400,"record_offset":67748,"parent_record_offset":67704,"ancestor_record_offset":66244,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":400,"record_offset":66952,"parent_record_offset":66680,"ancestor_record_offset":66244,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":67532,"parent_record_offset":66432,"ancestor_record_offset":66244,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":66872,"parent_record_offset":66848,"ancestor_record_offset":66244,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":69596,"parent_record_offset":69488,"ancestor_record_offset":66244,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":400,"record_offset":69244,"parent_record_offset":69136,"ancestor_record_offset":66244,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::impl$10::as_compute_pipeline
// 记录形式: compiled_procedure; instances=1
// 源码行: 120
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":451,"record_offset":51956,"rva":6377856,"va":5375086976}
// 编译实例 RVAs: 0x615180
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b8730
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3526303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375086976,"raw_hex":"e86131caff","size":5,"soft_fail":false,"target_va":5371561776,"va":5375088074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140387ca0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2676847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375086976,"raw_hex":"e89127d7ff","size":5,"soft_fail":false,"target_va":5372411040,"va":5375087882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2337340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375086976,"raw_hex":"e8c455dcff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375087031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2337558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375086976,"raw_hex":"e8ea54dcff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375087265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t27890461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375086976,"raw_hex":"e81d93a901","size":5,"soft_fail":false,"target_va":5402978144,"va":5375087678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-3779899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375086976,"raw_hex":"e8c552c6ff","size":5,"soft_fail":false,"target_va":5371307664,"va":5375087558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$::fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2522827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375086976,"raw_hex":"e8cb7e2600","size":5,"soft_fail":false,"target_va":5377610320,"va":5375087488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27892347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375086976,"raw_hex":"e87b9aa901","size":5,"soft_fail":false,"target_va":5402980128,"va":5375087776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":451,"record_offset":54280,"parent_record_offset":51956,"ancestor_record_offset":51956,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":451,"record_offset":54872,"parent_record_offset":51956,"ancestor_record_offset":51956,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":451,"record_offset":55224,"parent_record_offset":51956,"ancestor_record_offset":51956,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":451,"record_offset":55364,"parent_record_offset":51956,"ancestor_record_offset":51956,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":451,"record_offset":52168,"parent_record_offset":51956,"ancestor_record_offset":51956,"inlinee":7937,"source_line":55,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":451,"record_offset":53440,"parent_record_offset":51956,"ancestor_record_offset":51956,"inlinee":11369,"source_line":63,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":451,"record_offset":52560,"parent_record_offset":52536,"ancestor_record_offset":51956,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":451,"record_offset":52584,"parent_record_offset":52560,"ancestor_record_offset":51956,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":451,"record_offset":53388,"parent_record_offset":53364,"ancestor_record_offset":51956,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":451,"record_offset":52536,"parent_record_offset":52508,"ancestor_record_offset":51956,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":451,"record_offset":53364,"parent_record_offset":53340,"ancestor_record_offset":51956,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":451,"record_offset":55120,"parent_record_offset":55092,"ancestor_record_offset":51956,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":451,"record_offset":55300,"parent_record_offset":55268,"ancestor_record_offset":51956,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":451,"record_offset":52508,"parent_record_offset":52480,"ancestor_record_offset":51956,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":451,"record_offset":52480,"parent_record_offset":52452,"ancestor_record_offset":51956,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":451,"record_offset":52652,"parent_record_offset":52480,"ancestor_record_offset":51956,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":451,"record_offset":53340,"parent_record_offset":53316,"ancestor_record_offset":51956,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":451,"record_offset":52416,"parent_record_offset":52388,"ancestor_record_offset":51956,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":451,"record_offset":52388,"parent_record_offset":52360,"ancestor_record_offset":51956,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":451,"record_offset":52332,"parent_record_offset":52304,"ancestor_record_offset":51956,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":451,"record_offset":52360,"parent_record_offset":52332,"ancestor_record_offset":51956,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":451,"record_offset":52304,"parent_record_offset":52276,"ancestor_record_offset":51956,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":451,"record_offset":52276,"parent_record_offset":52248,"ancestor_record_offset":51956,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":451,"record_offset":54924,"parent_record_offset":54872,"ancestor_record_offset":51956,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":451,"record_offset":52248,"parent_record_offset":52220,"ancestor_record_offset":51956,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":451,"record_offset":55268,"parent_record_offset":55224,"ancestor_record_offset":51956,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":451,"record_offset":52452,"parent_record_offset":52416,"ancestor_record_offset":51956,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":451,"record_offset":52220,"parent_record_offset":52168,"ancestor_record_offset":51956,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":451,"record_offset":54952,"parent_record_offset":54872,"ancestor_record_offset":51956,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":451,"record_offset":52712,"parent_record_offset":52688,"ancestor_record_offset":51956,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":451,"record_offset":53512,"parent_record_offset":53484,"ancestor_record_offset":51956,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":451,"record_offset":53484,"parent_record_offset":53440,"ancestor_record_offset":51956,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":451,"record_offset":52688,"parent_record_offset":52416,"ancestor_record_offset":51956,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":451,"record_offset":53268,"parent_record_offset":52168,"ancestor_record_offset":51956,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":451,"record_offset":52608,"parent_record_offset":52584,"ancestor_record_offset":51956,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":451,"record_offset":55332,"parent_record_offset":55224,"ancestor_record_offset":51956,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":451,"record_offset":54980,"parent_record_offset":54872,"ancestor_record_offset":51956,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::impl$14::as_compute_pipeline
// 记录形式: compiled_procedure; instances=1
// 源码行: 120
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":451,"record_offset":55536,"rva":6378976,"va":5375088096}
// 编译实例 RVAs: 0x6155e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b8730
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3527407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375088096,"raw_hex":"e8112dcaff","size":5,"soft_fail":false,"target_va":5371561776,"va":5375089178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140387ca0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2677951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375088096,"raw_hex":"e84123d7ff","size":5,"soft_fail":false,"target_va":5372411040,"va":5375088986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2338460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375088096,"raw_hex":"e86451dcff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375088151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2338670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375088096,"raw_hex":"e89250dcff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375088377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t27889349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375088096,"raw_hex":"e8c58ea901","size":5,"soft_fail":false,"target_va":5402978144,"va":5375088790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-3781011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375088096,"raw_hex":"e86d4ec6ff","size":5,"soft_fail":false,"target_va":5371307664,"va":5375088670,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$::fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2521715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375088096,"raw_hex":"e8737a2600","size":5,"soft_fail":false,"target_va":5377610320,"va":5375088600,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27891235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375088096,"raw_hex":"e82396a901","size":5,"soft_fail":false,"target_va":5402980128,"va":5375088888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":451,"record_offset":57860,"parent_record_offset":55536,"ancestor_record_offset":55536,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":451,"record_offset":58452,"parent_record_offset":55536,"ancestor_record_offset":55536,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":451,"record_offset":58804,"parent_record_offset":55536,"ancestor_record_offset":55536,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":451,"record_offset":58944,"parent_record_offset":55536,"ancestor_record_offset":55536,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":451,"record_offset":55748,"parent_record_offset":55536,"ancestor_record_offset":55536,"inlinee":7937,"source_line":55,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":451,"record_offset":57020,"parent_record_offset":55536,"ancestor_record_offset":55536,"inlinee":11369,"source_line":63,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":451,"record_offset":56140,"parent_record_offset":56116,"ancestor_record_offset":55536,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":451,"record_offset":56164,"parent_record_offset":56140,"ancestor_record_offset":55536,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":451,"record_offset":56968,"parent_record_offset":56944,"ancestor_record_offset":55536,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":451,"record_offset":56116,"parent_record_offset":56088,"ancestor_record_offset":55536,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":451,"record_offset":56944,"parent_record_offset":56920,"ancestor_record_offset":55536,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":451,"record_offset":58700,"parent_record_offset":58672,"ancestor_record_offset":55536,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":451,"record_offset":58880,"parent_record_offset":58848,"ancestor_record_offset":55536,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":451,"record_offset":56088,"parent_record_offset":56060,"ancestor_record_offset":55536,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":451,"record_offset":56060,"parent_record_offset":56032,"ancestor_record_offset":55536,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":451,"record_offset":56232,"parent_record_offset":56060,"ancestor_record_offset":55536,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":451,"record_offset":56920,"parent_record_offset":56896,"ancestor_record_offset":55536,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":451,"record_offset":55996,"parent_record_offset":55968,"ancestor_record_offset":55536,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":451,"record_offset":55968,"parent_record_offset":55940,"ancestor_record_offset":55536,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":451,"record_offset":55912,"parent_record_offset":55884,"ancestor_record_offset":55536,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":451,"record_offset":55940,"parent_record_offset":55912,"ancestor_record_offset":55536,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":451,"record_offset":55884,"parent_record_offset":55856,"ancestor_record_offset":55536,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":451,"record_offset":55856,"parent_record_offset":55828,"ancestor_record_offset":55536,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":451,"record_offset":58504,"parent_record_offset":58452,"ancestor_record_offset":55536,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":451,"record_offset":55828,"parent_record_offset":55800,"ancestor_record_offset":55536,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":451,"record_offset":58848,"parent_record_offset":58804,"ancestor_record_offset":55536,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":451,"record_offset":56032,"parent_record_offset":55996,"ancestor_record_offset":55536,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":451,"record_offset":55800,"parent_record_offset":55748,"ancestor_record_offset":55536,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":451,"record_offset":58532,"parent_record_offset":58452,"ancestor_record_offset":55536,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":451,"record_offset":56292,"parent_record_offset":56268,"ancestor_record_offset":55536,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":451,"record_offset":57092,"parent_record_offset":57064,"ancestor_record_offset":55536,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":451,"record_offset":57064,"parent_record_offset":57020,"ancestor_record_offset":55536,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":451,"record_offset":56268,"parent_record_offset":55996,"ancestor_record_offset":55536,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":451,"record_offset":56848,"parent_record_offset":55748,"ancestor_record_offset":55536,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":451,"record_offset":56188,"parent_record_offset":56164,"ancestor_record_offset":55536,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":451,"record_offset":58912,"parent_record_offset":58804,"ancestor_record_offset":55536,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":451,"record_offset":58560,"parent_record_offset":58452,"ancestor_record_offset":55536,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::impl$18::as_compute_pipeline
// 记录形式: compiled_procedure; instances=1
// 源码行: 120
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":451,"record_offset":59116,"rva":6380080,"va":5375089200}
// 编译实例 RVAs: 0x615a30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b8730
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3528527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375089200,"raw_hex":"e8b128caff","size":5,"soft_fail":false,"target_va":5371561776,"va":5375090298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140387ca0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2679071","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375089200,"raw_hex":"e8e11ed7ff","size":5,"soft_fail":false,"target_va":5372411040,"va":5375090106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2339564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375089200,"raw_hex":"e8144ddcff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375089255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2339779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375089200,"raw_hex":"e83d4cdcff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375089486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t27888240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375089200,"raw_hex":"e8708aa901","size":5,"soft_fail":false,"target_va":5402978144,"va":5375089899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-3782120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375089200,"raw_hex":"e8184ac6ff","size":5,"soft_fail":false,"target_va":5371307664,"va":5375089779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$::fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2520606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375089200,"raw_hex":"e81e762600","size":5,"soft_fail":false,"target_va":5377610320,"va":5375089709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27890126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375089200,"raw_hex":"e8ce91a901","size":5,"soft_fail":false,"target_va":5402980128,"va":5375089997,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":451,"record_offset":61440,"parent_record_offset":59116,"ancestor_record_offset":59116,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":451,"record_offset":62032,"parent_record_offset":59116,"ancestor_record_offset":59116,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":451,"record_offset":62384,"parent_record_offset":59116,"ancestor_record_offset":59116,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":451,"record_offset":62524,"parent_record_offset":59116,"ancestor_record_offset":59116,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":451,"record_offset":59328,"parent_record_offset":59116,"ancestor_record_offset":59116,"inlinee":7937,"source_line":55,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":451,"record_offset":60600,"parent_record_offset":59116,"ancestor_record_offset":59116,"inlinee":11369,"source_line":63,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":451,"record_offset":59720,"parent_record_offset":59696,"ancestor_record_offset":59116,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":451,"record_offset":59744,"parent_record_offset":59720,"ancestor_record_offset":59116,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":451,"record_offset":60548,"parent_record_offset":60524,"ancestor_record_offset":59116,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":451,"record_offset":59696,"parent_record_offset":59668,"ancestor_record_offset":59116,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":451,"record_offset":60524,"parent_record_offset":60500,"ancestor_record_offset":59116,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":451,"record_offset":62280,"parent_record_offset":62252,"ancestor_record_offset":59116,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":451,"record_offset":62460,"parent_record_offset":62428,"ancestor_record_offset":59116,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":451,"record_offset":59668,"parent_record_offset":59640,"ancestor_record_offset":59116,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":451,"record_offset":59640,"parent_record_offset":59612,"ancestor_record_offset":59116,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":451,"record_offset":59812,"parent_record_offset":59640,"ancestor_record_offset":59116,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":451,"record_offset":60500,"parent_record_offset":60476,"ancestor_record_offset":59116,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":451,"record_offset":59576,"parent_record_offset":59548,"ancestor_record_offset":59116,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":451,"record_offset":59548,"parent_record_offset":59520,"ancestor_record_offset":59116,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":451,"record_offset":59492,"parent_record_offset":59464,"ancestor_record_offset":59116,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":451,"record_offset":59520,"parent_record_offset":59492,"ancestor_record_offset":59116,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":451,"record_offset":59464,"parent_record_offset":59436,"ancestor_record_offset":59116,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":451,"record_offset":59436,"parent_record_offset":59408,"ancestor_record_offset":59116,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":451,"record_offset":62084,"parent_record_offset":62032,"ancestor_record_offset":59116,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":451,"record_offset":59408,"parent_record_offset":59380,"ancestor_record_offset":59116,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":451,"record_offset":62428,"parent_record_offset":62384,"ancestor_record_offset":59116,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":451,"record_offset":59612,"parent_record_offset":59576,"ancestor_record_offset":59116,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":451,"record_offset":59380,"parent_record_offset":59328,"ancestor_record_offset":59116,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":451,"record_offset":62112,"parent_record_offset":62032,"ancestor_record_offset":59116,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":451,"record_offset":59872,"parent_record_offset":59848,"ancestor_record_offset":59116,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":451,"record_offset":60672,"parent_record_offset":60644,"ancestor_record_offset":59116,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":451,"record_offset":60644,"parent_record_offset":60600,"ancestor_record_offset":59116,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":451,"record_offset":59848,"parent_record_offset":59576,"ancestor_record_offset":59116,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":451,"record_offset":60428,"parent_record_offset":59328,"ancestor_record_offset":59116,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":451,"record_offset":59768,"parent_record_offset":59744,"ancestor_record_offset":59116,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":451,"record_offset":62492,"parent_record_offset":62384,"ancestor_record_offset":59116,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":451,"record_offset":62140,"parent_record_offset":62032,"ancestor_record_offset":59116,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::impl$2::as_compute_pipeline
// 记录形式: compiled_procedure; instances=1
// 源码行: 120
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":451,"record_offset":44804,"rva":6375632,"va":5375084752}
// 编译实例 RVAs: 0x6148d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b8730
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3524063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375084752,"raw_hex":"e8213acaff","size":5,"soft_fail":false,"target_va":5371561776,"va":5375085834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140387ca0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2674607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375084752,"raw_hex":"e85130d7ff","size":5,"soft_fail":false,"target_va":5372411040,"va":5375085642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2335116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375084752,"raw_hex":"e8745edcff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375084807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2335320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375084752,"raw_hex":"e8a85ddcff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375085027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t27892699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375084752,"raw_hex":"e8db9ba901","size":5,"soft_fail":false,"target_va":5402978144,"va":5375085440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-3777661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375084752,"raw_hex":"e8835bc6ff","size":5,"soft_fail":false,"target_va":5371307664,"va":5375085320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$::fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2525065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375084752,"raw_hex":"e889872600","size":5,"soft_fail":false,"target_va":5377610320,"va":5375085250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27894585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375084752,"raw_hex":"e839a3a901","size":5,"soft_fail":false,"target_va":5402980128,"va":5375085538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":451,"record_offset":47124,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":451,"record_offset":47716,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":451,"record_offset":48068,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":451,"record_offset":48208,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":451,"record_offset":45012,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":7937,"source_line":55,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":451,"record_offset":46284,"parent_record_offset":44804,"ancestor_record_offset":44804,"inlinee":11369,"source_line":63,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":451,"record_offset":45404,"parent_record_offset":45380,"ancestor_record_offset":44804,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":451,"record_offset":45428,"parent_record_offset":45404,"ancestor_record_offset":44804,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":451,"record_offset":46232,"parent_record_offset":46208,"ancestor_record_offset":44804,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":451,"record_offset":45380,"parent_record_offset":45352,"ancestor_record_offset":44804,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":451,"record_offset":46208,"parent_record_offset":46184,"ancestor_record_offset":44804,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":451,"record_offset":47964,"parent_record_offset":47936,"ancestor_record_offset":44804,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":451,"record_offset":48144,"parent_record_offset":48112,"ancestor_record_offset":44804,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":451,"record_offset":45352,"parent_record_offset":45324,"ancestor_record_offset":44804,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":451,"record_offset":45324,"parent_record_offset":45296,"ancestor_record_offset":44804,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":451,"record_offset":45496,"parent_record_offset":45324,"ancestor_record_offset":44804,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":451,"record_offset":46184,"parent_record_offset":46160,"ancestor_record_offset":44804,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":451,"record_offset":45260,"parent_record_offset":45232,"ancestor_record_offset":44804,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":451,"record_offset":45232,"parent_record_offset":45204,"ancestor_record_offset":44804,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":451,"record_offset":45176,"parent_record_offset":45148,"ancestor_record_offset":44804,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":451,"record_offset":45204,"parent_record_offset":45176,"ancestor_record_offset":44804,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":451,"record_offset":45148,"parent_record_offset":45120,"ancestor_record_offset":44804,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":451,"record_offset":45120,"parent_record_offset":45092,"ancestor_record_offset":44804,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":451,"record_offset":47768,"parent_record_offset":47716,"ancestor_record_offset":44804,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":451,"record_offset":45092,"parent_record_offset":45064,"ancestor_record_offset":44804,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":451,"record_offset":48112,"parent_record_offset":48068,"ancestor_record_offset":44804,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":451,"record_offset":45296,"parent_record_offset":45260,"ancestor_record_offset":44804,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":451,"record_offset":45064,"parent_record_offset":45012,"ancestor_record_offset":44804,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":451,"record_offset":47796,"parent_record_offset":47716,"ancestor_record_offset":44804,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":451,"record_offset":45556,"parent_record_offset":45532,"ancestor_record_offset":44804,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":451,"record_offset":46356,"parent_record_offset":46328,"ancestor_record_offset":44804,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":451,"record_offset":46328,"parent_record_offset":46284,"ancestor_record_offset":44804,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":451,"record_offset":45532,"parent_record_offset":45260,"ancestor_record_offset":44804,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":451,"record_offset":46112,"parent_record_offset":45012,"ancestor_record_offset":44804,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":451,"record_offset":45452,"parent_record_offset":45428,"ancestor_record_offset":44804,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":451,"record_offset":48176,"parent_record_offset":48068,"ancestor_record_offset":44804,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":451,"record_offset":47824,"parent_record_offset":47716,"ancestor_record_offset":44804,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::impl$6::as_compute_pipeline
// 记录形式: compiled_procedure; instances=1
// 源码行: 120
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":451,"record_offset":48380,"rva":6376736,"va":5375085856}
// 编译实例 RVAs: 0x614d20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b8730
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3525183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375085856,"raw_hex":"e8c135caff","size":5,"soft_fail":false,"target_va":5371561776,"va":5375086954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140387ca0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2675727","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375085856,"raw_hex":"e8f12bd7ff","size":5,"soft_fail":false,"target_va":5372411040,"va":5375086762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2336220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375085856,"raw_hex":"e8245adcff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375085911,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2336438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375085856,"raw_hex":"e84a59dcff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375086145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t27891581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375085856,"raw_hex":"e87d97a901","size":5,"soft_fail":false,"target_va":5402978144,"va":5375086558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-3778779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375085856,"raw_hex":"e82557c6ff","size":5,"soft_fail":false,"target_va":5371307664,"va":5375086438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$::fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2523947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375085856,"raw_hex":"e82b832600","size":5,"soft_fail":false,"target_va":5377610320,"va":5375086368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27893467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375085856,"raw_hex":"e8db9ea901","size":5,"soft_fail":false,"target_va":5402980128,"va":5375086656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":451,"record_offset":50700,"parent_record_offset":48380,"ancestor_record_offset":48380,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":451,"record_offset":51292,"parent_record_offset":48380,"ancestor_record_offset":48380,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":451,"record_offset":51644,"parent_record_offset":48380,"ancestor_record_offset":48380,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":451,"record_offset":51784,"parent_record_offset":48380,"ancestor_record_offset":48380,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":451,"record_offset":48588,"parent_record_offset":48380,"ancestor_record_offset":48380,"inlinee":7937,"source_line":55,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":451,"record_offset":49860,"parent_record_offset":48380,"ancestor_record_offset":48380,"inlinee":11369,"source_line":63,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":451,"record_offset":48980,"parent_record_offset":48956,"ancestor_record_offset":48380,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":451,"record_offset":49004,"parent_record_offset":48980,"ancestor_record_offset":48380,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":451,"record_offset":49808,"parent_record_offset":49784,"ancestor_record_offset":48380,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":451,"record_offset":48956,"parent_record_offset":48928,"ancestor_record_offset":48380,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":451,"record_offset":49784,"parent_record_offset":49760,"ancestor_record_offset":48380,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":451,"record_offset":51540,"parent_record_offset":51512,"ancestor_record_offset":48380,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":451,"record_offset":51720,"parent_record_offset":51688,"ancestor_record_offset":48380,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":451,"record_offset":48928,"parent_record_offset":48900,"ancestor_record_offset":48380,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":451,"record_offset":48900,"parent_record_offset":48872,"ancestor_record_offset":48380,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":451,"record_offset":49072,"parent_record_offset":48900,"ancestor_record_offset":48380,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":451,"record_offset":49760,"parent_record_offset":49736,"ancestor_record_offset":48380,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":451,"record_offset":48836,"parent_record_offset":48808,"ancestor_record_offset":48380,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":451,"record_offset":48808,"parent_record_offset":48780,"ancestor_record_offset":48380,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":451,"record_offset":48752,"parent_record_offset":48724,"ancestor_record_offset":48380,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":451,"record_offset":48780,"parent_record_offset":48752,"ancestor_record_offset":48380,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":451,"record_offset":48724,"parent_record_offset":48696,"ancestor_record_offset":48380,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":451,"record_offset":48696,"parent_record_offset":48668,"ancestor_record_offset":48380,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":451,"record_offset":51344,"parent_record_offset":51292,"ancestor_record_offset":48380,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":451,"record_offset":48668,"parent_record_offset":48640,"ancestor_record_offset":48380,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":451,"record_offset":51688,"parent_record_offset":51644,"ancestor_record_offset":48380,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":451,"record_offset":48872,"parent_record_offset":48836,"ancestor_record_offset":48380,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":451,"record_offset":48640,"parent_record_offset":48588,"ancestor_record_offset":48380,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":451,"record_offset":51372,"parent_record_offset":51292,"ancestor_record_offset":48380,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":451,"record_offset":49132,"parent_record_offset":49108,"ancestor_record_offset":48380,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":451,"record_offset":49932,"parent_record_offset":49904,"ancestor_record_offset":48380,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":451,"record_offset":49904,"parent_record_offset":49860,"ancestor_record_offset":48380,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":451,"record_offset":49108,"parent_record_offset":48836,"ancestor_record_offset":48380,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":451,"record_offset":49688,"parent_record_offset":48588,"ancestor_record_offset":48380,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":451,"record_offset":49028,"parent_record_offset":49004,"ancestor_record_offset":48380,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":451,"record_offset":51752,"parent_record_offset":51644,"ancestor_record_offset":48380,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":451,"record_offset":51400,"parent_record_offset":51292,"ancestor_record_offset":48380,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00451.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::vermeer::impl$7::as_compute_pipeline
// 记录形式: compiled_procedure; instances=1
// 源码行: 120
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":462,"record_offset":52908,"rva":6548608,"va":5375257728}
// 编译实例 RVAs: 0x63ec80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b8730
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3697055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375257728,"raw_hex":"e86196c7ff","size":5,"soft_fail":false,"target_va":5371561776,"va":5375258826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140387ca0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2847599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375257728,"raw_hex":"e8918cd4ff","size":5,"soft_fail":false,"target_va":5372411040,"va":5375258634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2508092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375257728,"raw_hex":"e8c4bad9ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375257783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2508300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375257728,"raw_hex":"e8f4b9d9ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375258007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t27719712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375257728,"raw_hex":"e820f8a601","size":5,"soft_fail":false,"target_va":5402978144,"va":5375258427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-3950648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375257728,"raw_hex":"e8c8b7c3ff","size":5,"soft_fail":false,"target_va":5371307664,"va":5375258307,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$::fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2352078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375257728,"raw_hex":"e8cee32300","size":5,"soft_fail":false,"target_va":5377610320,"va":5375258237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27721598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375257728,"raw_hex":"e87effa601","size":5,"soft_fail":false,"target_va":5402980128,"va":5375258525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":462,"record_offset":55196,"parent_record_offset":52908,"ancestor_record_offset":52908,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":462,"record_offset":55788,"parent_record_offset":52908,"ancestor_record_offset":52908,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":462,"record_offset":56140,"parent_record_offset":52908,"ancestor_record_offset":52908,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":462,"record_offset":56280,"parent_record_offset":52908,"ancestor_record_offset":52908,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":462,"record_offset":53084,"parent_record_offset":52908,"ancestor_record_offset":52908,"inlinee":7937,"source_line":55,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":462,"record_offset":54356,"parent_record_offset":52908,"ancestor_record_offset":52908,"inlinee":11369,"source_line":63,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":462,"record_offset":53476,"parent_record_offset":53452,"ancestor_record_offset":52908,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":462,"record_offset":53500,"parent_record_offset":53476,"ancestor_record_offset":52908,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":462,"record_offset":54304,"parent_record_offset":54280,"ancestor_record_offset":52908,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":462,"record_offset":53452,"parent_record_offset":53424,"ancestor_record_offset":52908,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":462,"record_offset":54280,"parent_record_offset":54256,"ancestor_record_offset":52908,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":462,"record_offset":56036,"parent_record_offset":56008,"ancestor_record_offset":52908,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":462,"record_offset":56216,"parent_record_offset":56184,"ancestor_record_offset":52908,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":462,"record_offset":53424,"parent_record_offset":53396,"ancestor_record_offset":52908,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":462,"record_offset":53396,"parent_record_offset":53368,"ancestor_record_offset":52908,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":462,"record_offset":53568,"parent_record_offset":53396,"ancestor_record_offset":52908,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":462,"record_offset":54256,"parent_record_offset":54232,"ancestor_record_offset":52908,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":462,"record_offset":53332,"parent_record_offset":53304,"ancestor_record_offset":52908,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":462,"record_offset":53304,"parent_record_offset":53276,"ancestor_record_offset":52908,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":462,"record_offset":53248,"parent_record_offset":53220,"ancestor_record_offset":52908,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":462,"record_offset":53276,"parent_record_offset":53248,"ancestor_record_offset":52908,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":462,"record_offset":53220,"parent_record_offset":53192,"ancestor_record_offset":52908,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":462,"record_offset":53192,"parent_record_offset":53164,"ancestor_record_offset":52908,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":462,"record_offset":55840,"parent_record_offset":55788,"ancestor_record_offset":52908,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":462,"record_offset":53164,"parent_record_offset":53136,"ancestor_record_offset":52908,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":462,"record_offset":56184,"parent_record_offset":56140,"ancestor_record_offset":52908,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":462,"record_offset":53368,"parent_record_offset":53332,"ancestor_record_offset":52908,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":462,"record_offset":53136,"parent_record_offset":53084,"ancestor_record_offset":52908,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":462,"record_offset":55868,"parent_record_offset":55788,"ancestor_record_offset":52908,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":462,"record_offset":53628,"parent_record_offset":53604,"ancestor_record_offset":52908,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":462,"record_offset":54428,"parent_record_offset":54400,"ancestor_record_offset":52908,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":462,"record_offset":54400,"parent_record_offset":54356,"ancestor_record_offset":52908,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":462,"record_offset":53604,"parent_record_offset":53332,"ancestor_record_offset":52908,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":462,"record_offset":54184,"parent_record_offset":53084,"ancestor_record_offset":52908,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":462,"record_offset":53524,"parent_record_offset":53500,"ancestor_record_offset":52908,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":462,"record_offset":56248,"parent_record_offset":56140,"ancestor_record_offset":52908,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":462,"record_offset":55896,"parent_record_offset":55788,"ancestor_record_offset":52908,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00462.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::shader_uri::impl$16::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 197
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":489,"record_offset":56008,"rva":6987344,"va":5375696464}
// 编译实例 RVAs: 0x6a9e50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field1_finish
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1887481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375696464,"raw_hex":"e8f9cc1c00","size":5,"soft_fail":false,"target_va":5377584016,"va":5375696530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::shader_uri::impl$3::hash
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 206
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5695
// 首条位置: {"module":489,"inlinee":22165}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::shader_uri::impl$5::eq
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 213
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x565A
// 首条位置: {"module":390,"inlinee":22106}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash::compute
// 记录形式: inline_instance, inlinee_declaration; instances=39
// 源码行: 220
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1EFF
// 首条位置: {"module":56,"inlinee":7935}
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
// 完整结构及原始类型记录: shader_uri.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::pass_builder::shader_uri::AsComputeShaderSource
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "dyn$<country_core::render::pass_builder::shader_uri::AsComputeShaderSource>", "constructor": "dyn$", "arguments": ["country_core::render::pass_builder::shader_uri::AsComputeShaderSource"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<dyn$<country_core::render::pass_builder::shader_uri::AsComputeShaderSource> >", "constructor": "ref$", "arguments": ["dyn$<country_core::render::pass_builder::shader_uri::AsComputeShaderSource>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<dyn$<country_core::render::pass_builder::shader_uri::AsComputeShaderSource> > >", "constructor": "country_core::render::pass_builder::compute_pass_builder::ComputePass::new", "arguments": ["ref$<dyn$<country_core::render::pass_builder::shader_uri::AsComputeShaderSource> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash,country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram> >", "constructor": "tuple$", "arguments": ["country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash", "country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash,country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash,country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "nohash_hasher::NoHashHasher<country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash>", "constructor": "nohash_hasher::NoHashHasher", "arguments": ["country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash> >", "constructor": "core::hash::BuildHasherDefault", "arguments": ["nohash_hasher::NoHashHasher<country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash,country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash> >,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash", "country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>", "core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::pass_builder::shader_uri::ShaderUri
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: shader_uri.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: shader_uri.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 11; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>", "bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafRng>,bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>", "bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafRng>", "bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafRng>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>
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
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash::compute
// 信息 ID: 693b24078d33a8cf2fdd7703
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 15 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >; module=115, inline_record=604; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<alloc::string::String> >; module=115, inline_record=3684; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::shader_uri::impl$1::source_hash<ref$<str$> >; module=390, inline_record=11152; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::dof::impl$3::source_hash; module=400, inline_record=62464; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::dof::impl$7::source_hash; module=400, inline_record=66212; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::dof::impl$11::source_hash; module=400, inline_record=69964; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketFlowers>; module=1318, inline_record=33388; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketWoodenPlank>; module=1318, inline_record=36588; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::shader_uri::ShaderUri::parse
// 信息 ID: 49c569b84b5c814df415d890
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::shader_uri::impl$1::as_compute_pipeline<ref$<alloc::string::String> > @ 0x7b43f -> 0x594690 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::shader_uri::impl$1::as_compute_pipeline<ref$<str$> > @ 0x26757e -> 0x594690 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::shader_uri::impl$1::as_compute_pipeline<ref$<str$> > @ 0x5131be -> 0x594690 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::shader_uri::impl$16::fmt
// 信息 ID: a47c8af30bf9209091c6cc40
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::shader_uri::impl$1::as_compute_pipeline<ref$<alloc::string::String> >
// 信息 ID: 0d6f55cbfb936627cdf57f51
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["alloc::string::String"]; 字段布局 = unknown。
// [类型关系] country_core::render::pass_builder::shader_uri::impl$1::as_compute_pipeline -> ["ref$<alloc::string::String>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<alloc::string::String> > @ 0x1a787d -> 0x7b410 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::shader_uri::impl$1::as_compute_pipeline<ref$<str$> >
// 信息 ID: 9a1798a6b2b133fa54b60ed3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] country_core::render::pass_builder::shader_uri::impl$1::as_compute_pipeline -> ["ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> > @ 0x1a704a -> 0x267550 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> > @ 0xdce287 -> 0x513190 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::shader_uri::impl$1::source_hash
// 信息 ID: 53c7c31c7d983e7dd206ce29
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >; module=115, inline_record=580; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<alloc::string::String> >; module=115, inline_record=3632; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >; module=1318, inline_record=46036; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::shader_uri::impl$1::source_hash<ref$<str$> >
// 信息 ID: cd04817fc8655cd6004aff81
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] country_core::render::pass_builder::shader_uri::impl$1::source_hash -> ["ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::shader_uri::impl$2::as_compute_pipeline
// 信息 ID: 905230d62a0b76bae166ca66
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<dyn$<country_core::render::pass_builder::shader_uri::AsComputeShaderSource> > >; module=1318, inline_record=44592; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::shader_uri::impl$2::source_hash
// 信息 ID: 448771cd8c4eeaf5d5f71a30
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<dyn$<country_core::render::pass_builder::shader_uri::AsComputeShaderSource> > >; module=1318, inline_record=42932; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::shader_uri::impl$3::hash
// 信息 ID: dda31748edea070958ed04c7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_compute_by_name; module=489, inline_record=41764; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::shader_uri::impl$5::eq
// 信息 ID: 1aaf57a2db43ff8e24dc0f51
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::asset_libraries::shader_library::AssetShaderLibrary::add_compute; module=489, inline_record=40116; 不是独立调用边。
// [PDB 内联宿主] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_compute_by_name; module=489, inline_record=42440; 不是独立调用边。
// [PDB 内联宿主] hashbrown::map::HashMap::insert<country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash,country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash> >,alloc::alloc::Global>; module=1370, inline_record=49488; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::dof::impl$11::as_compute_pipeline
// 信息 ID: 2e93ee231871fe867e5e9206
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::dof::impl$11::source_hash
// 信息 ID: fdec8eb1453239194f1350f6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::dof::impl$15::as_compute_pipeline
// 信息 ID: af24464cec24550f5628c271
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::dof::HalfResDof> @ 0xdd1027 -> 0x54f220 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::dof::impl$15::source_hash
// 信息 ID: 9579770ed9565f8f42b3a2e1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::dof::HalfResDof>; module=1318, inline_record=61900; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::dof::impl$19::as_compute_pipeline
// 信息 ID: 46d870ffbc4742157b76fb45
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::dof::HalfResAllDof> @ 0xdceba7 -> 0x54f670 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::dof::impl$19::source_hash
// 信息 ID: 11c892ddd538633fa9cb3079
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::dof::HalfResAllDof>; module=1318, inline_record=49200; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::dof::impl$3::as_compute_pipeline
// 信息 ID: a6ccabfe513126b3312d9608
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::dof::impl$3::source_hash
// 信息 ID: 47287e14271acbc7adf19a02
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::dof::impl$7::as_compute_pipeline
// 信息 ID: 9060cd99681963f6a190de7d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::dof::impl$7::source_hash
// 信息 ID: 9023d3da712495dc54adf371
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::impl$10::as_compute_pipeline
// 信息 ID: 1a2a8779fd654a070c2bbd60
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketFlowers> @ 0xdcbe27 -> 0x615180 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::impl$10::source_hash
// 信息 ID: 28eef01a8105b812da517582
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketFlowers>; module=1318, inline_record=33364; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::impl$14::as_compute_pipeline
// 信息 ID: 62f54d5e925c7fbd8727214e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketBush> @ 0xdcd067 -> 0x6155e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::impl$14::source_hash
// 信息 ID: 8c94246a869848ba646cd6c2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketBush>; module=1318, inline_record=39756; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::impl$15::hash
// 信息 ID: b14e0e51013f7fc2f0f169eb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::calculate_hash<tuple$<ref$<str$>,ref$<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketBush> > >; module=495, inline_record=3540; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::impl$18::as_compute_pipeline
// 信息 ID: 50279671ed8402b1655f6819
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketWoodenPlank> @ 0xdcc747 -> 0x615a30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::impl$18::source_hash
// 信息 ID: e6ffd5be7726bf2f1541e5ac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketWoodenPlank>; module=1318, inline_record=36564; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::impl$2::as_compute_pipeline
// 信息 ID: 8d552b730727f62c578d2ae0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketMain> @ 0xdd0707 -> 0x6148d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::impl$2::source_hash
// 信息 ID: da3e7bcf955e40436ddd4b33
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketMain>; module=1318, inline_record=58740; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::impl$6::as_compute_pipeline
// 信息 ID: b1f9f94aa6cb752247475f4a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketRoofTile> @ 0xdcfde7 -> 0x614d20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::impl$6::source_hash
// 信息 ID: 17a746c79d5d7cff8dfec9ed
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketRoofTile>; module=1318, inline_record=55548; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::vermeer::impl$7::as_compute_pipeline
// 信息 ID: c7ac348fe73dfa1c181730ff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::vermeer::GenerateStrokes> @ 0xdcf4c7 -> 0x63ec80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::vermeer::impl$7::source_hash
// 信息 ID: ebf35728bf96d5c70a95bc18
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::vermeer::GenerateStrokes>; module=1318, inline_record=52352; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
