// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/render/pass_builder/shader_uri.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: b574401fbd9473af171bceea1d78b9ba046b570b8d9cd5d1f5b662bcb3182ff4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: shader_uri.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 15, 238, 252, 487, 495, 504, 509, 516, 534, 544, 549, 599, 1606, 1613, 1632, 1634
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use shader_pipeline; // namespace candidate
//   package identities: [{"name":"shader-pipeline","source":"local_or_path","version":"0.1.0"}]
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
// 首条位置: {"module":516,"record_offset":61904,"rva":9865568,"va":5378574688}
// 编译实例 RVAs: 0x968960
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32966774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378574688,"raw_hex":"e87608f701","size":5,"soft_fail":false,"target_va":5411541776,"va":5378574997,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::memchr::memrchr
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4325790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378574688,"raw_hex":"e89e014200","size":5,"soft_fail":false,"target_va":5382900528,"va":5378574733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32968332","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378574688,"raw_hex":"e88c0ef701","size":5,"soft_fail":false,"target_va":5411543360,"va":5378575023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":516,"record_offset":63532,"parent_record_offset":61904,"ancestor_record_offset":61904,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00516.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::rfind
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":516,"record_offset":62060,"parent_record_offset":61904,"ancestor_record_offset":61904,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00516.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":516,"record_offset":62376,"parent_record_offset":61904,"ancestor_record_offset":61904,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00516.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":516,"record_offset":62284,"parent_record_offset":62084,"ancestor_record_offset":61904,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00516.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":516,"record_offset":62164,"parent_record_offset":62140,"ancestor_record_offset":61904,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00516.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":516,"record_offset":62504,"parent_record_offset":62476,"ancestor_record_offset":61904,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00516.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":516,"record_offset":62588,"parent_record_offset":62560,"ancestor_record_offset":61904,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00516.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":516,"record_offset":62308,"parent_record_offset":62284,"ancestor_record_offset":61904,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00516.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":516,"record_offset":62332,"parent_record_offset":62308,"ancestor_record_offset":61904,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00516.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":516,"record_offset":62116,"parent_record_offset":62084,"ancestor_record_offset":61904,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00516.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":516,"record_offset":62140,"parent_record_offset":62116,"ancestor_record_offset":61904,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/00516.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":516,"record_offset":62476,"parent_record_offset":62444,"ancestor_record_offset":61904,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00516.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$2::next_match_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":516,"record_offset":62084,"parent_record_offset":62060,"ancestor_record_offset":61904,"inlinee":18898,"source_line":499,"archive":"_pdb/B/modules/00516.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":516,"record_offset":62444,"parent_record_offset":62408,"ancestor_record_offset":61904,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00516.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":516,"record_offset":62536,"parent_record_offset":62444,"ancestor_record_offset":61904,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00516.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":516,"record_offset":62408,"parent_record_offset":62376,"ancestor_record_offset":61904,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00516.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":516,"record_offset":62560,"parent_record_offset":62536,"ancestor_record_offset":61904,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00516.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::shader_uri::impl$1::source_hash
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x193C
// 首条位置: {"module":238,"inlinee":6460}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash::compute
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":238,"record_offset":680,"parent_record_offset":656,"ancestor_record_offset":656,"inlinee":6433,"source_line":237,"archive":"_pdb/B/modules/00238.jsonl.gz"}
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
// 首条位置: {"module":487,"record_offset":33688,"rva":9061120,"va":5377770240}
// 编译实例 RVAs: 0x8a4300
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bdda0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3040639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377770240,"raw_hex":"e8819ad1ff","size":5,"soft_fail":false,"target_va":5374729632,"va":5377770266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash::compute
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":487,"record_offset":33852,"parent_record_offset":33688,"ancestor_record_offset":33688,"inlinee":6433,"source_line":237,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::shader_uri::impl$1::as_compute_pipeline<alloc::string::String>
// 记录形式: compiled_procedure; instances=1
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":15,"record_offset":20008,"rva":355632,"va":5369064752}
// 编译实例 RVAs: 0x56d30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140539370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5121344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369064752,"raw_hex":"e840254e00","size":5,"soft_fail":false,"target_va":5374186352,"va":5369065003,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6016638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369064752,"raw_hex":"e87ece5b00","size":5,"soft_fail":false,"target_va":5375081488,"va":5369064845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6016366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369064752,"raw_hex":"e86ecd5b00","size":5,"soft_fail":false,"target_va":5375081504,"va":5369065133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42782284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369064752,"raw_hex":"e84cce8c02","size":5,"soft_fail":false,"target_va":5411847168,"va":5369064879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42475916","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369064752,"raw_hex":"e88c218802","size":5,"soft_fail":false,"target_va":5411540963,"va":5369065042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::shader_uri::ShaderUri::parse
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9509882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369064752,"raw_hex":"e8fa1b9100","size":5,"soft_fail":false,"target_va":5378574688,"va":5369064801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":15,"record_offset":20196,"parent_record_offset":20008,"ancestor_record_offset":20008,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":15,"record_offset":20816,"parent_record_offset":20008,"ancestor_record_offset":20008,"inlinee":6435,"source_line":55,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":15,"record_offset":20660,"parent_record_offset":20636,"ancestor_record_offset":20008,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":15,"record_offset":20684,"parent_record_offset":20660,"ancestor_record_offset":20008,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":15,"record_offset":21508,"parent_record_offset":21484,"ancestor_record_offset":20008,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":15,"record_offset":20636,"parent_record_offset":20460,"ancestor_record_offset":20008,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":15,"record_offset":21484,"parent_record_offset":21428,"ancestor_record_offset":20008,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":20400,"parent_record_offset":20372,"ancestor_record_offset":20008,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":21456,"parent_record_offset":21428,"ancestor_record_offset":20008,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":21428,"parent_record_offset":21404,"ancestor_record_offset":20008,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":20460,"parent_record_offset":20428,"ancestor_record_offset":20008,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":20428,"parent_record_offset":20400,"ancestor_record_offset":20008,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":21404,"parent_record_offset":21380,"ancestor_record_offset":20008,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":20496,"parent_record_offset":20460,"ancestor_record_offset":20008,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":15,"record_offset":20336,"parent_record_offset":20308,"ancestor_record_offset":20008,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":15,"record_offset":20308,"parent_record_offset":20280,"ancestor_record_offset":20008,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":15,"record_offset":20252,"parent_record_offset":20224,"ancestor_record_offset":20008,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":15,"record_offset":20280,"parent_record_offset":20252,"ancestor_record_offset":20008,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":15,"record_offset":20224,"parent_record_offset":20196,"ancestor_record_offset":20008,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":15,"record_offset":20860,"parent_record_offset":20816,"ancestor_record_offset":20008,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":15,"record_offset":20372,"parent_record_offset":20336,"ancestor_record_offset":20008,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":15,"record_offset":20520,"parent_record_offset":20496,"ancestor_record_offset":20008,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":15,"record_offset":20544,"parent_record_offset":20520,"ancestor_record_offset":20008,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":15,"record_offset":20760,"parent_record_offset":20736,"ancestor_record_offset":20008,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":15,"record_offset":20592,"parent_record_offset":20568,"ancestor_record_offset":20008,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":15,"record_offset":20568,"parent_record_offset":20544,"ancestor_record_offset":20008,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":15,"record_offset":20736,"parent_record_offset":20336,"ancestor_record_offset":20008,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":15,"record_offset":21332,"parent_record_offset":20816,"ancestor_record_offset":20008,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00015.jsonl.gz"}
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
// 首条位置: {"module":252,"record_offset":70008,"rva":6090992,"va":5374800112}
// 编译实例 RVAs: 0x5cf0f0, 0x8a4330
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140539370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-614015","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374800112,"raw_hex":"e881a1f6ff","size":5,"soft_fail":false,"target_va":5374186352,"va":5374800362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t281279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374800112,"raw_hex":"e8bf4a0400","size":5,"soft_fail":false,"target_va":5375081488,"va":5374800204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t281006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374800112,"raw_hex":"e8ae490400","size":5,"soft_fail":false,"target_va":5375081504,"va":5374800493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409e3ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1308289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377770288,"raw_hex":"e881f61300","size":5,"soft_fail":false,"target_va":5379078832,"va":5377770538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37046925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374800112,"raw_hex":"e88d4a3502","size":5,"soft_fail":false,"target_va":5411847168,"va":5374800238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t36740557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374800112,"raw_hex":"e8cd9d3002","size":5,"soft_fail":false,"target_va":5411540963,"va":5374800401,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::shader_uri::ShaderUri::parse
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3774523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374800112,"raw_hex":"e83b983900","size":5,"soft_fail":false,"target_va":5378574688,"va":5374800160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":252,"record_offset":70184,"parent_record_offset":70008,"ancestor_record_offset":70008,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00252.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":252,"record_offset":70804,"parent_record_offset":70008,"ancestor_record_offset":70008,"inlinee":6435,"source_line":55,"archive":"_pdb/B/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":252,"record_offset":70648,"parent_record_offset":70624,"ancestor_record_offset":70008,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":252,"record_offset":70672,"parent_record_offset":70648,"ancestor_record_offset":70008,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":252,"record_offset":71496,"parent_record_offset":71472,"ancestor_record_offset":70008,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":252,"record_offset":70624,"parent_record_offset":70448,"ancestor_record_offset":70008,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":252,"record_offset":71472,"parent_record_offset":71416,"ancestor_record_offset":70008,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":252,"record_offset":70388,"parent_record_offset":70360,"ancestor_record_offset":70008,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":252,"record_offset":71444,"parent_record_offset":71416,"ancestor_record_offset":70008,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":252,"record_offset":71416,"parent_record_offset":71392,"ancestor_record_offset":70008,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":252,"record_offset":70448,"parent_record_offset":70416,"ancestor_record_offset":70008,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":252,"record_offset":70416,"parent_record_offset":70388,"ancestor_record_offset":70008,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":252,"record_offset":71392,"parent_record_offset":71368,"ancestor_record_offset":70008,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":252,"record_offset":70484,"parent_record_offset":70448,"ancestor_record_offset":70008,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":252,"record_offset":70324,"parent_record_offset":70296,"ancestor_record_offset":70008,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":252,"record_offset":70296,"parent_record_offset":70268,"ancestor_record_offset":70008,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":252,"record_offset":70240,"parent_record_offset":70212,"ancestor_record_offset":70008,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":252,"record_offset":70268,"parent_record_offset":70240,"ancestor_record_offset":70008,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":252,"record_offset":70212,"parent_record_offset":70184,"ancestor_record_offset":70008,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":252,"record_offset":70848,"parent_record_offset":70804,"ancestor_record_offset":70008,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":252,"record_offset":70360,"parent_record_offset":70324,"ancestor_record_offset":70008,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":252,"record_offset":70508,"parent_record_offset":70484,"ancestor_record_offset":70008,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":252,"record_offset":70532,"parent_record_offset":70508,"ancestor_record_offset":70008,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":252,"record_offset":70748,"parent_record_offset":70724,"ancestor_record_offset":70008,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":252,"record_offset":70580,"parent_record_offset":70556,"ancestor_record_offset":70008,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":252,"record_offset":70556,"parent_record_offset":70532,"ancestor_record_offset":70008,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":252,"record_offset":70724,"parent_record_offset":70324,"ancestor_record_offset":70008,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00252.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":252,"record_offset":71320,"parent_record_offset":70804,"ancestor_record_offset":70008,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00252.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::shader_uri::impl$2::source_hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x73A3
// 首条位置: {"module":1632,"inlinee":29603}
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
// IPI IDs: 0x73A4
// 首条位置: {"module":1632,"inlinee":29604}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::shader_uri::impl$3::as_shader_def_value
// 记录形式: inline_instance, inlinee_declaration; instances=23
// 源码行: 75
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x69C4
// 首条位置: {"module":487,"inlinee":27076}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":487,"record_offset":78352,"parent_record_offset":78316,"ancestor_record_offset":78316,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":487,"record_offset":78708,"parent_record_offset":78684,"ancestor_record_offset":78316,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":487,"record_offset":78684,"parent_record_offset":78628,"ancestor_record_offset":78316,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":487,"record_offset":78656,"parent_record_offset":78628,"ancestor_record_offset":78316,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":487,"record_offset":78628,"parent_record_offset":78604,"ancestor_record_offset":78316,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":487,"record_offset":78604,"parent_record_offset":78580,"ancestor_record_offset":78316,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":487,"record_offset":78440,"parent_record_offset":78388,"ancestor_record_offset":78316,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":487,"record_offset":78388,"parent_record_offset":78352,"ancestor_record_offset":78316,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":487,"record_offset":78468,"parent_record_offset":78388,"ancestor_record_offset":78316,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=54
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":487,"record_offset":78532,"parent_record_offset":78388,"ancestor_record_offset":78316,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":487,"record_offset":78496,"parent_record_offset":78388,"ancestor_record_offset":78316,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::shader_uri::impl$4::as_shader_def_value
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B5A
// 首条位置: {"module":495,"inlinee":27482}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":495,"record_offset":92120,"parent_record_offset":92096,"ancestor_record_offset":92096,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":495,"record_offset":92168,"parent_record_offset":92144,"ancestor_record_offset":92096,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":495,"record_offset":92144,"parent_record_offset":92120,"ancestor_record_offset":92096,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::impl$9::hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 116
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11FE7
// 首条位置: {"module":1606,"inlinee":73703}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":5152,"parent_record_offset":5128,"ancestor_record_offset":5128,"inlinee":6316,"source_line":946,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::impl$5::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/rasterize_terrain_stroke.rs
//     evidence={"module":1606,"record_offset":5324,"parent_record_offset":5128,"ancestor_record_offset":5128,"inlinee":73705,"source_line":35,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":5372,"parent_record_offset":5348,"ancestor_record_offset":5128,"inlinee":10581,"source_line":430,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":5276,"parent_record_offset":5248,"ancestor_record_offset":5128,"inlinee":9036,"source_line":363,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":5396,"parent_record_offset":5372,"ancestor_record_offset":5128,"inlinee":15756,"source_line":393,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":5348,"parent_record_offset":5324,"ancestor_record_offset":5128,"inlinee":12339,"source_line":811,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":5176,"parent_record_offset":5152,"ancestor_record_offset":5128,"inlinee":6318,"source_line":863,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":5472,"parent_record_offset":5448,"ancestor_record_offset":5128,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":5224,"parent_record_offset":5200,"ancestor_record_offset":5128,"inlinee":9034,"source_line":236,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":5248,"parent_record_offset":5224,"ancestor_record_offset":5128,"inlinee":9035,"source_line":302,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":5420,"parent_record_offset":5396,"ancestor_record_offset":5128,"inlinee":15757,"source_line":3479,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":5448,"parent_record_offset":5396,"ancestor_record_offset":5128,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":5200,"parent_record_offset":5176,"ancestor_record_offset":5128,"inlinee":9033,"source_line":137,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::taa::impl$2::hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 116
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11FEF
// 首条位置: {"module":1606,"inlinee":73711}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::hash::impls::impl$16::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":23892,"parent_record_offset":23868,"ancestor_record_offset":23868,"inlinee":15885,"source_line":811,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_i32
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":23916,"parent_record_offset":23892,"ancestor_record_offset":23868,"inlinee":10584,"source_line":412,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":23940,"parent_record_offset":23916,"ancestor_record_offset":23868,"inlinee":8974,"source_line":375,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":24016,"parent_record_offset":23992,"ancestor_record_offset":23868,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":23964,"parent_record_offset":23940,"ancestor_record_offset":23868,"inlinee":8975,"source_line":3479,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":23992,"parent_record_offset":23940,"ancestor_record_offset":23868,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::vermeer::impl$8::hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 116
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11FE5
// 首条位置: {"module":1606,"inlinee":73701}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::hash::impls::impl$16::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":3096,"parent_record_offset":3072,"ancestor_record_offset":3072,"inlinee":15885,"source_line":811,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_i32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":3120,"parent_record_offset":3096,"ancestor_record_offset":3072,"inlinee":10584,"source_line":412,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1606,"record_offset":3144,"parent_record_offset":3120,"ancestor_record_offset":3072,"inlinee":8974,"source_line":375,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1606,"record_offset":3220,"parent_record_offset":3196,"ancestor_record_offset":3072,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":3168,"parent_record_offset":3144,"ancestor_record_offset":3072,"inlinee":8975,"source_line":3479,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1606,"record_offset":3196,"parent_record_offset":3144,"ancestor_record_offset":3072,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::cull_shaders::impl$10::source_hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 129
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1235F
// 首条位置: {"module":1632,"inlinee":74591}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash::compute
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":1632,"record_offset":48848,"parent_record_offset":48824,"ancestor_record_offset":48824,"inlinee":6433,"source_line":237,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::cull_shaders::impl$14::source_hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 129
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12365
// 首条位置: {"module":1632,"inlinee":74597}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash::compute
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":1632,"record_offset":72284,"parent_record_offset":72260,"ancestor_record_offset":72260,"inlinee":6433,"source_line":237,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::cull_shaders::impl$18::source_hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 129
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1235D
// 首条位置: {"module":1632,"inlinee":74589}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash::compute
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":1632,"record_offset":37120,"parent_record_offset":37096,"ancestor_record_offset":37096,"inlinee":6433,"source_line":237,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::cull_shaders::impl$22::source_hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 129
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12361
// 首条位置: {"module":1632,"inlinee":74593}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash::compute
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":1632,"record_offset":60588,"parent_record_offset":60564,"ancestor_record_offset":60564,"inlinee":6433,"source_line":237,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::cull_shaders::impl$26::source_hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 129
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12363
// 首条位置: {"module":1632,"inlinee":74595}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash::compute
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":1632,"record_offset":63524,"parent_record_offset":63500,"ancestor_record_offset":63500,"inlinee":6433,"source_line":237,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::cull_shaders::impl$30::source_hash
// 记录形式: compiled_procedure; instances=1
// 源码行: 129
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1634,"record_offset":47520,"rva":23218032,"va":5391927152}
// 编译实例 RVAs: 0x1624770
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141459e20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1878392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391927152,"raw_hex":"e88856e3ff","size":5,"soft_fail":false,"target_va":5390048800,"va":5391927187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash::compute
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":1634,"record_offset":47668,"parent_record_offset":47520,"ancestor_record_offset":47520,"inlinee":6433,"source_line":237,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::cull_shaders::impl$34::source_hash
// 记录形式: compiled_procedure; instances=1
// 源码行: 129
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1634,"record_offset":51400,"rva":23219216,"va":5391928336}
// 编译实例 RVAs: 0x1624c10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141459e20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1879576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391928336,"raw_hex":"e8e851e3ff","size":5,"soft_fail":false,"target_va":5390048800,"va":5391928371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash::compute
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":1634,"record_offset":51548,"parent_record_offset":51400,"ancestor_record_offset":51400,"inlinee":6433,"source_line":237,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::cull_shaders::impl$38::source_hash
// 记录形式: compiled_procedure; instances=1
// 源码行: 129
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1634,"record_offset":55280,"rva":23220416,"va":5391929536}
// 编译实例 RVAs: 0x16250c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141459e20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1880776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391929536,"raw_hex":"e8384de3ff","size":5,"soft_fail":false,"target_va":5390048800,"va":5391929571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash::compute
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":1634,"record_offset":55428,"parent_record_offset":55280,"ancestor_record_offset":55280,"inlinee":6433,"source_line":237,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::cull_shaders::impl$42::source_hash
// 记录形式: compiled_procedure; instances=1
// 源码行: 129
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1634,"record_offset":59160,"rva":23221600,"va":5391930720}
// 编译实例 RVAs: 0x1625560
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141459e20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1881960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391930720,"raw_hex":"e89848e3ff","size":5,"soft_fail":false,"target_va":5390048800,"va":5391930755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash::compute
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":1634,"record_offset":59308,"parent_record_offset":59160,"ancestor_record_offset":59160,"inlinee":6433,"source_line":237,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::bin_lights::impl$2::source_hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 129
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x77BD
// 首条位置: {"module":1632,"inlinee":30653}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash::compute
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":1632,"record_offset":42996,"parent_record_offset":42972,"ancestor_record_offset":42972,"inlinee":6433,"source_line":237,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::dof::impl$11::source_hash
// 记录形式: compiled_procedure; instances=1
// 源码行: 129
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":549,"record_offset":23244,"rva":10870272,"va":5379579392}
// 编译实例 RVAs: 0xa5de00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141459e20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10469368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379579392,"raw_hex":"e8f8bf9f00","size":5,"soft_fail":false,"target_va":5390048800,"va":5379579427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash::compute
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":549,"record_offset":23408,"parent_record_offset":23244,"ancestor_record_offset":23244,"inlinee":6433,"source_line":237,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::dof::impl$15::source_hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 129
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7B1E
// 首条位置: {"module":1632,"inlinee":31518}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash::compute
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":1632,"record_offset":51780,"parent_record_offset":51756,"ancestor_record_offset":51756,"inlinee":6433,"source_line":237,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::dof::impl$19::source_hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 129
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7B21
// 首条位置: {"module":1632,"inlinee":31521}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash::compute
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":1632,"record_offset":40056,"parent_record_offset":40032,"ancestor_record_offset":40032,"inlinee":6433,"source_line":237,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::dof::impl$3::source_hash
// 记录形式: compiled_procedure; instances=1
// 源码行: 129
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":549,"record_offset":15436,"rva":10867904,"va":5379577024}
// 编译实例 RVAs: 0xa5d4c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141459e20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10471736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379577024,"raw_hex":"e838c99f00","size":5,"soft_fail":false,"target_va":5390048800,"va":5379577059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash::compute
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":549,"record_offset":15596,"parent_record_offset":15436,"ancestor_record_offset":15436,"inlinee":6433,"source_line":237,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::dof::impl$7::source_hash
// 记录形式: compiled_procedure; instances=1
// 源码行: 129
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":549,"record_offset":19340,"rva":10869088,"va":5379578208}
// 编译实例 RVAs: 0xa5d960
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141459e20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10470552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379578208,"raw_hex":"e898c49f00","size":5,"soft_fail":false,"target_va":5390048800,"va":5379578243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash::compute
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":549,"record_offset":19500,"parent_record_offset":19340,"ancestor_record_offset":19340,"inlinee":6433,"source_line":237,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::impl$8::source_hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 129
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B69
// 首条位置: {"module":1632,"inlinee":27497}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash::compute
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":1632,"record_offset":66476,"parent_record_offset":66452,"ancestor_record_offset":66452,"inlinee":6433,"source_line":237,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::taa::impl$1::source_hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 129
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7A1B
// 首条位置: {"module":1632,"inlinee":31259}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash::compute
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":1632,"record_offset":57660,"parent_record_offset":57636,"ancestor_record_offset":57636,"inlinee":6433,"source_line":237,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::vermeer::impl$7::source_hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 129
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x69E5
// 首条位置: {"module":1632,"inlinee":27109}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash::compute
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":1632,"record_offset":45920,"parent_record_offset":45896,"ancestor_record_offset":45896,"inlinee":6433,"source_line":237,"archive":"_pdb/B/modules/01632.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::cull_shaders::impl$10::as_compute_pipeline
// 记录形式: compiled_procedure; instances=1
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1634,"record_offset":29020,"rva":23212320,"va":5391921440}
// 编译实例 RVAs: 0x1623120
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ed460
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16998687","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391921440,"raw_hex":"e8e19efcfe","size":5,"soft_fail":false,"target_va":5374923872,"va":5391922554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fe2e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16929247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391921440,"raw_hex":"e821aefdfe","size":5,"soft_fail":false,"target_va":5374993120,"va":5391922362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16840012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391921440,"raw_hex":"e8b40afffe","size":5,"soft_fail":false,"target_va":5375081488,"va":5391921495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16840216","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391921440,"raw_hex":"e8e809fffe","size":5,"soft_fail":false,"target_va":5375081504,"va":5391921715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t543180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391921440,"raw_hex":"e8cc490800","size":5,"soft_fail":false,"target_va":5392465200,"va":5391922015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t19618824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391921440,"raw_hex":"e8085c2b01","size":5,"soft_fail":false,"target_va":5411540963,"va":5391922134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$::fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9002391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391921440,"raw_hex":"e869a276ff","size":5,"soft_fail":false,"target_va":5382919552,"va":5391921938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19620622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391921440,"raw_hex":"e80e632b01","size":5,"soft_fail":false,"target_va":5411542880,"va":5391922253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1634,"record_offset":31300,"parent_record_offset":29020,"ancestor_record_offset":29020,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1634,"record_offset":32324,"parent_record_offset":29020,"ancestor_record_offset":29020,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1634,"record_offset":32520,"parent_record_offset":29020,"ancestor_record_offset":29020,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::impl$3::as_shader_def_value
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":1634,"record_offset":31868,"parent_record_offset":29020,"ancestor_record_offset":29020,"inlinee":27076,"source_line":75,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":1634,"record_offset":29196,"parent_record_offset":29020,"ancestor_record_offset":29020,"inlinee":6435,"source_line":55,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":1634,"record_offset":30456,"parent_record_offset":29020,"ancestor_record_offset":29020,"inlinee":15206,"source_line":63,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":29620,"parent_record_offset":29596,"ancestor_record_offset":29020,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":29644,"parent_record_offset":29620,"ancestor_record_offset":29020,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":30400,"parent_record_offset":30376,"ancestor_record_offset":29020,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":29596,"parent_record_offset":29568,"ancestor_record_offset":29020,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":30376,"parent_record_offset":30352,"ancestor_record_offset":29020,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":32400,"parent_record_offset":32368,"ancestor_record_offset":29020,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":29508,"parent_record_offset":29480,"ancestor_record_offset":29020,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":32208,"parent_record_offset":32180,"ancestor_record_offset":29020,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":30352,"parent_record_offset":30328,"ancestor_record_offset":29020,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":32448,"parent_record_offset":32424,"ancestor_record_offset":29020,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":32424,"parent_record_offset":32400,"ancestor_record_offset":29020,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":29568,"parent_record_offset":29536,"ancestor_record_offset":29020,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":29536,"parent_record_offset":29508,"ancestor_record_offset":29020,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":30328,"parent_record_offset":30304,"ancestor_record_offset":29020,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":29444,"parent_record_offset":29416,"ancestor_record_offset":29020,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":29416,"parent_record_offset":29388,"ancestor_record_offset":29020,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":29360,"parent_record_offset":29332,"ancestor_record_offset":29020,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":29388,"parent_record_offset":29360,"ancestor_record_offset":29020,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":29332,"parent_record_offset":29304,"ancestor_record_offset":29020,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":1634,"record_offset":29304,"parent_record_offset":29276,"ancestor_record_offset":29020,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1634,"record_offset":31992,"parent_record_offset":31940,"ancestor_record_offset":29020,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1634,"record_offset":31904,"parent_record_offset":31868,"ancestor_record_offset":29020,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1634,"record_offset":31940,"parent_record_offset":31904,"ancestor_record_offset":29020,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1634,"record_offset":29276,"parent_record_offset":29248,"ancestor_record_offset":29020,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1634,"record_offset":32368,"parent_record_offset":32324,"ancestor_record_offset":29020,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1634,"record_offset":29480,"parent_record_offset":29444,"ancestor_record_offset":29020,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1634,"record_offset":29248,"parent_record_offset":29196,"ancestor_record_offset":29020,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1634,"record_offset":32020,"parent_record_offset":31940,"ancestor_record_offset":29020,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1634,"record_offset":29720,"parent_record_offset":29696,"ancestor_record_offset":29020,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1634,"record_offset":30528,"parent_record_offset":30500,"ancestor_record_offset":29020,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1634,"record_offset":30500,"parent_record_offset":30456,"ancestor_record_offset":29020,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1634,"record_offset":29696,"parent_record_offset":29444,"ancestor_record_offset":29020,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1634,"record_offset":30256,"parent_record_offset":29196,"ancestor_record_offset":29020,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1634,"record_offset":32488,"parent_record_offset":32324,"ancestor_record_offset":29020,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1634,"record_offset":32048,"parent_record_offset":31940,"ancestor_record_offset":29020,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::cull_shaders::impl$14::as_compute_pipeline
// 记录形式: compiled_procedure; instances=1
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1634,"record_offset":32720,"rva":23213456,"va":5391922576}
// 编译实例 RVAs: 0x1623590
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ed460
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16999839","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391922576,"raw_hex":"e8619afcfe","size":5,"soft_fail":false,"target_va":5374923872,"va":5391923706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fe2e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16930399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391922576,"raw_hex":"e8a1a9fdfe","size":5,"soft_fail":false,"target_va":5374993120,"va":5391923514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16841148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391922576,"raw_hex":"e84406fffe","size":5,"soft_fail":false,"target_va":5375081488,"va":5391922631,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16841366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391922576,"raw_hex":"e86a05fffe","size":5,"soft_fail":false,"target_va":5375081504,"va":5391922865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t542030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391922576,"raw_hex":"e84e450800","size":5,"soft_fail":false,"target_va":5392465200,"va":5391923165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t19617674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391922576,"raw_hex":"e88a572b01","size":5,"soft_fail":false,"target_va":5411540963,"va":5391923284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$::fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9003541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391922576,"raw_hex":"e8eb9d76ff","size":5,"soft_fail":false,"target_va":5382919552,"va":5391923088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19619472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391922576,"raw_hex":"e8905e2b01","size":5,"soft_fail":false,"target_va":5411542880,"va":5391923403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1634,"record_offset":35000,"parent_record_offset":32720,"ancestor_record_offset":32720,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1634,"record_offset":36024,"parent_record_offset":32720,"ancestor_record_offset":32720,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1634,"record_offset":36220,"parent_record_offset":32720,"ancestor_record_offset":32720,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::impl$3::as_shader_def_value
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":1634,"record_offset":35568,"parent_record_offset":32720,"ancestor_record_offset":32720,"inlinee":27076,"source_line":75,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":1634,"record_offset":32896,"parent_record_offset":32720,"ancestor_record_offset":32720,"inlinee":6435,"source_line":55,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":1634,"record_offset":34156,"parent_record_offset":32720,"ancestor_record_offset":32720,"inlinee":15206,"source_line":63,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":33320,"parent_record_offset":33296,"ancestor_record_offset":32720,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":33344,"parent_record_offset":33320,"ancestor_record_offset":32720,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":34100,"parent_record_offset":34076,"ancestor_record_offset":32720,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":33296,"parent_record_offset":33268,"ancestor_record_offset":32720,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":34076,"parent_record_offset":34052,"ancestor_record_offset":32720,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":36100,"parent_record_offset":36068,"ancestor_record_offset":32720,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":33208,"parent_record_offset":33180,"ancestor_record_offset":32720,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":35908,"parent_record_offset":35880,"ancestor_record_offset":32720,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":34052,"parent_record_offset":34028,"ancestor_record_offset":32720,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":36148,"parent_record_offset":36124,"ancestor_record_offset":32720,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":36124,"parent_record_offset":36100,"ancestor_record_offset":32720,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":33268,"parent_record_offset":33236,"ancestor_record_offset":32720,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":33236,"parent_record_offset":33208,"ancestor_record_offset":32720,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":34028,"parent_record_offset":34004,"ancestor_record_offset":32720,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":33144,"parent_record_offset":33116,"ancestor_record_offset":32720,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":33116,"parent_record_offset":33088,"ancestor_record_offset":32720,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":33060,"parent_record_offset":33032,"ancestor_record_offset":32720,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":33088,"parent_record_offset":33060,"ancestor_record_offset":32720,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":33032,"parent_record_offset":33004,"ancestor_record_offset":32720,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":1634,"record_offset":33004,"parent_record_offset":32976,"ancestor_record_offset":32720,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1634,"record_offset":35692,"parent_record_offset":35640,"ancestor_record_offset":32720,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1634,"record_offset":35604,"parent_record_offset":35568,"ancestor_record_offset":32720,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1634,"record_offset":35640,"parent_record_offset":35604,"ancestor_record_offset":32720,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1634,"record_offset":32976,"parent_record_offset":32948,"ancestor_record_offset":32720,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1634,"record_offset":36068,"parent_record_offset":36024,"ancestor_record_offset":32720,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1634,"record_offset":33180,"parent_record_offset":33144,"ancestor_record_offset":32720,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1634,"record_offset":32948,"parent_record_offset":32896,"ancestor_record_offset":32720,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1634,"record_offset":35720,"parent_record_offset":35640,"ancestor_record_offset":32720,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1634,"record_offset":33420,"parent_record_offset":33396,"ancestor_record_offset":32720,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1634,"record_offset":34228,"parent_record_offset":34200,"ancestor_record_offset":32720,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1634,"record_offset":34200,"parent_record_offset":34156,"ancestor_record_offset":32720,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1634,"record_offset":33396,"parent_record_offset":33144,"ancestor_record_offset":32720,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1634,"record_offset":33956,"parent_record_offset":32896,"ancestor_record_offset":32720,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1634,"record_offset":36188,"parent_record_offset":36024,"ancestor_record_offset":32720,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1634,"record_offset":35748,"parent_record_offset":35640,"ancestor_record_offset":32720,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::cull_shaders::impl$18::as_compute_pipeline
// 记录形式: compiled_procedure; instances=1
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1634,"record_offset":36420,"rva":23214608,"va":5391923728}
// 编译实例 RVAs: 0x1623a10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ed460
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17000991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391923728,"raw_hex":"e8e195fcfe","size":5,"soft_fail":false,"target_va":5374923872,"va":5391924858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fe2e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16931551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391923728,"raw_hex":"e821a5fdfe","size":5,"soft_fail":false,"target_va":5374993120,"va":5391924666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16842300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391923728,"raw_hex":"e8c401fffe","size":5,"soft_fail":false,"target_va":5375081488,"va":5391923783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16842518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391923728,"raw_hex":"e8ea00fffe","size":5,"soft_fail":false,"target_va":5375081504,"va":5391924017,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t540878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391923728,"raw_hex":"e8ce400800","size":5,"soft_fail":false,"target_va":5392465200,"va":5391924317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t19616522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391923728,"raw_hex":"e80a532b01","size":5,"soft_fail":false,"target_va":5411540963,"va":5391924436,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$::fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9004693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391923728,"raw_hex":"e86b9976ff","size":5,"soft_fail":false,"target_va":5382919552,"va":5391924240,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19618320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391923728,"raw_hex":"e8105a2b01","size":5,"soft_fail":false,"target_va":5411542880,"va":5391924555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1634,"record_offset":38700,"parent_record_offset":36420,"ancestor_record_offset":36420,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1634,"record_offset":39724,"parent_record_offset":36420,"ancestor_record_offset":36420,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1634,"record_offset":39920,"parent_record_offset":36420,"ancestor_record_offset":36420,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::impl$3::as_shader_def_value
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":1634,"record_offset":39268,"parent_record_offset":36420,"ancestor_record_offset":36420,"inlinee":27076,"source_line":75,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":1634,"record_offset":36596,"parent_record_offset":36420,"ancestor_record_offset":36420,"inlinee":6435,"source_line":55,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":1634,"record_offset":37856,"parent_record_offset":36420,"ancestor_record_offset":36420,"inlinee":15206,"source_line":63,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":37020,"parent_record_offset":36996,"ancestor_record_offset":36420,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":37044,"parent_record_offset":37020,"ancestor_record_offset":36420,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":37800,"parent_record_offset":37776,"ancestor_record_offset":36420,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":36996,"parent_record_offset":36968,"ancestor_record_offset":36420,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":37776,"parent_record_offset":37752,"ancestor_record_offset":36420,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":39800,"parent_record_offset":39768,"ancestor_record_offset":36420,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":36908,"parent_record_offset":36880,"ancestor_record_offset":36420,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":39608,"parent_record_offset":39580,"ancestor_record_offset":36420,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":37752,"parent_record_offset":37728,"ancestor_record_offset":36420,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":39848,"parent_record_offset":39824,"ancestor_record_offset":36420,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":39824,"parent_record_offset":39800,"ancestor_record_offset":36420,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":36968,"parent_record_offset":36936,"ancestor_record_offset":36420,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":36936,"parent_record_offset":36908,"ancestor_record_offset":36420,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":37728,"parent_record_offset":37704,"ancestor_record_offset":36420,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":36844,"parent_record_offset":36816,"ancestor_record_offset":36420,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":36816,"parent_record_offset":36788,"ancestor_record_offset":36420,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":36760,"parent_record_offset":36732,"ancestor_record_offset":36420,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":36788,"parent_record_offset":36760,"ancestor_record_offset":36420,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":36732,"parent_record_offset":36704,"ancestor_record_offset":36420,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":1634,"record_offset":36704,"parent_record_offset":36676,"ancestor_record_offset":36420,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1634,"record_offset":39392,"parent_record_offset":39340,"ancestor_record_offset":36420,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1634,"record_offset":39304,"parent_record_offset":39268,"ancestor_record_offset":36420,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1634,"record_offset":39340,"parent_record_offset":39304,"ancestor_record_offset":36420,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1634,"record_offset":36676,"parent_record_offset":36648,"ancestor_record_offset":36420,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1634,"record_offset":39768,"parent_record_offset":39724,"ancestor_record_offset":36420,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1634,"record_offset":36880,"parent_record_offset":36844,"ancestor_record_offset":36420,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1634,"record_offset":36648,"parent_record_offset":36596,"ancestor_record_offset":36420,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1634,"record_offset":39420,"parent_record_offset":39340,"ancestor_record_offset":36420,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1634,"record_offset":37120,"parent_record_offset":37096,"ancestor_record_offset":36420,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1634,"record_offset":37928,"parent_record_offset":37900,"ancestor_record_offset":36420,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1634,"record_offset":37900,"parent_record_offset":37856,"ancestor_record_offset":36420,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1634,"record_offset":37096,"parent_record_offset":36844,"ancestor_record_offset":36420,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1634,"record_offset":37656,"parent_record_offset":36596,"ancestor_record_offset":36420,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1634,"record_offset":39888,"parent_record_offset":39724,"ancestor_record_offset":36420,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1634,"record_offset":39448,"parent_record_offset":39340,"ancestor_record_offset":36420,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::cull_shaders::impl$22::as_compute_pipeline
// 记录形式: compiled_procedure; instances=1
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1634,"record_offset":40120,"rva":23215760,"va":5391924880}
// 编译实例 RVAs: 0x1623e90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ed460
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17002127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391924880,"raw_hex":"e87191fcfe","size":5,"soft_fail":false,"target_va":5374923872,"va":5391925994,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fe2e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16932687","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391924880,"raw_hex":"e8b1a0fdfe","size":5,"soft_fail":false,"target_va":5374993120,"va":5391925802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16843452","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391924880,"raw_hex":"e844fdfefe","size":5,"soft_fail":false,"target_va":5375081488,"va":5391924935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16843662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391924880,"raw_hex":"e872fcfefe","size":5,"soft_fail":false,"target_va":5375081504,"va":5391925161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t539734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391924880,"raw_hex":"e8563c0800","size":5,"soft_fail":false,"target_va":5392465200,"va":5391925461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t19615378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391924880,"raw_hex":"e8924e2b01","size":5,"soft_fail":false,"target_va":5411540963,"va":5391925580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$::fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9005837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391924880,"raw_hex":"e8f39476ff","size":5,"soft_fail":false,"target_va":5382919552,"va":5391925384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19617176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391924880,"raw_hex":"e898552b01","size":5,"soft_fail":false,"target_va":5411542880,"va":5391925699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1634,"record_offset":42400,"parent_record_offset":40120,"ancestor_record_offset":40120,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1634,"record_offset":43424,"parent_record_offset":40120,"ancestor_record_offset":40120,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1634,"record_offset":43620,"parent_record_offset":40120,"ancestor_record_offset":40120,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::impl$3::as_shader_def_value
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":1634,"record_offset":42968,"parent_record_offset":40120,"ancestor_record_offset":40120,"inlinee":27076,"source_line":75,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":1634,"record_offset":40296,"parent_record_offset":40120,"ancestor_record_offset":40120,"inlinee":6435,"source_line":55,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":1634,"record_offset":41556,"parent_record_offset":40120,"ancestor_record_offset":40120,"inlinee":15206,"source_line":63,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":40720,"parent_record_offset":40696,"ancestor_record_offset":40120,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":40744,"parent_record_offset":40720,"ancestor_record_offset":40120,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":41500,"parent_record_offset":41476,"ancestor_record_offset":40120,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":40696,"parent_record_offset":40668,"ancestor_record_offset":40120,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":41476,"parent_record_offset":41452,"ancestor_record_offset":40120,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":43500,"parent_record_offset":43468,"ancestor_record_offset":40120,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":40608,"parent_record_offset":40580,"ancestor_record_offset":40120,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":43308,"parent_record_offset":43280,"ancestor_record_offset":40120,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":41452,"parent_record_offset":41428,"ancestor_record_offset":40120,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":43548,"parent_record_offset":43524,"ancestor_record_offset":40120,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":43524,"parent_record_offset":43500,"ancestor_record_offset":40120,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":40668,"parent_record_offset":40636,"ancestor_record_offset":40120,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":40636,"parent_record_offset":40608,"ancestor_record_offset":40120,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":41428,"parent_record_offset":41404,"ancestor_record_offset":40120,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":40544,"parent_record_offset":40516,"ancestor_record_offset":40120,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":40516,"parent_record_offset":40488,"ancestor_record_offset":40120,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":40460,"parent_record_offset":40432,"ancestor_record_offset":40120,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":40488,"parent_record_offset":40460,"ancestor_record_offset":40120,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":40432,"parent_record_offset":40404,"ancestor_record_offset":40120,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":1634,"record_offset":40404,"parent_record_offset":40376,"ancestor_record_offset":40120,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1634,"record_offset":43092,"parent_record_offset":43040,"ancestor_record_offset":40120,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1634,"record_offset":43004,"parent_record_offset":42968,"ancestor_record_offset":40120,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1634,"record_offset":43040,"parent_record_offset":43004,"ancestor_record_offset":40120,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1634,"record_offset":40376,"parent_record_offset":40348,"ancestor_record_offset":40120,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1634,"record_offset":43468,"parent_record_offset":43424,"ancestor_record_offset":40120,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1634,"record_offset":40580,"parent_record_offset":40544,"ancestor_record_offset":40120,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1634,"record_offset":40348,"parent_record_offset":40296,"ancestor_record_offset":40120,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1634,"record_offset":43120,"parent_record_offset":43040,"ancestor_record_offset":40120,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1634,"record_offset":40820,"parent_record_offset":40796,"ancestor_record_offset":40120,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1634,"record_offset":41628,"parent_record_offset":41600,"ancestor_record_offset":40120,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1634,"record_offset":41600,"parent_record_offset":41556,"ancestor_record_offset":40120,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1634,"record_offset":40796,"parent_record_offset":40544,"ancestor_record_offset":40120,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1634,"record_offset":41356,"parent_record_offset":40296,"ancestor_record_offset":40120,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1634,"record_offset":43588,"parent_record_offset":43424,"ancestor_record_offset":40120,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1634,"record_offset":43148,"parent_record_offset":43040,"ancestor_record_offset":40120,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::cull_shaders::impl$26::as_compute_pipeline
// 记录形式: compiled_procedure; instances=1
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1634,"record_offset":43820,"rva":23216896,"va":5391926016}
// 编译实例 RVAs: 0x1624300
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ed460
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17003263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391926016,"raw_hex":"e8018dfcfe","size":5,"soft_fail":false,"target_va":5374923872,"va":5391927130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fe2e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16933823","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391926016,"raw_hex":"e8419cfdfe","size":5,"soft_fail":false,"target_va":5374993120,"va":5391926938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16844588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391926016,"raw_hex":"e8d4f8fefe","size":5,"soft_fail":false,"target_va":5375081488,"va":5391926071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16844803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391926016,"raw_hex":"e8fdf7fefe","size":5,"soft_fail":false,"target_va":5375081504,"va":5391926302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t538593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391926016,"raw_hex":"e8e1370800","size":5,"soft_fail":false,"target_va":5392465200,"va":5391926602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t19614237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391926016,"raw_hex":"e81d4a2b01","size":5,"soft_fail":false,"target_va":5411540963,"va":5391926721,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$::fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9006978","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391926016,"raw_hex":"e87e9076ff","size":5,"soft_fail":false,"target_va":5382919552,"va":5391926525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19616035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391926016,"raw_hex":"e823512b01","size":5,"soft_fail":false,"target_va":5411542880,"va":5391926840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1634,"record_offset":46100,"parent_record_offset":43820,"ancestor_record_offset":43820,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1634,"record_offset":47124,"parent_record_offset":43820,"ancestor_record_offset":43820,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1634,"record_offset":47320,"parent_record_offset":43820,"ancestor_record_offset":43820,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::impl$3::as_shader_def_value
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":1634,"record_offset":46668,"parent_record_offset":43820,"ancestor_record_offset":43820,"inlinee":27076,"source_line":75,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":1634,"record_offset":43996,"parent_record_offset":43820,"ancestor_record_offset":43820,"inlinee":6435,"source_line":55,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":1634,"record_offset":45256,"parent_record_offset":43820,"ancestor_record_offset":43820,"inlinee":15206,"source_line":63,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":44420,"parent_record_offset":44396,"ancestor_record_offset":43820,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":44444,"parent_record_offset":44420,"ancestor_record_offset":43820,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":45200,"parent_record_offset":45176,"ancestor_record_offset":43820,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":44396,"parent_record_offset":44368,"ancestor_record_offset":43820,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":45176,"parent_record_offset":45152,"ancestor_record_offset":43820,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":47200,"parent_record_offset":47168,"ancestor_record_offset":43820,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":44308,"parent_record_offset":44280,"ancestor_record_offset":43820,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":47008,"parent_record_offset":46980,"ancestor_record_offset":43820,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":45152,"parent_record_offset":45128,"ancestor_record_offset":43820,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":47248,"parent_record_offset":47224,"ancestor_record_offset":43820,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":47224,"parent_record_offset":47200,"ancestor_record_offset":43820,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":44368,"parent_record_offset":44336,"ancestor_record_offset":43820,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":44336,"parent_record_offset":44308,"ancestor_record_offset":43820,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":45128,"parent_record_offset":45104,"ancestor_record_offset":43820,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":44244,"parent_record_offset":44216,"ancestor_record_offset":43820,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":44216,"parent_record_offset":44188,"ancestor_record_offset":43820,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":44160,"parent_record_offset":44132,"ancestor_record_offset":43820,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":44188,"parent_record_offset":44160,"ancestor_record_offset":43820,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":44132,"parent_record_offset":44104,"ancestor_record_offset":43820,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":1634,"record_offset":44104,"parent_record_offset":44076,"ancestor_record_offset":43820,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1634,"record_offset":46792,"parent_record_offset":46740,"ancestor_record_offset":43820,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1634,"record_offset":46704,"parent_record_offset":46668,"ancestor_record_offset":43820,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1634,"record_offset":46740,"parent_record_offset":46704,"ancestor_record_offset":43820,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1634,"record_offset":44076,"parent_record_offset":44048,"ancestor_record_offset":43820,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1634,"record_offset":47168,"parent_record_offset":47124,"ancestor_record_offset":43820,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1634,"record_offset":44280,"parent_record_offset":44244,"ancestor_record_offset":43820,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1634,"record_offset":44048,"parent_record_offset":43996,"ancestor_record_offset":43820,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1634,"record_offset":46820,"parent_record_offset":46740,"ancestor_record_offset":43820,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1634,"record_offset":44520,"parent_record_offset":44496,"ancestor_record_offset":43820,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1634,"record_offset":45328,"parent_record_offset":45300,"ancestor_record_offset":43820,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1634,"record_offset":45300,"parent_record_offset":45256,"ancestor_record_offset":43820,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1634,"record_offset":44496,"parent_record_offset":44244,"ancestor_record_offset":43820,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1634,"record_offset":45056,"parent_record_offset":43996,"ancestor_record_offset":43820,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1634,"record_offset":47288,"parent_record_offset":47124,"ancestor_record_offset":43820,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1634,"record_offset":46848,"parent_record_offset":46740,"ancestor_record_offset":43820,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::cull_shaders::impl$30::as_compute_pipeline
// 记录形式: compiled_procedure; instances=1
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1634,"record_offset":47700,"rva":23218080,"va":5391927200}
// 编译实例 RVAs: 0x16247a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ed460
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17004447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391927200,"raw_hex":"e86188fcfe","size":5,"soft_fail":false,"target_va":5374923872,"va":5391928314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fe2e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16935007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391927200,"raw_hex":"e8a197fdfe","size":5,"soft_fail":false,"target_va":5374993120,"va":5391928122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16845772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391927200,"raw_hex":"e834f4fefe","size":5,"soft_fail":false,"target_va":5375081488,"va":5391927255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16845987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391927200,"raw_hex":"e85df3fefe","size":5,"soft_fail":false,"target_va":5375081504,"va":5391927486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t537409","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391927200,"raw_hex":"e841330800","size":5,"soft_fail":false,"target_va":5392465200,"va":5391927786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t19613053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391927200,"raw_hex":"e87d452b01","size":5,"soft_fail":false,"target_va":5411540963,"va":5391927905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$::fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9008162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391927200,"raw_hex":"e8de8b76ff","size":5,"soft_fail":false,"target_va":5382919552,"va":5391927709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19614851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391927200,"raw_hex":"e8834c2b01","size":5,"soft_fail":false,"target_va":5411542880,"va":5391928024,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1634,"record_offset":49980,"parent_record_offset":47700,"ancestor_record_offset":47700,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1634,"record_offset":51004,"parent_record_offset":47700,"ancestor_record_offset":47700,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1634,"record_offset":51200,"parent_record_offset":47700,"ancestor_record_offset":47700,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::impl$3::as_shader_def_value
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":1634,"record_offset":50548,"parent_record_offset":47700,"ancestor_record_offset":47700,"inlinee":27076,"source_line":75,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":1634,"record_offset":47876,"parent_record_offset":47700,"ancestor_record_offset":47700,"inlinee":6435,"source_line":55,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":1634,"record_offset":49136,"parent_record_offset":47700,"ancestor_record_offset":47700,"inlinee":15206,"source_line":63,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":48300,"parent_record_offset":48276,"ancestor_record_offset":47700,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":48324,"parent_record_offset":48300,"ancestor_record_offset":47700,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":49080,"parent_record_offset":49056,"ancestor_record_offset":47700,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":48276,"parent_record_offset":48248,"ancestor_record_offset":47700,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":49056,"parent_record_offset":49032,"ancestor_record_offset":47700,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":51080,"parent_record_offset":51048,"ancestor_record_offset":47700,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":48188,"parent_record_offset":48160,"ancestor_record_offset":47700,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":50888,"parent_record_offset":50860,"ancestor_record_offset":47700,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":49032,"parent_record_offset":49008,"ancestor_record_offset":47700,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":51128,"parent_record_offset":51104,"ancestor_record_offset":47700,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":51104,"parent_record_offset":51080,"ancestor_record_offset":47700,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":48248,"parent_record_offset":48216,"ancestor_record_offset":47700,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":48216,"parent_record_offset":48188,"ancestor_record_offset":47700,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":49008,"parent_record_offset":48984,"ancestor_record_offset":47700,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":48124,"parent_record_offset":48096,"ancestor_record_offset":47700,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":48096,"parent_record_offset":48068,"ancestor_record_offset":47700,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":48040,"parent_record_offset":48012,"ancestor_record_offset":47700,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":48068,"parent_record_offset":48040,"ancestor_record_offset":47700,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":48012,"parent_record_offset":47984,"ancestor_record_offset":47700,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":1634,"record_offset":47984,"parent_record_offset":47956,"ancestor_record_offset":47700,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1634,"record_offset":50672,"parent_record_offset":50620,"ancestor_record_offset":47700,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1634,"record_offset":50584,"parent_record_offset":50548,"ancestor_record_offset":47700,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1634,"record_offset":50620,"parent_record_offset":50584,"ancestor_record_offset":47700,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1634,"record_offset":47956,"parent_record_offset":47928,"ancestor_record_offset":47700,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1634,"record_offset":51048,"parent_record_offset":51004,"ancestor_record_offset":47700,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1634,"record_offset":48160,"parent_record_offset":48124,"ancestor_record_offset":47700,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1634,"record_offset":47928,"parent_record_offset":47876,"ancestor_record_offset":47700,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1634,"record_offset":50700,"parent_record_offset":50620,"ancestor_record_offset":47700,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1634,"record_offset":48400,"parent_record_offset":48376,"ancestor_record_offset":47700,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1634,"record_offset":49208,"parent_record_offset":49180,"ancestor_record_offset":47700,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1634,"record_offset":49180,"parent_record_offset":49136,"ancestor_record_offset":47700,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1634,"record_offset":48376,"parent_record_offset":48124,"ancestor_record_offset":47700,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1634,"record_offset":48936,"parent_record_offset":47876,"ancestor_record_offset":47700,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1634,"record_offset":51168,"parent_record_offset":51004,"ancestor_record_offset":47700,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1634,"record_offset":50728,"parent_record_offset":50620,"ancestor_record_offset":47700,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::cull_shaders::impl$34::as_compute_pipeline
// 记录形式: compiled_procedure; instances=1
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1634,"record_offset":51580,"rva":23219264,"va":5391928384}
// 编译实例 RVAs: 0x1624c40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ed460
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17005647","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391928384,"raw_hex":"e8b183fcfe","size":5,"soft_fail":false,"target_va":5374923872,"va":5391929514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fe2e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16936207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391928384,"raw_hex":"e8f192fdfe","size":5,"soft_fail":false,"target_va":5374993120,"va":5391929322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16846956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391928384,"raw_hex":"e894effefe","size":5,"soft_fail":false,"target_va":5375081488,"va":5391928439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16847174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391928384,"raw_hex":"e8baeefefe","size":5,"soft_fail":false,"target_va":5375081504,"va":5391928673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t536222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391928384,"raw_hex":"e89e2e0800","size":5,"soft_fail":false,"target_va":5392465200,"va":5391928973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t19611866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391928384,"raw_hex":"e8da402b01","size":5,"soft_fail":false,"target_va":5411540963,"va":5391929092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$::fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9009349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391928384,"raw_hex":"e83b8776ff","size":5,"soft_fail":false,"target_va":5382919552,"va":5391928896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19613664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391928384,"raw_hex":"e8e0472b01","size":5,"soft_fail":false,"target_va":5411542880,"va":5391929211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1634,"record_offset":53860,"parent_record_offset":51580,"ancestor_record_offset":51580,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1634,"record_offset":54884,"parent_record_offset":51580,"ancestor_record_offset":51580,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1634,"record_offset":55080,"parent_record_offset":51580,"ancestor_record_offset":51580,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::impl$3::as_shader_def_value
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":1634,"record_offset":54428,"parent_record_offset":51580,"ancestor_record_offset":51580,"inlinee":27076,"source_line":75,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":1634,"record_offset":51756,"parent_record_offset":51580,"ancestor_record_offset":51580,"inlinee":6435,"source_line":55,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":1634,"record_offset":53016,"parent_record_offset":51580,"ancestor_record_offset":51580,"inlinee":15206,"source_line":63,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":52180,"parent_record_offset":52156,"ancestor_record_offset":51580,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":52204,"parent_record_offset":52180,"ancestor_record_offset":51580,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":52960,"parent_record_offset":52936,"ancestor_record_offset":51580,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":52156,"parent_record_offset":52128,"ancestor_record_offset":51580,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":52936,"parent_record_offset":52912,"ancestor_record_offset":51580,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":54960,"parent_record_offset":54928,"ancestor_record_offset":51580,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":52068,"parent_record_offset":52040,"ancestor_record_offset":51580,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":54768,"parent_record_offset":54740,"ancestor_record_offset":51580,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":52912,"parent_record_offset":52888,"ancestor_record_offset":51580,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":55008,"parent_record_offset":54984,"ancestor_record_offset":51580,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":54984,"parent_record_offset":54960,"ancestor_record_offset":51580,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":52128,"parent_record_offset":52096,"ancestor_record_offset":51580,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":52096,"parent_record_offset":52068,"ancestor_record_offset":51580,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":52888,"parent_record_offset":52864,"ancestor_record_offset":51580,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":52004,"parent_record_offset":51976,"ancestor_record_offset":51580,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":51976,"parent_record_offset":51948,"ancestor_record_offset":51580,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":51920,"parent_record_offset":51892,"ancestor_record_offset":51580,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":51948,"parent_record_offset":51920,"ancestor_record_offset":51580,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":51892,"parent_record_offset":51864,"ancestor_record_offset":51580,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":1634,"record_offset":51864,"parent_record_offset":51836,"ancestor_record_offset":51580,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1634,"record_offset":54552,"parent_record_offset":54500,"ancestor_record_offset":51580,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1634,"record_offset":54464,"parent_record_offset":54428,"ancestor_record_offset":51580,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1634,"record_offset":54500,"parent_record_offset":54464,"ancestor_record_offset":51580,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1634,"record_offset":51836,"parent_record_offset":51808,"ancestor_record_offset":51580,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1634,"record_offset":54928,"parent_record_offset":54884,"ancestor_record_offset":51580,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1634,"record_offset":52040,"parent_record_offset":52004,"ancestor_record_offset":51580,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1634,"record_offset":51808,"parent_record_offset":51756,"ancestor_record_offset":51580,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1634,"record_offset":54580,"parent_record_offset":54500,"ancestor_record_offset":51580,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1634,"record_offset":52280,"parent_record_offset":52256,"ancestor_record_offset":51580,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1634,"record_offset":53088,"parent_record_offset":53060,"ancestor_record_offset":51580,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1634,"record_offset":53060,"parent_record_offset":53016,"ancestor_record_offset":51580,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1634,"record_offset":52256,"parent_record_offset":52004,"ancestor_record_offset":51580,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1634,"record_offset":52816,"parent_record_offset":51756,"ancestor_record_offset":51580,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1634,"record_offset":55048,"parent_record_offset":54884,"ancestor_record_offset":51580,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1634,"record_offset":54608,"parent_record_offset":54500,"ancestor_record_offset":51580,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::cull_shaders::impl$38::as_compute_pipeline
// 记录形式: compiled_procedure; instances=1
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1634,"record_offset":55460,"rva":23220464,"va":5391929584}
// 编译实例 RVAs: 0x16250f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ed460
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17006831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391929584,"raw_hex":"e8117ffcfe","size":5,"soft_fail":false,"target_va":5374923872,"va":5391930698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fe2e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16937391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391929584,"raw_hex":"e8518efdfe","size":5,"soft_fail":false,"target_va":5374993120,"va":5391930506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16848156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391929584,"raw_hex":"e8e4eafefe","size":5,"soft_fail":false,"target_va":5375081488,"va":5391929639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16848367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391929584,"raw_hex":"e811eafefe","size":5,"soft_fail":false,"target_va":5375081504,"va":5391929866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t535029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391929584,"raw_hex":"e8f5290800","size":5,"soft_fail":false,"target_va":5392465200,"va":5391930166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t19610673","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391929584,"raw_hex":"e8313c2b01","size":5,"soft_fail":false,"target_va":5411540963,"va":5391930285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$::fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9010542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391929584,"raw_hex":"e8928276ff","size":5,"soft_fail":false,"target_va":5382919552,"va":5391930089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19612471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391929584,"raw_hex":"e837432b01","size":5,"soft_fail":false,"target_va":5411542880,"va":5391930404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1634,"record_offset":57740,"parent_record_offset":55460,"ancestor_record_offset":55460,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1634,"record_offset":58764,"parent_record_offset":55460,"ancestor_record_offset":55460,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1634,"record_offset":58960,"parent_record_offset":55460,"ancestor_record_offset":55460,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::impl$3::as_shader_def_value
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":1634,"record_offset":58308,"parent_record_offset":55460,"ancestor_record_offset":55460,"inlinee":27076,"source_line":75,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":1634,"record_offset":55636,"parent_record_offset":55460,"ancestor_record_offset":55460,"inlinee":6435,"source_line":55,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":1634,"record_offset":56896,"parent_record_offset":55460,"ancestor_record_offset":55460,"inlinee":15206,"source_line":63,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":56060,"parent_record_offset":56036,"ancestor_record_offset":55460,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":56084,"parent_record_offset":56060,"ancestor_record_offset":55460,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":56840,"parent_record_offset":56816,"ancestor_record_offset":55460,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":56036,"parent_record_offset":56008,"ancestor_record_offset":55460,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":56816,"parent_record_offset":56792,"ancestor_record_offset":55460,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":58840,"parent_record_offset":58808,"ancestor_record_offset":55460,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":55948,"parent_record_offset":55920,"ancestor_record_offset":55460,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":58648,"parent_record_offset":58620,"ancestor_record_offset":55460,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":56792,"parent_record_offset":56768,"ancestor_record_offset":55460,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":58888,"parent_record_offset":58864,"ancestor_record_offset":55460,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":58864,"parent_record_offset":58840,"ancestor_record_offset":55460,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":56008,"parent_record_offset":55976,"ancestor_record_offset":55460,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":55976,"parent_record_offset":55948,"ancestor_record_offset":55460,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":56768,"parent_record_offset":56744,"ancestor_record_offset":55460,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":55884,"parent_record_offset":55856,"ancestor_record_offset":55460,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":55856,"parent_record_offset":55828,"ancestor_record_offset":55460,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":55800,"parent_record_offset":55772,"ancestor_record_offset":55460,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":55828,"parent_record_offset":55800,"ancestor_record_offset":55460,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":55772,"parent_record_offset":55744,"ancestor_record_offset":55460,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":1634,"record_offset":55744,"parent_record_offset":55716,"ancestor_record_offset":55460,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1634,"record_offset":58432,"parent_record_offset":58380,"ancestor_record_offset":55460,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1634,"record_offset":58344,"parent_record_offset":58308,"ancestor_record_offset":55460,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1634,"record_offset":58380,"parent_record_offset":58344,"ancestor_record_offset":55460,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1634,"record_offset":55716,"parent_record_offset":55688,"ancestor_record_offset":55460,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1634,"record_offset":58808,"parent_record_offset":58764,"ancestor_record_offset":55460,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1634,"record_offset":55920,"parent_record_offset":55884,"ancestor_record_offset":55460,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1634,"record_offset":55688,"parent_record_offset":55636,"ancestor_record_offset":55460,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1634,"record_offset":58460,"parent_record_offset":58380,"ancestor_record_offset":55460,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1634,"record_offset":56160,"parent_record_offset":56136,"ancestor_record_offset":55460,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1634,"record_offset":56968,"parent_record_offset":56940,"ancestor_record_offset":55460,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1634,"record_offset":56940,"parent_record_offset":56896,"ancestor_record_offset":55460,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1634,"record_offset":56136,"parent_record_offset":55884,"ancestor_record_offset":55460,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1634,"record_offset":56696,"parent_record_offset":55636,"ancestor_record_offset":55460,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1634,"record_offset":58928,"parent_record_offset":58764,"ancestor_record_offset":55460,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1634,"record_offset":58488,"parent_record_offset":58380,"ancestor_record_offset":55460,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::cull_shaders::impl$42::as_compute_pipeline
// 记录形式: compiled_procedure; instances=1
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1634,"record_offset":59340,"rva":23221648,"va":5391930768}
// 编译实例 RVAs: 0x1625590
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ed460
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17008015","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391930768,"raw_hex":"e8717afcfe","size":5,"soft_fail":false,"target_va":5374923872,"va":5391931882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fe2e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16938575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391930768,"raw_hex":"e8b189fdfe","size":5,"soft_fail":false,"target_va":5374993120,"va":5391931690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16849340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391930768,"raw_hex":"e844e6fefe","size":5,"soft_fail":false,"target_va":5375081488,"va":5391930823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16849555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391930768,"raw_hex":"e86de5fefe","size":5,"soft_fail":false,"target_va":5375081504,"va":5391931054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t533841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391930768,"raw_hex":"e851250800","size":5,"soft_fail":false,"target_va":5392465200,"va":5391931354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t19609485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391930768,"raw_hex":"e88d372b01","size":5,"soft_fail":false,"target_va":5411540963,"va":5391931473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$::fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9011730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391930768,"raw_hex":"e8ee7d76ff","size":5,"soft_fail":false,"target_va":5382919552,"va":5391931277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19611283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391930768,"raw_hex":"e8933e2b01","size":5,"soft_fail":false,"target_va":5411542880,"va":5391931592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1634,"record_offset":61620,"parent_record_offset":59340,"ancestor_record_offset":59340,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1634,"record_offset":62644,"parent_record_offset":59340,"ancestor_record_offset":59340,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1634,"record_offset":62840,"parent_record_offset":59340,"ancestor_record_offset":59340,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::impl$3::as_shader_def_value
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":1634,"record_offset":62188,"parent_record_offset":59340,"ancestor_record_offset":59340,"inlinee":27076,"source_line":75,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":1634,"record_offset":59516,"parent_record_offset":59340,"ancestor_record_offset":59340,"inlinee":6435,"source_line":55,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":1634,"record_offset":60776,"parent_record_offset":59340,"ancestor_record_offset":59340,"inlinee":15206,"source_line":63,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":59940,"parent_record_offset":59916,"ancestor_record_offset":59340,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":59964,"parent_record_offset":59940,"ancestor_record_offset":59340,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":60720,"parent_record_offset":60696,"ancestor_record_offset":59340,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":59916,"parent_record_offset":59888,"ancestor_record_offset":59340,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1634,"record_offset":60696,"parent_record_offset":60672,"ancestor_record_offset":59340,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":62720,"parent_record_offset":62688,"ancestor_record_offset":59340,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":59828,"parent_record_offset":59800,"ancestor_record_offset":59340,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":62528,"parent_record_offset":62500,"ancestor_record_offset":59340,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":60672,"parent_record_offset":60648,"ancestor_record_offset":59340,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":62768,"parent_record_offset":62744,"ancestor_record_offset":59340,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":62744,"parent_record_offset":62720,"ancestor_record_offset":59340,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":59888,"parent_record_offset":59856,"ancestor_record_offset":59340,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":59856,"parent_record_offset":59828,"ancestor_record_offset":59340,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1634,"record_offset":60648,"parent_record_offset":60624,"ancestor_record_offset":59340,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":59764,"parent_record_offset":59736,"ancestor_record_offset":59340,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":59736,"parent_record_offset":59708,"ancestor_record_offset":59340,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":59680,"parent_record_offset":59652,"ancestor_record_offset":59340,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":59708,"parent_record_offset":59680,"ancestor_record_offset":59340,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1634,"record_offset":59652,"parent_record_offset":59624,"ancestor_record_offset":59340,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":1634,"record_offset":59624,"parent_record_offset":59596,"ancestor_record_offset":59340,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1634,"record_offset":62312,"parent_record_offset":62260,"ancestor_record_offset":59340,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1634,"record_offset":62224,"parent_record_offset":62188,"ancestor_record_offset":59340,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1634,"record_offset":62260,"parent_record_offset":62224,"ancestor_record_offset":59340,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1634,"record_offset":59596,"parent_record_offset":59568,"ancestor_record_offset":59340,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1634,"record_offset":62688,"parent_record_offset":62644,"ancestor_record_offset":59340,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1634,"record_offset":59800,"parent_record_offset":59764,"ancestor_record_offset":59340,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1634,"record_offset":59568,"parent_record_offset":59516,"ancestor_record_offset":59340,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1634,"record_offset":62340,"parent_record_offset":62260,"ancestor_record_offset":59340,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1634,"record_offset":60040,"parent_record_offset":60016,"ancestor_record_offset":59340,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1634,"record_offset":60848,"parent_record_offset":60820,"ancestor_record_offset":59340,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1634,"record_offset":60820,"parent_record_offset":60776,"ancestor_record_offset":59340,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1634,"record_offset":60016,"parent_record_offset":59764,"ancestor_record_offset":59340,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1634,"record_offset":60576,"parent_record_offset":59516,"ancestor_record_offset":59340,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1634,"record_offset":62808,"parent_record_offset":62644,"ancestor_record_offset":59340,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1634,"record_offset":62368,"parent_record_offset":62260,"ancestor_record_offset":59340,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/01634.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::bin_lights::impl$2::as_compute_pipeline
// 记录形式: compiled_procedure; instances=1
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":534,"record_offset":53700,"rva":10390960,"va":5379100080}
// 编译实例 RVAs: 0x9e8db0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ed460
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4177327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379100080,"raw_hex":"e85142c0ff","size":5,"soft_fail":false,"target_va":5374923872,"va":5379101194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fe2e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4107887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379100080,"raw_hex":"e89151c1ff","size":5,"soft_fail":false,"target_va":5374993120,"va":5379101002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4018652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379100080,"raw_hex":"e824aec2ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379100135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4018863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379100080,"raw_hex":"e851adc2ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379100362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13364534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379100080,"raw_hex":"e836edcb00","size":5,"soft_fail":false,"target_va":5392465200,"va":5379100661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t32440178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379100080,"raw_hex":"e872ffee01","size":5,"soft_fail":false,"target_va":5411540963,"va":5379100780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$::fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3818963","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379100080,"raw_hex":"e8d3453a00","size":5,"soft_fail":false,"target_va":5382919552,"va":5379100584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32441976","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379100080,"raw_hex":"e87806ef01","size":5,"soft_fail":false,"target_va":5411542880,"va":5379100899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":534,"record_offset":56000,"parent_record_offset":53700,"ancestor_record_offset":53700,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":534,"record_offset":57024,"parent_record_offset":53700,"ancestor_record_offset":53700,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":534,"record_offset":57220,"parent_record_offset":53700,"ancestor_record_offset":53700,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::impl$3::as_shader_def_value
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":534,"record_offset":56568,"parent_record_offset":53700,"ancestor_record_offset":53700,"inlinee":27076,"source_line":75,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":534,"record_offset":53896,"parent_record_offset":53700,"ancestor_record_offset":53700,"inlinee":6435,"source_line":55,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":534,"record_offset":55156,"parent_record_offset":53700,"ancestor_record_offset":53700,"inlinee":15206,"source_line":63,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":54320,"parent_record_offset":54296,"ancestor_record_offset":53700,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":54344,"parent_record_offset":54320,"ancestor_record_offset":53700,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":55100,"parent_record_offset":55076,"ancestor_record_offset":53700,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":54296,"parent_record_offset":54268,"ancestor_record_offset":53700,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":55076,"parent_record_offset":55052,"ancestor_record_offset":53700,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":57100,"parent_record_offset":57068,"ancestor_record_offset":53700,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":54208,"parent_record_offset":54180,"ancestor_record_offset":53700,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":56908,"parent_record_offset":56880,"ancestor_record_offset":53700,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":55052,"parent_record_offset":55028,"ancestor_record_offset":53700,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":57148,"parent_record_offset":57124,"ancestor_record_offset":53700,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":57124,"parent_record_offset":57100,"ancestor_record_offset":53700,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":54268,"parent_record_offset":54236,"ancestor_record_offset":53700,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":54236,"parent_record_offset":54208,"ancestor_record_offset":53700,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":55028,"parent_record_offset":55004,"ancestor_record_offset":53700,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":534,"record_offset":54144,"parent_record_offset":54116,"ancestor_record_offset":53700,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":534,"record_offset":54116,"parent_record_offset":54088,"ancestor_record_offset":53700,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":534,"record_offset":54060,"parent_record_offset":54032,"ancestor_record_offset":53700,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":534,"record_offset":54088,"parent_record_offset":54060,"ancestor_record_offset":53700,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":534,"record_offset":54032,"parent_record_offset":54004,"ancestor_record_offset":53700,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":534,"record_offset":54004,"parent_record_offset":53976,"ancestor_record_offset":53700,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":534,"record_offset":56692,"parent_record_offset":56640,"ancestor_record_offset":53700,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":534,"record_offset":56604,"parent_record_offset":56568,"ancestor_record_offset":53700,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":534,"record_offset":56640,"parent_record_offset":56604,"ancestor_record_offset":53700,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":534,"record_offset":53976,"parent_record_offset":53948,"ancestor_record_offset":53700,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":534,"record_offset":57068,"parent_record_offset":57024,"ancestor_record_offset":53700,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":534,"record_offset":54180,"parent_record_offset":54144,"ancestor_record_offset":53700,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":534,"record_offset":53948,"parent_record_offset":53896,"ancestor_record_offset":53700,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":534,"record_offset":56720,"parent_record_offset":56640,"ancestor_record_offset":53700,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":534,"record_offset":54420,"parent_record_offset":54396,"ancestor_record_offset":53700,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":534,"record_offset":55228,"parent_record_offset":55200,"ancestor_record_offset":53700,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":534,"record_offset":55200,"parent_record_offset":55156,"ancestor_record_offset":53700,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":534,"record_offset":54396,"parent_record_offset":54144,"ancestor_record_offset":53700,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":534,"record_offset":54956,"parent_record_offset":53896,"ancestor_record_offset":53700,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":534,"record_offset":57188,"parent_record_offset":57024,"ancestor_record_offset":53700,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":534,"record_offset":56748,"parent_record_offset":56640,"ancestor_record_offset":53700,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::dof::impl$11::as_compute_pipeline
// 记录形式: compiled_procedure; instances=1
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":549,"record_offset":23440,"rva":10870320,"va":5379579440}
// 编译实例 RVAs: 0xa5de30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ed460
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4656671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379579440,"raw_hex":"e8e1f1b8ff","size":5,"soft_fail":false,"target_va":5374923872,"va":5379580538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fe2e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4587231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379579440,"raw_hex":"e82101baff","size":5,"soft_fail":false,"target_va":5374993120,"va":5379580346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4498012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379579440,"raw_hex":"e8a45dbbff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379579495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4498212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379579440,"raw_hex":"e8dc5cbbff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379579711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12885184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379579440,"raw_hex":"e8c09cc400","size":5,"soft_fail":false,"target_va":5392465200,"va":5379580011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t31960828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379579440,"raw_hex":"e8fcaee701","size":5,"soft_fail":false,"target_va":5411540963,"va":5379580130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$::fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3339613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379579440,"raw_hex":"e85df53200","size":5,"soft_fail":false,"target_va":5382919552,"va":5379579934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31962626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379579440,"raw_hex":"e802b6e701","size":5,"soft_fail":false,"target_va":5411542880,"va":5379580249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":549,"record_offset":25736,"parent_record_offset":23440,"ancestor_record_offset":23440,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":26760,"parent_record_offset":23440,"ancestor_record_offset":23440,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":549,"record_offset":26956,"parent_record_offset":23440,"ancestor_record_offset":23440,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::impl$3::as_shader_def_value
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":549,"record_offset":26304,"parent_record_offset":23440,"ancestor_record_offset":23440,"inlinee":27076,"source_line":75,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":549,"record_offset":23632,"parent_record_offset":23440,"ancestor_record_offset":23440,"inlinee":6435,"source_line":55,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":549,"record_offset":24892,"parent_record_offset":23440,"ancestor_record_offset":23440,"inlinee":15206,"source_line":63,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":24056,"parent_record_offset":24032,"ancestor_record_offset":23440,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":24080,"parent_record_offset":24056,"ancestor_record_offset":23440,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":24836,"parent_record_offset":24812,"ancestor_record_offset":23440,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":24032,"parent_record_offset":24004,"ancestor_record_offset":23440,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":24812,"parent_record_offset":24788,"ancestor_record_offset":23440,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":26836,"parent_record_offset":26804,"ancestor_record_offset":23440,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":23944,"parent_record_offset":23916,"ancestor_record_offset":23440,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":26644,"parent_record_offset":26616,"ancestor_record_offset":23440,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":24788,"parent_record_offset":24764,"ancestor_record_offset":23440,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":26884,"parent_record_offset":26860,"ancestor_record_offset":23440,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":26860,"parent_record_offset":26836,"ancestor_record_offset":23440,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":24004,"parent_record_offset":23972,"ancestor_record_offset":23440,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":23972,"parent_record_offset":23944,"ancestor_record_offset":23440,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":24764,"parent_record_offset":24740,"ancestor_record_offset":23440,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":549,"record_offset":23880,"parent_record_offset":23852,"ancestor_record_offset":23440,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":549,"record_offset":23852,"parent_record_offset":23824,"ancestor_record_offset":23440,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":549,"record_offset":23796,"parent_record_offset":23768,"ancestor_record_offset":23440,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":549,"record_offset":23824,"parent_record_offset":23796,"ancestor_record_offset":23440,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":549,"record_offset":23768,"parent_record_offset":23740,"ancestor_record_offset":23440,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":549,"record_offset":23740,"parent_record_offset":23712,"ancestor_record_offset":23440,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":549,"record_offset":26428,"parent_record_offset":26376,"ancestor_record_offset":23440,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":549,"record_offset":26340,"parent_record_offset":26304,"ancestor_record_offset":23440,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":549,"record_offset":26376,"parent_record_offset":26340,"ancestor_record_offset":23440,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":549,"record_offset":23712,"parent_record_offset":23684,"ancestor_record_offset":23440,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":26804,"parent_record_offset":26760,"ancestor_record_offset":23440,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":23916,"parent_record_offset":23880,"ancestor_record_offset":23440,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":549,"record_offset":23684,"parent_record_offset":23632,"ancestor_record_offset":23440,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":549,"record_offset":26456,"parent_record_offset":26376,"ancestor_record_offset":23440,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":549,"record_offset":24156,"parent_record_offset":24132,"ancestor_record_offset":23440,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":549,"record_offset":24964,"parent_record_offset":24936,"ancestor_record_offset":23440,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":549,"record_offset":24936,"parent_record_offset":24892,"ancestor_record_offset":23440,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":549,"record_offset":24132,"parent_record_offset":23880,"ancestor_record_offset":23440,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":549,"record_offset":24692,"parent_record_offset":23632,"ancestor_record_offset":23440,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":549,"record_offset":26924,"parent_record_offset":26760,"ancestor_record_offset":23440,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":549,"record_offset":26484,"parent_record_offset":26376,"ancestor_record_offset":23440,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::dof::impl$15::as_compute_pipeline
// 记录形式: compiled_procedure; instances=1
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":549,"record_offset":27156,"rva":10871440,"va":5379580560}
// 编译实例 RVAs: 0xa5e290
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ed460
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4657807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379580560,"raw_hex":"e871edb8ff","size":5,"soft_fail":false,"target_va":5374923872,"va":5379581674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fe2e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4588367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379580560,"raw_hex":"e8b1fcb9ff","size":5,"soft_fail":false,"target_va":5374993120,"va":5379581482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4499132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379580560,"raw_hex":"e84459bbff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379580615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4499339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379580560,"raw_hex":"e87558bbff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379580838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12884057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379580560,"raw_hex":"e85998c400","size":5,"soft_fail":false,"target_va":5392465200,"va":5379581138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t31959701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379580560,"raw_hex":"e895aae701","size":5,"soft_fail":false,"target_va":5411540963,"va":5379581257,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$::fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3338486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379580560,"raw_hex":"e8f6f03200","size":5,"soft_fail":false,"target_va":5382919552,"va":5379581061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31961499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379580560,"raw_hex":"e89bb1e701","size":5,"soft_fail":false,"target_va":5411542880,"va":5379581376,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":549,"record_offset":29452,"parent_record_offset":27156,"ancestor_record_offset":27156,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":30476,"parent_record_offset":27156,"ancestor_record_offset":27156,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":549,"record_offset":30672,"parent_record_offset":27156,"ancestor_record_offset":27156,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::impl$3::as_shader_def_value
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":549,"record_offset":30020,"parent_record_offset":27156,"ancestor_record_offset":27156,"inlinee":27076,"source_line":75,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":549,"record_offset":27348,"parent_record_offset":27156,"ancestor_record_offset":27156,"inlinee":6435,"source_line":55,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":549,"record_offset":28608,"parent_record_offset":27156,"ancestor_record_offset":27156,"inlinee":15206,"source_line":63,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":27772,"parent_record_offset":27748,"ancestor_record_offset":27156,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":27796,"parent_record_offset":27772,"ancestor_record_offset":27156,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":28552,"parent_record_offset":28528,"ancestor_record_offset":27156,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":27748,"parent_record_offset":27720,"ancestor_record_offset":27156,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":28528,"parent_record_offset":28504,"ancestor_record_offset":27156,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":30552,"parent_record_offset":30520,"ancestor_record_offset":27156,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":27660,"parent_record_offset":27632,"ancestor_record_offset":27156,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":30360,"parent_record_offset":30332,"ancestor_record_offset":27156,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":28504,"parent_record_offset":28480,"ancestor_record_offset":27156,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":30600,"parent_record_offset":30576,"ancestor_record_offset":27156,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":30576,"parent_record_offset":30552,"ancestor_record_offset":27156,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":27720,"parent_record_offset":27688,"ancestor_record_offset":27156,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":27688,"parent_record_offset":27660,"ancestor_record_offset":27156,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":28480,"parent_record_offset":28456,"ancestor_record_offset":27156,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":549,"record_offset":27596,"parent_record_offset":27568,"ancestor_record_offset":27156,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":549,"record_offset":27568,"parent_record_offset":27540,"ancestor_record_offset":27156,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":549,"record_offset":27512,"parent_record_offset":27484,"ancestor_record_offset":27156,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":549,"record_offset":27540,"parent_record_offset":27512,"ancestor_record_offset":27156,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":549,"record_offset":27484,"parent_record_offset":27456,"ancestor_record_offset":27156,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":549,"record_offset":27456,"parent_record_offset":27428,"ancestor_record_offset":27156,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":549,"record_offset":30144,"parent_record_offset":30092,"ancestor_record_offset":27156,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":549,"record_offset":30056,"parent_record_offset":30020,"ancestor_record_offset":27156,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":549,"record_offset":30092,"parent_record_offset":30056,"ancestor_record_offset":27156,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":549,"record_offset":27428,"parent_record_offset":27400,"ancestor_record_offset":27156,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":30520,"parent_record_offset":30476,"ancestor_record_offset":27156,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":27632,"parent_record_offset":27596,"ancestor_record_offset":27156,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":549,"record_offset":27400,"parent_record_offset":27348,"ancestor_record_offset":27156,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":549,"record_offset":30172,"parent_record_offset":30092,"ancestor_record_offset":27156,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":549,"record_offset":27872,"parent_record_offset":27848,"ancestor_record_offset":27156,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":549,"record_offset":28680,"parent_record_offset":28652,"ancestor_record_offset":27156,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":549,"record_offset":28652,"parent_record_offset":28608,"ancestor_record_offset":27156,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":549,"record_offset":27848,"parent_record_offset":27596,"ancestor_record_offset":27156,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":549,"record_offset":28408,"parent_record_offset":27348,"ancestor_record_offset":27156,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":549,"record_offset":30640,"parent_record_offset":30476,"ancestor_record_offset":27156,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":549,"record_offset":30200,"parent_record_offset":30092,"ancestor_record_offset":27156,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::dof::impl$19::as_compute_pipeline
// 记录形式: compiled_procedure; instances=1
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":549,"record_offset":30872,"rva":10872576,"va":5379581696}
// 编译实例 RVAs: 0xa5e700
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ed460
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4658943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379581696,"raw_hex":"e801e9b8ff","size":5,"soft_fail":false,"target_va":5374923872,"va":5379582810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fe2e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4589503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379581696,"raw_hex":"e841f8b9ff","size":5,"soft_fail":false,"target_va":5374993120,"va":5379582618,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4500268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379581696,"raw_hex":"e8d454bbff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379581751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4500482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379581696,"raw_hex":"e8fe53bbff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379581981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12882914","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379581696,"raw_hex":"e8e293c400","size":5,"soft_fail":false,"target_va":5392465200,"va":5379582281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t31958558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379581696,"raw_hex":"e81ea6e701","size":5,"soft_fail":false,"target_va":5411540963,"va":5379582400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$::fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3337343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379581696,"raw_hex":"e87fec3200","size":5,"soft_fail":false,"target_va":5382919552,"va":5379582204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31960356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379581696,"raw_hex":"e824ade701","size":5,"soft_fail":false,"target_va":5411542880,"va":5379582519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":549,"record_offset":33168,"parent_record_offset":30872,"ancestor_record_offset":30872,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":34192,"parent_record_offset":30872,"ancestor_record_offset":30872,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":549,"record_offset":34388,"parent_record_offset":30872,"ancestor_record_offset":30872,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::impl$3::as_shader_def_value
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":549,"record_offset":33736,"parent_record_offset":30872,"ancestor_record_offset":30872,"inlinee":27076,"source_line":75,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":549,"record_offset":31064,"parent_record_offset":30872,"ancestor_record_offset":30872,"inlinee":6435,"source_line":55,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":549,"record_offset":32324,"parent_record_offset":30872,"ancestor_record_offset":30872,"inlinee":15206,"source_line":63,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":31488,"parent_record_offset":31464,"ancestor_record_offset":30872,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":31512,"parent_record_offset":31488,"ancestor_record_offset":30872,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":32268,"parent_record_offset":32244,"ancestor_record_offset":30872,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":31464,"parent_record_offset":31436,"ancestor_record_offset":30872,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":32244,"parent_record_offset":32220,"ancestor_record_offset":30872,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":34268,"parent_record_offset":34236,"ancestor_record_offset":30872,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":31376,"parent_record_offset":31348,"ancestor_record_offset":30872,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":34076,"parent_record_offset":34048,"ancestor_record_offset":30872,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":32220,"parent_record_offset":32196,"ancestor_record_offset":30872,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":34316,"parent_record_offset":34292,"ancestor_record_offset":30872,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":34292,"parent_record_offset":34268,"ancestor_record_offset":30872,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":31436,"parent_record_offset":31404,"ancestor_record_offset":30872,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":31404,"parent_record_offset":31376,"ancestor_record_offset":30872,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":32196,"parent_record_offset":32172,"ancestor_record_offset":30872,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":549,"record_offset":31312,"parent_record_offset":31284,"ancestor_record_offset":30872,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":549,"record_offset":31284,"parent_record_offset":31256,"ancestor_record_offset":30872,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":549,"record_offset":31228,"parent_record_offset":31200,"ancestor_record_offset":30872,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":549,"record_offset":31256,"parent_record_offset":31228,"ancestor_record_offset":30872,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":549,"record_offset":31200,"parent_record_offset":31172,"ancestor_record_offset":30872,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":549,"record_offset":31172,"parent_record_offset":31144,"ancestor_record_offset":30872,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":549,"record_offset":33860,"parent_record_offset":33808,"ancestor_record_offset":30872,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":549,"record_offset":33772,"parent_record_offset":33736,"ancestor_record_offset":30872,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":549,"record_offset":33808,"parent_record_offset":33772,"ancestor_record_offset":30872,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":549,"record_offset":31144,"parent_record_offset":31116,"ancestor_record_offset":30872,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":34236,"parent_record_offset":34192,"ancestor_record_offset":30872,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":31348,"parent_record_offset":31312,"ancestor_record_offset":30872,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":549,"record_offset":31116,"parent_record_offset":31064,"ancestor_record_offset":30872,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":549,"record_offset":33888,"parent_record_offset":33808,"ancestor_record_offset":30872,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":549,"record_offset":31588,"parent_record_offset":31564,"ancestor_record_offset":30872,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":549,"record_offset":32396,"parent_record_offset":32368,"ancestor_record_offset":30872,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":549,"record_offset":32368,"parent_record_offset":32324,"ancestor_record_offset":30872,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":549,"record_offset":31564,"parent_record_offset":31312,"ancestor_record_offset":30872,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":549,"record_offset":32124,"parent_record_offset":31064,"ancestor_record_offset":30872,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":549,"record_offset":34356,"parent_record_offset":34192,"ancestor_record_offset":30872,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":549,"record_offset":33916,"parent_record_offset":33808,"ancestor_record_offset":30872,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::dof::impl$3::as_compute_pipeline
// 记录形式: compiled_procedure; instances=1
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":549,"record_offset":15628,"rva":10867952,"va":5379577072}
// 编译实例 RVAs: 0xa5d4f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ed460
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4654319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379577072,"raw_hex":"e811fbb8ff","size":5,"soft_fail":false,"target_va":5374923872,"va":5379578186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fe2e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4584879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379577072,"raw_hex":"e8510abaff","size":5,"soft_fail":false,"target_va":5374993120,"va":5379577994,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4495644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379577072,"raw_hex":"e8e466bbff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379577127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4495848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379577072,"raw_hex":"e81866bbff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379577347,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12887548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379577072,"raw_hex":"e8fca5c400","size":5,"soft_fail":false,"target_va":5392465200,"va":5379577647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t31963192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379577072,"raw_hex":"e838b8e701","size":5,"soft_fail":false,"target_va":5411540963,"va":5379577766,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$::fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3341977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379577072,"raw_hex":"e899fe3200","size":5,"soft_fail":false,"target_va":5382919552,"va":5379577570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31964990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379577072,"raw_hex":"e83ebfe701","size":5,"soft_fail":false,"target_va":5411542880,"va":5379577885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":549,"record_offset":17920,"parent_record_offset":15628,"ancestor_record_offset":15628,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":18944,"parent_record_offset":15628,"ancestor_record_offset":15628,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":549,"record_offset":19140,"parent_record_offset":15628,"ancestor_record_offset":15628,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::impl$3::as_shader_def_value
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":549,"record_offset":18488,"parent_record_offset":15628,"ancestor_record_offset":15628,"inlinee":27076,"source_line":75,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":549,"record_offset":15816,"parent_record_offset":15628,"ancestor_record_offset":15628,"inlinee":6435,"source_line":55,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":549,"record_offset":17076,"parent_record_offset":15628,"ancestor_record_offset":15628,"inlinee":15206,"source_line":63,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":16240,"parent_record_offset":16216,"ancestor_record_offset":15628,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":16264,"parent_record_offset":16240,"ancestor_record_offset":15628,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":17020,"parent_record_offset":16996,"ancestor_record_offset":15628,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":16216,"parent_record_offset":16188,"ancestor_record_offset":15628,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":16996,"parent_record_offset":16972,"ancestor_record_offset":15628,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":19020,"parent_record_offset":18988,"ancestor_record_offset":15628,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":16128,"parent_record_offset":16100,"ancestor_record_offset":15628,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":18828,"parent_record_offset":18800,"ancestor_record_offset":15628,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":16972,"parent_record_offset":16948,"ancestor_record_offset":15628,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":19068,"parent_record_offset":19044,"ancestor_record_offset":15628,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":19044,"parent_record_offset":19020,"ancestor_record_offset":15628,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":16188,"parent_record_offset":16156,"ancestor_record_offset":15628,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":16156,"parent_record_offset":16128,"ancestor_record_offset":15628,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":16948,"parent_record_offset":16924,"ancestor_record_offset":15628,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":549,"record_offset":16064,"parent_record_offset":16036,"ancestor_record_offset":15628,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":549,"record_offset":16036,"parent_record_offset":16008,"ancestor_record_offset":15628,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":549,"record_offset":15980,"parent_record_offset":15952,"ancestor_record_offset":15628,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":549,"record_offset":16008,"parent_record_offset":15980,"ancestor_record_offset":15628,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":549,"record_offset":15952,"parent_record_offset":15924,"ancestor_record_offset":15628,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":549,"record_offset":15924,"parent_record_offset":15896,"ancestor_record_offset":15628,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":549,"record_offset":18612,"parent_record_offset":18560,"ancestor_record_offset":15628,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":549,"record_offset":18524,"parent_record_offset":18488,"ancestor_record_offset":15628,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":549,"record_offset":18560,"parent_record_offset":18524,"ancestor_record_offset":15628,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":549,"record_offset":15896,"parent_record_offset":15868,"ancestor_record_offset":15628,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":18988,"parent_record_offset":18944,"ancestor_record_offset":15628,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":16100,"parent_record_offset":16064,"ancestor_record_offset":15628,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":549,"record_offset":15868,"parent_record_offset":15816,"ancestor_record_offset":15628,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":549,"record_offset":18640,"parent_record_offset":18560,"ancestor_record_offset":15628,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":549,"record_offset":16340,"parent_record_offset":16316,"ancestor_record_offset":15628,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":549,"record_offset":17148,"parent_record_offset":17120,"ancestor_record_offset":15628,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":549,"record_offset":17120,"parent_record_offset":17076,"ancestor_record_offset":15628,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":549,"record_offset":16316,"parent_record_offset":16064,"ancestor_record_offset":15628,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":549,"record_offset":16876,"parent_record_offset":15816,"ancestor_record_offset":15628,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":549,"record_offset":19108,"parent_record_offset":18944,"ancestor_record_offset":15628,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":549,"record_offset":18668,"parent_record_offset":18560,"ancestor_record_offset":15628,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::dof::impl$7::as_compute_pipeline
// 记录形式: compiled_procedure; instances=1
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":549,"record_offset":19532,"rva":10869136,"va":5379578256}
// 编译实例 RVAs: 0xa5d990
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ed460
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4655503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379578256,"raw_hex":"e871f6b8ff","size":5,"soft_fail":false,"target_va":5374923872,"va":5379579370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fe2e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4586063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379578256,"raw_hex":"e8b105baff","size":5,"soft_fail":false,"target_va":5374993120,"va":5379579178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4496828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379578256,"raw_hex":"e84462bbff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379578311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4497032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379578256,"raw_hex":"e87861bbff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379578531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12886364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379578256,"raw_hex":"e85ca1c400","size":5,"soft_fail":false,"target_va":5392465200,"va":5379578831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t31962008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379578256,"raw_hex":"e898b3e701","size":5,"soft_fail":false,"target_va":5411540963,"va":5379578950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$::fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3340793","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379578256,"raw_hex":"e8f9f93200","size":5,"soft_fail":false,"target_va":5382919552,"va":5379578754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31963806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379578256,"raw_hex":"e89ebae701","size":5,"soft_fail":false,"target_va":5411542880,"va":5379579069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":549,"record_offset":21824,"parent_record_offset":19532,"ancestor_record_offset":19532,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":22848,"parent_record_offset":19532,"ancestor_record_offset":19532,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":549,"record_offset":23044,"parent_record_offset":19532,"ancestor_record_offset":19532,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::impl$3::as_shader_def_value
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":549,"record_offset":22392,"parent_record_offset":19532,"ancestor_record_offset":19532,"inlinee":27076,"source_line":75,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":549,"record_offset":19720,"parent_record_offset":19532,"ancestor_record_offset":19532,"inlinee":6435,"source_line":55,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":549,"record_offset":20980,"parent_record_offset":19532,"ancestor_record_offset":19532,"inlinee":15206,"source_line":63,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":20144,"parent_record_offset":20120,"ancestor_record_offset":19532,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":20168,"parent_record_offset":20144,"ancestor_record_offset":19532,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":20924,"parent_record_offset":20900,"ancestor_record_offset":19532,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":20120,"parent_record_offset":20092,"ancestor_record_offset":19532,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":549,"record_offset":20900,"parent_record_offset":20876,"ancestor_record_offset":19532,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":22924,"parent_record_offset":22892,"ancestor_record_offset":19532,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":20032,"parent_record_offset":20004,"ancestor_record_offset":19532,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":22732,"parent_record_offset":22704,"ancestor_record_offset":19532,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":20876,"parent_record_offset":20852,"ancestor_record_offset":19532,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":22972,"parent_record_offset":22948,"ancestor_record_offset":19532,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":22948,"parent_record_offset":22924,"ancestor_record_offset":19532,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":20092,"parent_record_offset":20060,"ancestor_record_offset":19532,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":20060,"parent_record_offset":20032,"ancestor_record_offset":19532,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":549,"record_offset":20852,"parent_record_offset":20828,"ancestor_record_offset":19532,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":549,"record_offset":19968,"parent_record_offset":19940,"ancestor_record_offset":19532,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":549,"record_offset":19940,"parent_record_offset":19912,"ancestor_record_offset":19532,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":549,"record_offset":19884,"parent_record_offset":19856,"ancestor_record_offset":19532,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":549,"record_offset":19912,"parent_record_offset":19884,"ancestor_record_offset":19532,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":549,"record_offset":19856,"parent_record_offset":19828,"ancestor_record_offset":19532,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":549,"record_offset":19828,"parent_record_offset":19800,"ancestor_record_offset":19532,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":549,"record_offset":22516,"parent_record_offset":22464,"ancestor_record_offset":19532,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":549,"record_offset":22428,"parent_record_offset":22392,"ancestor_record_offset":19532,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":549,"record_offset":22464,"parent_record_offset":22428,"ancestor_record_offset":19532,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":549,"record_offset":19800,"parent_record_offset":19772,"ancestor_record_offset":19532,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":22892,"parent_record_offset":22848,"ancestor_record_offset":19532,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":549,"record_offset":20004,"parent_record_offset":19968,"ancestor_record_offset":19532,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":549,"record_offset":19772,"parent_record_offset":19720,"ancestor_record_offset":19532,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":549,"record_offset":22544,"parent_record_offset":22464,"ancestor_record_offset":19532,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":549,"record_offset":20244,"parent_record_offset":20220,"ancestor_record_offset":19532,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":549,"record_offset":21052,"parent_record_offset":21024,"ancestor_record_offset":19532,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":549,"record_offset":21024,"parent_record_offset":20980,"ancestor_record_offset":19532,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":549,"record_offset":20220,"parent_record_offset":19968,"ancestor_record_offset":19532,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":549,"record_offset":20780,"parent_record_offset":19720,"ancestor_record_offset":19532,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":549,"record_offset":23012,"parent_record_offset":22848,"ancestor_record_offset":19532,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":549,"record_offset":22572,"parent_record_offset":22464,"ancestor_record_offset":19532,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00549.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::impl$8::as_compute_pipeline
// 记录形式: compiled_procedure; instances=1
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":495,"record_offset":89208,"rva":9216448,"va":5377925568}
// 编译实例 RVAs: 0x8ca1c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ed460
//     owner=unresolved_namespace; count=2
//     target_aliases=26 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3003071","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377925568,"raw_hex":"e8412dd2ff","size":5,"soft_fail":false,"target_va":5374923872,"va":5377926938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fe2e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2933599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377925568,"raw_hex":"e8a13cd3ff","size":5,"soft_fail":false,"target_va":5374993120,"va":5377926714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2844137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377925568,"raw_hex":"e8179ad4ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5377925620,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2844331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377925568,"raw_hex":"e85599d4ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5377925830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4914170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377925568,"raw_hex":"e8fafb4a00","size":5,"soft_fail":false,"target_va":5382840208,"va":5377926033,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t14539093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377925568,"raw_hex":"e855d9dd00","size":5,"soft_fail":false,"target_va":5392465200,"va":5377926102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t33614499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377925568,"raw_hex":"e8a3ea0002","size":5,"soft_fail":false,"target_va":5411540963,"va":5377926459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":495,"record_offset":91528,"parent_record_offset":89208,"ancestor_record_offset":89208,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":495,"record_offset":92208,"parent_record_offset":89208,"ancestor_record_offset":89208,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":495,"record_offset":94384,"parent_record_offset":89208,"ancestor_record_offset":89208,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::impl$4::as_shader_def_value
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":495,"record_offset":92096,"parent_record_offset":89208,"ancestor_record_offset":89208,"inlinee":27482,"source_line":81,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::impl$0::as_shader_def_value
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/rasterize_terrain_stroke.rs
//     evidence={"module":495,"record_offset":92972,"parent_record_offset":89208,"ancestor_record_offset":89208,"inlinee":27484,"source_line":43,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":495,"record_offset":89424,"parent_record_offset":89208,"ancestor_record_offset":89208,"inlinee":6435,"source_line":55,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":495,"record_offset":90684,"parent_record_offset":89208,"ancestor_record_offset":89208,"inlinee":15206,"source_line":63,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":495,"record_offset":89848,"parent_record_offset":89824,"ancestor_record_offset":89208,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":495,"record_offset":89872,"parent_record_offset":89848,"ancestor_record_offset":89208,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":495,"record_offset":90628,"parent_record_offset":90604,"ancestor_record_offset":89208,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":495,"record_offset":89824,"parent_record_offset":89796,"ancestor_record_offset":89208,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":495,"record_offset":90604,"parent_record_offset":90580,"ancestor_record_offset":89208,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":495,"record_offset":92120,"parent_record_offset":92096,"ancestor_record_offset":89208,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":495,"record_offset":92168,"parent_record_offset":92144,"ancestor_record_offset":89208,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":495,"record_offset":92284,"parent_record_offset":92252,"ancestor_record_offset":89208,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":495,"record_offset":89736,"parent_record_offset":89708,"ancestor_record_offset":89208,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":495,"record_offset":90580,"parent_record_offset":90556,"ancestor_record_offset":89208,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":495,"record_offset":92332,"parent_record_offset":92308,"ancestor_record_offset":89208,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":495,"record_offset":92308,"parent_record_offset":92284,"ancestor_record_offset":89208,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":495,"record_offset":89796,"parent_record_offset":89764,"ancestor_record_offset":89208,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":495,"record_offset":89764,"parent_record_offset":89736,"ancestor_record_offset":89208,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":495,"record_offset":90556,"parent_record_offset":90532,"ancestor_record_offset":89208,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":495,"record_offset":89672,"parent_record_offset":89644,"ancestor_record_offset":89208,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":495,"record_offset":89644,"parent_record_offset":89616,"ancestor_record_offset":89208,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":495,"record_offset":89588,"parent_record_offset":89560,"ancestor_record_offset":89208,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":495,"record_offset":89616,"parent_record_offset":89588,"ancestor_record_offset":89208,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":495,"record_offset":89560,"parent_record_offset":89532,"ancestor_record_offset":89208,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":495,"record_offset":89532,"parent_record_offset":89504,"ancestor_record_offset":89208,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":495,"record_offset":89504,"parent_record_offset":89476,"ancestor_record_offset":89208,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":495,"record_offset":92252,"parent_record_offset":92208,"ancestor_record_offset":89208,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":495,"record_offset":89708,"parent_record_offset":89672,"ancestor_record_offset":89208,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":495,"record_offset":89476,"parent_record_offset":89424,"ancestor_record_offset":89208,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":495,"record_offset":89948,"parent_record_offset":89924,"ancestor_record_offset":89208,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":495,"record_offset":90756,"parent_record_offset":90728,"ancestor_record_offset":89208,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":495,"record_offset":90728,"parent_record_offset":90684,"ancestor_record_offset":89208,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":495,"record_offset":89924,"parent_record_offset":89672,"ancestor_record_offset":89208,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":495,"record_offset":90484,"parent_record_offset":89424,"ancestor_record_offset":89208,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":495,"record_offset":92372,"parent_record_offset":92208,"ancestor_record_offset":89208,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::compute_passes::taa::impl$1::as_compute_pipeline
// 记录形式: compiled_procedure; instances=1
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":544,"record_offset":94200,"rva":10739968,"va":5379449088}
// 编译实例 RVAs: 0xa3e100
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ed460
//     owner=unresolved_namespace; count=2
//     target_aliases=26 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4526818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379449088,"raw_hex":"e81eedbaff","size":5,"soft_fail":false,"target_va":5374923872,"va":5379450685,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fe2e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4457251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379449088,"raw_hex":"e8ddfcbbff","size":5,"soft_fail":false,"target_va":5374993120,"va":5379450366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4367664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379449088,"raw_hex":"e8d05abdff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379449147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4367868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379449088,"raw_hex":"e8045abdff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379449367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t13015522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379449088,"raw_hex":"e8e299c600","size":5,"soft_fail":false,"target_va":5392465200,"va":5379449673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t32090898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379449088,"raw_hex":"e812abe901","size":5,"soft_fail":false,"target_va":5411540963,"va":5379450060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$::fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t3469957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379449088,"raw_hex":"e885f23400","size":5,"soft_fail":false,"target_va":5382919552,"va":5379449590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32092691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379449088,"raw_hex":"e813b2e901","size":5,"soft_fail":false,"target_va":5411542880,"va":5379450184,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":544,"record_offset":96492,"parent_record_offset":94200,"ancestor_record_offset":94200,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":544,"record_offset":97516,"parent_record_offset":94200,"ancestor_record_offset":94200,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":544,"record_offset":98932,"parent_record_offset":94200,"ancestor_record_offset":94200,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::impl$3::as_shader_def_value
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":544,"record_offset":97060,"parent_record_offset":94200,"ancestor_record_offset":94200,"inlinee":27076,"source_line":75,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":544,"record_offset":94388,"parent_record_offset":94200,"ancestor_record_offset":94200,"inlinee":6435,"source_line":55,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":544,"record_offset":95648,"parent_record_offset":94200,"ancestor_record_offset":94200,"inlinee":15206,"source_line":63,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":544,"record_offset":94812,"parent_record_offset":94788,"ancestor_record_offset":94200,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":544,"record_offset":94836,"parent_record_offset":94812,"ancestor_record_offset":94200,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":544,"record_offset":95592,"parent_record_offset":95568,"ancestor_record_offset":94200,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":544,"record_offset":94788,"parent_record_offset":94760,"ancestor_record_offset":94200,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":544,"record_offset":95568,"parent_record_offset":95544,"ancestor_record_offset":94200,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":97592,"parent_record_offset":97560,"ancestor_record_offset":94200,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":94700,"parent_record_offset":94672,"ancestor_record_offset":94200,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":97400,"parent_record_offset":97372,"ancestor_record_offset":94200,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":95544,"parent_record_offset":95520,"ancestor_record_offset":94200,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":97640,"parent_record_offset":97616,"ancestor_record_offset":94200,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":97616,"parent_record_offset":97592,"ancestor_record_offset":94200,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":94760,"parent_record_offset":94728,"ancestor_record_offset":94200,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":94728,"parent_record_offset":94700,"ancestor_record_offset":94200,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":544,"record_offset":95520,"parent_record_offset":95496,"ancestor_record_offset":94200,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":544,"record_offset":94636,"parent_record_offset":94608,"ancestor_record_offset":94200,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":544,"record_offset":94608,"parent_record_offset":94580,"ancestor_record_offset":94200,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":544,"record_offset":94552,"parent_record_offset":94524,"ancestor_record_offset":94200,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":544,"record_offset":94580,"parent_record_offset":94552,"ancestor_record_offset":94200,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":544,"record_offset":94524,"parent_record_offset":94496,"ancestor_record_offset":94200,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":544,"record_offset":94496,"parent_record_offset":94468,"ancestor_record_offset":94200,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":544,"record_offset":97184,"parent_record_offset":97132,"ancestor_record_offset":94200,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":544,"record_offset":97096,"parent_record_offset":97060,"ancestor_record_offset":94200,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":544,"record_offset":97132,"parent_record_offset":97096,"ancestor_record_offset":94200,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":544,"record_offset":94468,"parent_record_offset":94440,"ancestor_record_offset":94200,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":544,"record_offset":97560,"parent_record_offset":97516,"ancestor_record_offset":94200,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":544,"record_offset":94672,"parent_record_offset":94636,"ancestor_record_offset":94200,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":544,"record_offset":94440,"parent_record_offset":94388,"ancestor_record_offset":94200,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":544,"record_offset":97212,"parent_record_offset":97132,"ancestor_record_offset":94200,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":544,"record_offset":94912,"parent_record_offset":94888,"ancestor_record_offset":94200,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":544,"record_offset":95720,"parent_record_offset":95692,"ancestor_record_offset":94200,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":544,"record_offset":95692,"parent_record_offset":95648,"ancestor_record_offset":94200,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":544,"record_offset":94888,"parent_record_offset":94636,"ancestor_record_offset":94200,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":544,"record_offset":95448,"parent_record_offset":94388,"ancestor_record_offset":94200,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":544,"record_offset":97680,"parent_record_offset":97516,"ancestor_record_offset":94200,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":544,"record_offset":97240,"parent_record_offset":97132,"ancestor_record_offset":94200,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00544.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::vermeer::impl$7::as_compute_pipeline
// 记录形式: compiled_procedure; instances=1
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":487,"record_offset":75468,"rva":9087856,"va":5377796976}
// 编译实例 RVAs: 0x8aab70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ed460
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2874223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377796976,"raw_hex":"e89124d4ff","size":5,"soft_fail":false,"target_va":5374923872,"va":5377798090,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fe2e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2804783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377796976,"raw_hex":"e8d133d5ff","size":5,"soft_fail":false,"target_va":5374993120,"va":5377797898,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2715548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377796976,"raw_hex":"e86490d6ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5377797031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shader_uri.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2715756","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377796976,"raw_hex":"e8948fd6ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5377797255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t14667633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377796976,"raw_hex":"e871cfdf00","size":5,"soft_fail":false,"target_va":5392465200,"va":5377797562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t33743277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377796976,"raw_hex":"e8ade10202","size":5,"soft_fail":false,"target_va":5411540963,"va":5377797681,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$::fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5122062","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377796976,"raw_hex":"e80e284e00","size":5,"soft_fail":false,"target_va":5382919552,"va":5377797485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33745075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377796976,"raw_hex":"e8b3e80202","size":5,"soft_fail":false,"target_va":5411542880,"va":5377797800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":487,"record_offset":77748,"parent_record_offset":75468,"ancestor_record_offset":75468,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":487,"record_offset":78772,"parent_record_offset":75468,"ancestor_record_offset":75468,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":487,"record_offset":78968,"parent_record_offset":75468,"ancestor_record_offset":75468,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::shader_uri::impl$3::as_shader_def_value
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":487,"record_offset":78316,"parent_record_offset":75468,"ancestor_record_offset":75468,"inlinee":27076,"source_line":75,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":487,"record_offset":75644,"parent_record_offset":75468,"ancestor_record_offset":75468,"inlinee":6435,"source_line":55,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":487,"record_offset":76904,"parent_record_offset":75468,"ancestor_record_offset":75468,"inlinee":15206,"source_line":63,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":487,"record_offset":76068,"parent_record_offset":76044,"ancestor_record_offset":75468,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":487,"record_offset":76092,"parent_record_offset":76068,"ancestor_record_offset":75468,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":487,"record_offset":76848,"parent_record_offset":76824,"ancestor_record_offset":75468,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":487,"record_offset":76044,"parent_record_offset":76016,"ancestor_record_offset":75468,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":487,"record_offset":76824,"parent_record_offset":76800,"ancestor_record_offset":75468,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":487,"record_offset":78848,"parent_record_offset":78816,"ancestor_record_offset":75468,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":487,"record_offset":75956,"parent_record_offset":75928,"ancestor_record_offset":75468,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":487,"record_offset":78656,"parent_record_offset":78628,"ancestor_record_offset":75468,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":487,"record_offset":76800,"parent_record_offset":76776,"ancestor_record_offset":75468,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":487,"record_offset":78896,"parent_record_offset":78872,"ancestor_record_offset":75468,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":487,"record_offset":78872,"parent_record_offset":78848,"ancestor_record_offset":75468,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":487,"record_offset":76016,"parent_record_offset":75984,"ancestor_record_offset":75468,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":487,"record_offset":75984,"parent_record_offset":75956,"ancestor_record_offset":75468,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":487,"record_offset":76776,"parent_record_offset":76752,"ancestor_record_offset":75468,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":487,"record_offset":75892,"parent_record_offset":75864,"ancestor_record_offset":75468,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":487,"record_offset":75864,"parent_record_offset":75836,"ancestor_record_offset":75468,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":487,"record_offset":75808,"parent_record_offset":75780,"ancestor_record_offset":75468,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":487,"record_offset":75836,"parent_record_offset":75808,"ancestor_record_offset":75468,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":487,"record_offset":75780,"parent_record_offset":75752,"ancestor_record_offset":75468,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":487,"record_offset":75752,"parent_record_offset":75724,"ancestor_record_offset":75468,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":487,"record_offset":78440,"parent_record_offset":78388,"ancestor_record_offset":75468,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":487,"record_offset":78352,"parent_record_offset":78316,"ancestor_record_offset":75468,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":487,"record_offset":78388,"parent_record_offset":78352,"ancestor_record_offset":75468,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":487,"record_offset":75724,"parent_record_offset":75696,"ancestor_record_offset":75468,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":487,"record_offset":78816,"parent_record_offset":78772,"ancestor_record_offset":75468,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":487,"record_offset":75928,"parent_record_offset":75892,"ancestor_record_offset":75468,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":487,"record_offset":75696,"parent_record_offset":75644,"ancestor_record_offset":75468,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":487,"record_offset":78468,"parent_record_offset":78388,"ancestor_record_offset":75468,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":487,"record_offset":76168,"parent_record_offset":76144,"ancestor_record_offset":75468,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":487,"record_offset":76976,"parent_record_offset":76948,"ancestor_record_offset":75468,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":487,"record_offset":76948,"parent_record_offset":76904,"ancestor_record_offset":75468,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":487,"record_offset":76144,"parent_record_offset":75892,"ancestor_record_offset":75468,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":487,"record_offset":76704,"parent_record_offset":75644,"ancestor_record_offset":75468,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":487,"record_offset":78936,"parent_record_offset":78772,"ancestor_record_offset":75468,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":487,"record_offset":78496,"parent_record_offset":78388,"ancestor_record_offset":75468,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::shader_uri::impl$18::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 214
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":509,"record_offset":72156,"rva":9671712,"va":5378380832}
// 编译实例 RVAs: 0x939420
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field1_finish
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4511625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378380832,"raw_hex":"e889d74400","size":5,"soft_fail":false,"target_va":5382892528,"va":5378380898,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::shader_uri::impl$5::hash
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 223
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x69C8
// 首条位置: {"module":509,"inlinee":27080}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::pass_builder::shader_uri::impl$7::eq
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 230
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x69A7
// 首条位置: {"module":487,"inlinee":27047}
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
// 记录形式: inline_instance, inlinee_declaration; instances=56
// 源码行: 237
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1921
// 首条位置: {"module":15,"inlinee":6433}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "dyn$<country_core::render::pass_builder::shader_uri::AsComputeShaderSource>", "constructor": "dyn$", "arguments": ["country_core::render::pass_builder::shader_uri::AsComputeShaderSource"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<dyn$<country_core::render::pass_builder::shader_uri::AsComputeShaderSource>,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["dyn$<country_core::render::pass_builder::shader_uri::AsComputeShaderSource>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::boxed::Box<dyn$<country_core::render::pass_builder::shader_uri::AsComputeShaderSource>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::boxed::Box<dyn$<country_core::render::pass_builder::shader_uri::AsComputeShaderSource>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<dyn$<country_core::render::pass_builder::shader_uri::AsComputeShaderSource> >", "constructor": "ref$", "arguments": ["dyn$<country_core::render::pass_builder::shader_uri::AsComputeShaderSource>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::create_cull_and_bucket_pass<ref$<dyn$<country_core::render::pass_builder::shader_uri::AsComputeShaderSource> > >", "constructor": "country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::create_cull_and_bucket_pass", "arguments": ["ref$<dyn$<country_core::render::pass_builder::shader_uri::AsComputeShaderSource> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<dyn$<country_core::render::pass_builder::shader_uri::AsComputeShaderSource> > >", "constructor": "country_core::render::pass_builder::compute_pass_builder::ComputePass::new", "arguments": ["ref$<dyn$<country_core::render::pass_builder::shader_uri::AsComputeShaderSource> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash,country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram> >", "constructor": "tuple$", "arguments": ["country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash", "country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "nohash_hasher::NoHashHasher<country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash>", "constructor": "nohash_hasher::NoHashHasher", "arguments": ["country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash> >", "constructor": "core::hash::BuildHasherDefault", "arguments": ["nohash_hasher::NoHashHasher<country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash,country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash> > >", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash", "country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>", "core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash,country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash,country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash> > > >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash,country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash,country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 8; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustClosed>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>", "bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustClosed>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustClosed>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] asset::prefab::PrefabName (*)(ref$<asset::prefab::PrefabName>)
// 候选参数类型: ["ref$<asset::prefab::PrefabName>"]
// [文件级接口候选，具体函数归属 unknown] country_core::resources::walls::decorator::DecoratorId (*)(ref$<country_core::resources::walls::decorator::DecoratorId>)
// 候选参数类型: ["ref$<country_core::resources::walls::decorator::DecoratorId>"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::read_mouse_depth_value::MouseDepthValue>,bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::read_mouse_depth_value::MouseDepthValue>", "bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::read_mouse_depth_value::MouseDepthValue>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>

// ------------------------------------------------------------------
// 函数: country_core::render::nani::cull_shaders::impl$10::as_compute_pipeline
// 信息 ID: bf8160ca7b6b1538cfd71c85
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::render::nani::cull_shaders::NaniCullAndBucketMain> @ 0x15ff4b0 -> 0x1623120 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::cull_shaders::impl$10::source_hash
// 信息 ID: 0bd95bf45d4f9de74c1215e4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::render::nani::cull_shaders::NaniCullAndBucketMain>; module=1632, inline_record=48824; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::cull_shaders::impl$14::as_compute_pipeline
// 信息 ID: 9b0e870f368a36a699946ec6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::render::nani::cull_shaders::NaniCullAndBucketRoofTile> @ 0x1603d10 -> 0x1623590 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::cull_shaders::impl$14::source_hash
// 信息 ID: 3b08da55ee8a02c57390e768
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::render::nani::cull_shaders::NaniCullAndBucketRoofTile>; module=1632, inline_record=72260; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::cull_shaders::impl$18::as_compute_pipeline
// 信息 ID: 2ea52b9bfcf206ce43a0fe5e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::render::nani::cull_shaders::NaniCullAndBucketFlowers> @ 0x15fd070 -> 0x1623a10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::cull_shaders::impl$18::source_hash
// 信息 ID: 9b05525da736494d06bb4cdf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::render::nani::cull_shaders::NaniCullAndBucketFlowers>; module=1632, inline_record=37096; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::cull_shaders::impl$22::as_compute_pipeline
// 信息 ID: c6bcaa1e751842f9fc98e57f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::render::nani::cull_shaders::NaniCullAndBucketBush> @ 0x16018d0 -> 0x1623e90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::cull_shaders::impl$22::source_hash
// 信息 ID: 64c3aafa496b3162c873c350
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::render::nani::cull_shaders::NaniCullAndBucketBush>; module=1632, inline_record=60564; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::cull_shaders::impl$26::as_compute_pipeline
// 信息 ID: 7634021f5da22e8f2cacf3ee
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::render::nani::cull_shaders::NaniCullAndBucketWoodenPlank> @ 0x16021e0 -> 0x1624300 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::cull_shaders::impl$26::source_hash
// 信息 ID: 641966c20d3b91222e4108ba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::render::nani::cull_shaders::NaniCullAndBucketWoodenPlank>; module=1632, inline_record=63500; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::cull_shaders::impl$30::as_compute_pipeline
// 信息 ID: d26713582d349c682f559d10
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::cull_shaders::impl$30::source_hash
// 信息 ID: 0c34e9e82ad6fbb9acfbfaa4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::cull_shaders::impl$34::as_compute_pipeline
// 信息 ID: 5743927c446b15d69719acb8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::cull_shaders::impl$34::source_hash
// 信息 ID: ecc34aaa9b1139665c0c1e2e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::cull_shaders::impl$38::as_compute_pipeline
// 信息 ID: 87b0c41e32573d356543a19c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::cull_shaders::impl$38::source_hash
// 信息 ID: 51fbad8314825056f5bb6b40
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::cull_shaders::impl$42::as_compute_pipeline
// 信息 ID: 942a3ba37a628e16593bc1a0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::cull_shaders::impl$42::source_hash
// 信息 ID: 428b2303a76f42fc898d5b82
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash::compute
// 信息 ID: cd5466571c0ec44a4c14eb01
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 22 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<alloc::string::String>; module=238, inline_record=680; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >; module=238, inline_record=3144; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::shader_uri::impl$1::source_hash<ref$<str$> >; module=487, inline_record=33852; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::dof::impl$3::source_hash; module=549, inline_record=15596; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::dof::impl$7::source_hash; module=549, inline_record=19500; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::dof::impl$11::source_hash; module=549, inline_record=23408; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::render::nani::cull_shaders::NaniCullAndBucketFlowers>; module=1632, inline_record=37120; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::dof::HalfResAllDof>; module=1632, inline_record=40056; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::shader_uri::ShaderUri::parse
// 信息 ID: cfc116d4c5273d0111674e67
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::shader_uri::impl$1::as_compute_pipeline<alloc::string::String> @ 0x56d61 -> 0x968960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::shader_uri::impl$1::as_compute_pipeline<ref$<str$> > @ 0x5cf120 -> 0x968960 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::shader_uri::impl$1::as_compute_pipeline<ref$<str$> > @ 0x8a4360 -> 0x968960 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::shader_uri::impl$18::fmt
// 信息 ID: b648cf5a091a843ff910a797
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::shader_uri::impl$1::as_compute_pipeline<alloc::string::String>
// 信息 ID: 96e904fad4c2398ac5ac3b15
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::render::pass_builder::shader_uri::impl$1::as_compute_pipeline -> ["alloc::string::String"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<alloc::string::String> @ 0x5877ad -> 0x56d30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::shader_uri::impl$1::as_compute_pipeline<ref$<str$> >
// 信息 ID: eddef1e5a1db73410a504c21
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] country_core::render::pass_builder::shader_uri::impl$1::as_compute_pipeline -> ["ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> > @ 0x587fbc -> 0x5cf0f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> > @ 0x1603403 -> 0x8a4330 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::shader_uri::impl$1::source_hash
// 信息 ID: 76bcbecdd3e24fd37ef56ce1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<alloc::string::String>; module=238, inline_record=656; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >; module=238, inline_record=3120; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >; module=1632, inline_record=69328; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::shader_uri::impl$1::source_hash<ref$<str$> >
// 信息 ID: e2d4b3ddf2131de33c608dca
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] country_core::render::pass_builder::shader_uri::impl$1::source_hash -> ["ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::shader_uri::impl$2::as_compute_pipeline
// 信息 ID: cd56360826184580b55467be
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<dyn$<country_core::render::pass_builder::shader_uri::AsComputeShaderSource> > >; module=1632, inline_record=55628; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::shader_uri::impl$2::source_hash
// 信息 ID: 89e467a1f1ca566c14bec979
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<dyn$<country_core::render::pass_builder::shader_uri::AsComputeShaderSource> > >; module=1632, inline_record=54704; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::shader_uri::impl$3::as_shader_def_value
// 信息 ID: 6428bee846381461cb924222
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 18 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::vermeer::impl$7::as_compute_pipeline; module=487, inline_record=78316; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::bin_lights::impl$2::as_compute_pipeline; module=534, inline_record=56568; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::taa::impl$1::as_compute_pipeline; module=544, inline_record=97060; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::taa::impl$1::as_compute_pipeline; module=544, inline_record=98280; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::dof::impl$3::as_compute_pipeline; module=549, inline_record=18488; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::dof::impl$7::as_compute_pipeline; module=549, inline_record=22392; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::dof::impl$11::as_compute_pipeline; module=549, inline_record=26304; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::dof::impl$15::as_compute_pipeline; module=549, inline_record=30020; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::shader_uri::impl$4::as_shader_def_value
// 信息 ID: f74c069aa31ef2b59fa213b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::impl$8::as_compute_pipeline; module=495, inline_record=92096; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::shader_uri::impl$5::hash
// 信息 ID: 0cdc7294d490b7344c12970b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_compute_by_name; module=509, inline_record=56588; 不是独立调用边。
// [PDB 内联宿主] hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash,country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash,country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash> > > >; module=599, inline_record=330864; 不是独立调用边。
// [PDB 内联宿主] hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash,country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash,country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash> > > >; module=599, inline_record=333316; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::pass_builder::shader_uri::impl$7::eq
// 信息 ID: 71fc581288c64d3aa605a40f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::asset_libraries::shader_library::AssetShaderLibrary::add_compute; module=509, inline_record=54828; 不是独立调用边。
// [PDB 内联宿主] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_compute_by_name; module=509, inline_record=57276; 不是独立调用边。
// [PDB 内联宿主] hashbrown::map::HashMap::insert<country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash,country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::render::pass_builder::shader_uri::ComputeShaderSourceHash> >,alloc::alloc::Global>; module=1613, inline_record=157876; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::bin_lights::impl$2::as_compute_pipeline
// 信息 ID: 02e56737ac3f638bd5d14d48
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::bin_lights::BinLights> @ 0x15fe290 -> 0x9e8db0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::bin_lights::impl$2::source_hash
// 信息 ID: 6de8fad3542f0afae49780a6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::bin_lights::BinLights>; module=1632, inline_record=42972; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::dof::impl$11::as_compute_pipeline
// 信息 ID: 203b7fb1990032c3718f7d86
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::dof::impl$11::source_hash
// 信息 ID: fcb2d9faebd10565738fe5a5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::dof::impl$15::as_compute_pipeline
// 信息 ID: 5f71cf5cf861b228a2271204
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::dof::HalfResDof> @ 0x15ffdc0 -> 0xa5e290 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::dof::impl$15::source_hash
// 信息 ID: e95cb0d8a90bd763d3f6b656
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::dof::HalfResDof>; module=1632, inline_record=51756; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::dof::impl$19::as_compute_pipeline
// 信息 ID: b395ef34f062d04e29a03837
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::dof::HalfResAllDof> @ 0x15fd980 -> 0xa5e700 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::dof::impl$19::source_hash
// 信息 ID: 7904d78071c38c678b7f4303
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::dof::HalfResAllDof>; module=1632, inline_record=40032; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::dof::impl$3::as_compute_pipeline
// 信息 ID: 3554ea888e0a360602f7a75e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::dof::impl$3::source_hash
// 信息 ID: 2d43c8554e8262e97ca695f0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::dof::impl$7::as_compute_pipeline
// 信息 ID: 9b04902418a7b6b6efdf5dcb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::dof::impl$7::source_hash
// 信息 ID: e30527310db9b007b3680b9c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::impl$8::as_compute_pipeline
// 信息 ID: 3c30dfb68a42256e3613a482
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::GaussBlur> @ 0x1602aec -> 0x8ca1c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::impl$8::source_hash
// 信息 ID: 1db7dd1092d600e5912a2ba7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::GaussBlur>; module=1632, inline_record=66452; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::impl$9::hash
// 信息 ID: 544d7072718c78f7075da6a7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::calculate_hash<tuple$<ref$<str$>,ref$<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::GaussBlur> > >; module=1606, inline_record=5128; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::taa::impl$1::as_compute_pipeline
// 信息 ID: 21b5cd6768e2575836b4a59c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::taa::TgsrMain> @ 0x1600fcc -> 0xa3e100 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::taa::impl$1::source_hash
// 信息 ID: 6038dc7d9162bf210da3ecf3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::taa::TgsrMain>; module=1632, inline_record=57636; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::compute_passes::taa::impl$2::hash
// 信息 ID: e366fda7e984f4d15979986c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::calculate_hash<tuple$<ref$<str$>,ref$<country_core::systems::render_loop::compute_passes::taa::TgsrMain> > >; module=1606, inline_record=23868; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::vermeer::impl$7::as_compute_pipeline
// 信息 ID: c5e2f8d6a4ec6bcd41fcc6d8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::vermeer::GenerateStrokes> @ 0x15feba0 -> 0x8aab70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::vermeer::impl$7::source_hash
// 信息 ID: 292b888624abc9f384793b88
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::vermeer::GenerateStrokes>; module=1632, inline_record=45896; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::vermeer::impl$8::hash
// 信息 ID: c9e0ae68845ae58153adb09a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::calculate_hash<tuple$<ref$<str$>,ref$<country_core::systems::render_loop::vermeer::GenerateStrokes> > >; module=1606, inline_record=3072; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
