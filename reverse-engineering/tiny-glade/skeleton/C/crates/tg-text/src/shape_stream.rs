// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tg-text/src/shape_stream.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: afe60081d5710b8f344f166179988f9ac3a194b5dd7706430e5cb1099ac58b3e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: shape_stream.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 27, 246, 347, 356, 633, 1267, 1289, 2874, 2887, 4228, 4229
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use ron; // namespace candidate
//   package identities: [{"name":"ron","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.0"},{"name":"ron","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.1"}]
// use serde_core; // namespace candidate
//   package identities: [{"name":"serde_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use tg_text; // namespace candidate
//   package identities: [{"name":"tg-text","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::RawShapeStream::append
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":633,"record_offset":2580,"rva":13316192,"va":5382025312}
// 编译实例 RVAs: 0xcb3060
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34327506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382025312,"raw_hex":"e8d2cb0b02","size":5,"soft_fail":false,"target_va":5416353040,"va":5382025529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34595737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382025312,"raw_hex":"e899e30f02","size":5,"soft_fail":false,"target_va":5416621632,"va":5382025890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*48(%rsp)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5382025312,"raw_hex":"ff542430","size":4,"soft_fail":false,"va":5382025686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$6::try_into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":633,"record_offset":3244,"parent_record_offset":2580,"ancestor_record_offset":2580,"inlinee":6246,"source_line":818,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":633,"record_offset":2808,"parent_record_offset":2580,"ancestor_record_offset":2580,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":633,"record_offset":3300,"parent_record_offset":2580,"ancestor_record_offset":2580,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":633,"record_offset":2736,"parent_record_offset":2580,"ancestor_record_offset":2580,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_direct] tg_text::shape_stream::FillColor::with_color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/shape_stream.rs
//     evidence={"module":633,"record_offset":3336,"parent_record_offset":2580,"ancestor_record_offset":2580,"inlinee":39596,"source_line":185,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_direct] tg_text::shape_stream::Image::with_scaling
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/shape_stream.rs
//     evidence={"module":633,"record_offset":3924,"parent_record_offset":2580,"ancestor_record_offset":2580,"inlinee":39598,"source_line":193,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_direct] tg_text::shape_stream::RawShapeStream::encode
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/shape_stream.rs
//     evidence={"module":633,"record_offset":2836,"parent_record_offset":2580,"ancestor_record_offset":2580,"inlinee":39594,"source_line":206,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":633,"record_offset":3112,"parent_record_offset":3080,"ancestor_record_offset":2580,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":633,"record_offset":2984,"parent_record_offset":2956,"ancestor_record_offset":2580,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":633,"record_offset":3612,"parent_record_offset":3584,"ancestor_record_offset":2580,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":633,"record_offset":3040,"parent_record_offset":3012,"ancestor_record_offset":2580,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":633,"record_offset":3160,"parent_record_offset":3136,"ancestor_record_offset":2580,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":633,"record_offset":3136,"parent_record_offset":3112,"ancestor_record_offset":2580,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":633,"record_offset":3012,"parent_record_offset":2984,"ancestor_record_offset":2580,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":633,"record_offset":2920,"parent_record_offset":2892,"ancestor_record_offset":2580,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":633,"record_offset":3080,"parent_record_offset":2920,"ancestor_record_offset":2580,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":633,"record_offset":3200,"parent_record_offset":2920,"ancestor_record_offset":2580,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":633,"record_offset":2956,"parent_record_offset":2920,"ancestor_record_offset":2580,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$22::extend
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":633,"record_offset":2864,"parent_record_offset":2836,"ancestor_record_offset":2580,"inlinee":11091,"source_line":4166,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":633,"record_offset":2892,"parent_record_offset":2864,"ancestor_record_offset":2580,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$138::try_from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\num.rs
//     evidence={"module":633,"record_offset":3268,"parent_record_offset":3244,"ancestor_record_offset":2580,"inlinee":10438,"source_line":296,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":633,"record_offset":2768,"parent_record_offset":2736,"ancestor_record_offset":2580,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 8, 52, 86
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B49A
// 首条位置: {"module":2887,"inlinee":111770}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2887,"record_offset":104148,"parent_record_offset":104096,"ancestor_record_offset":104096,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":2887,"record_offset":104120,"parent_record_offset":104096,"ancestor_record_offset":104096,"inlinee":7615,"source_line":1139,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::impl$8::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 8
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x568F
// 首条位置: {"module":356,"inlinee":22159}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::ShapeStream::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":633,"record_offset":1992,"rva":12121296,"va":5380830416}
// 编译实例 RVAs: 0xb8f4d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::ShapeStream::fill_color
// 记录形式: compiled_procedure; instances=1
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":633,"record_offset":724,"rva":13315632,"va":5382024752}
// 编译实例 RVAs: 0xcb2e30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a5950
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12637562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382024752,"raw_hex":"e8862a3fff","size":5,"soft_fail":false,"target_va":5369387344,"va":5382024901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34325583","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382024752,"raw_hex":"e84fc40b02","size":5,"soft_fail":false,"target_va":5416350384,"va":5382024796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":633,"record_offset":856,"parent_record_offset":724,"ancestor_record_offset":724,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":633,"record_offset":948,"parent_record_offset":916,"ancestor_record_offset":724,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":633,"record_offset":996,"parent_record_offset":972,"ancestor_record_offset":724,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":633,"record_offset":972,"parent_record_offset":948,"ancestor_record_offset":724,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":633,"record_offset":916,"parent_record_offset":880,"ancestor_record_offset":724,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":633,"record_offset":880,"parent_record_offset":856,"ancestor_record_offset":724,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":633,"record_offset":1036,"parent_record_offset":880,"ancestor_record_offset":724,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::ShapeStream::image
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1C0C
// 首条位置: {"module":246,"inlinee":7180}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":246,"record_offset":476,"parent_record_offset":400,"ancestor_record_offset":400,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":246,"record_offset":448,"parent_record_offset":400,"ancestor_record_offset":400,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":246,"record_offset":912,"parent_record_offset":888,"ancestor_record_offset":400,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":246,"record_offset":936,"parent_record_offset":912,"ancestor_record_offset":400,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":246,"record_offset":960,"parent_record_offset":936,"ancestor_record_offset":400,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":246,"record_offset":888,"parent_record_offset":864,"ancestor_record_offset":400,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":580,"parent_record_offset":548,"ancestor_record_offset":400,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":864,"parent_record_offset":840,"ancestor_record_offset":400,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":628,"parent_record_offset":604,"ancestor_record_offset":400,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":604,"parent_record_offset":580,"ancestor_record_offset":400,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":246,"record_offset":840,"parent_record_offset":816,"ancestor_record_offset":400,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":246,"record_offset":548,"parent_record_offset":504,"ancestor_record_offset":400,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":246,"record_offset":504,"parent_record_offset":476,"ancestor_record_offset":400,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":246,"record_offset":696,"parent_record_offset":504,"ancestor_record_offset":400,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":246,"record_offset":668,"parent_record_offset":504,"ancestor_record_offset":400,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00246.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::ShapeStream::corner_radius
// 记录形式: compiled_procedure; instances=1
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":633,"record_offset":1076,"rva":13315808,"va":5382024928}
// 编译实例 RVAs: 0xcb2ee0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a5950
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12637742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382024928,"raw_hex":"e8d2293fff","size":5,"soft_fail":false,"target_va":5369387344,"va":5382025081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34325404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382024928,"raw_hex":"e89cc30b02","size":5,"soft_fail":false,"target_va":5416350384,"va":5382024975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":633,"record_offset":1212,"parent_record_offset":1076,"ancestor_record_offset":1076,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":633,"record_offset":1304,"parent_record_offset":1272,"ancestor_record_offset":1076,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":633,"record_offset":1352,"parent_record_offset":1328,"ancestor_record_offset":1076,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":633,"record_offset":1328,"parent_record_offset":1304,"ancestor_record_offset":1076,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":633,"record_offset":1272,"parent_record_offset":1236,"ancestor_record_offset":1076,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":633,"record_offset":1236,"parent_record_offset":1212,"ancestor_record_offset":1076,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":633,"record_offset":1392,"parent_record_offset":1236,"ancestor_record_offset":1076,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::ShapeStream::is_empty
// 记录形式: compiled_procedure; instances=1
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":633,"record_offset":2108,"rva":13316144,"va":5382025264}
// 编译实例 RVAs: 0xcb3030
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":633,"record_offset":2236,"parent_record_offset":2108,"ancestor_record_offset":2108,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::ShapeStream::encode_no_images
// 记录形式: compiled_procedure; instances=1
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":633,"record_offset":1432,"rva":13315984,"va":5382025104}
// 编译实例 RVAs: 0xcb2f90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6346539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382025104,"raw_hex":"e8d5289fff","size":5,"soft_fail":false,"target_va":5375678704,"va":5382025238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::shape_stream::RawShapeStream::append
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382025104,"raw_hex":"e88c000000","size":5,"soft_fail":false,"target_va":5382025312,"va":5382025167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":633,"record_offset":1684,"parent_record_offset":1432,"ancestor_record_offset":1432,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_direct] tg_text::shape_stream::RawShapeStream::clear
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/shape_stream.rs
//     evidence={"module":633,"record_offset":1628,"parent_record_offset":1432,"ancestor_record_offset":1432,"inlinee":39593,"source_line":165,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_direct] tg_text::shape_stream::RawShapeStream::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/shape_stream.rs
//     evidence={"module":633,"record_offset":1576,"parent_record_offset":1432,"ancestor_record_offset":1432,"inlinee":39590,"source_line":114,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":633,"record_offset":1880,"parent_record_offset":1856,"ancestor_record_offset":1432,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":633,"record_offset":1904,"parent_record_offset":1880,"ancestor_record_offset":1432,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":633,"record_offset":1928,"parent_record_offset":1904,"ancestor_record_offset":1432,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":633,"record_offset":1856,"parent_record_offset":1768,"ancestor_record_offset":1432,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":633,"record_offset":1796,"parent_record_offset":1768,"ancestor_record_offset":1432,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":633,"record_offset":1768,"parent_record_offset":1744,"ancestor_record_offset":1432,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":633,"record_offset":1744,"parent_record_offset":1724,"ancestor_record_offset":1432,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":633,"record_offset":1652,"parent_record_offset":1628,"ancestor_record_offset":1432,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":633,"record_offset":1824,"parent_record_offset":1796,"ancestor_record_offset":1432,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":633,"record_offset":1704,"parent_record_offset":1684,"ancestor_record_offset":1432,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] tg_text::shape_stream::impl$23::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/shape_stream.rs
//     evidence={"module":633,"record_offset":1600,"parent_record_offset":1576,"ancestor_record_offset":1432,"inlinee":39592,"source_line":96,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4228,"record_offset":660,"rva":43598256,"va":5412307376}
// 编译实例 RVAs: 0x29941b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde_core..de..Error$GT$::unknown_variant
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4248214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412307376,"raw_hex":"e896d24000","size":5,"soft_fail":false,"target_va":5416555792,"va":5412307573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4228,"record_offset":836,"parent_record_offset":660,"ancestor_record_offset":660,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/04228.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":4228,"record_offset":856,"parent_record_offset":836,"ancestor_record_offset":660,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/04228.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":4228,"record_offset":876,"parent_record_offset":856,"ancestor_record_offset":660,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/04228.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":4228,"record_offset":900,"parent_record_offset":876,"ancestor_record_offset":660,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/04228.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 52, 86
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B490
// 首条位置: {"module":2887,"inlinee":111760}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::impl$2::deserialize_identifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":2887,"record_offset":15036,"parent_record_offset":15012,"ancestor_record_offset":15012,"inlinee":9935,"source_line":759,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2887,"record_offset":17512,"parent_record_offset":17484,"ancestor_record_offset":17296,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2887,"record_offset":15072,"parent_record_offset":15036,"ancestor_record_offset":15012,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2887,"record_offset":15136,"parent_record_offset":15036,"ancestor_record_offset":15012,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2887,"record_offset":17540,"parent_record_offset":17512,"ancestor_record_offset":17296,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2887,"record_offset":17576,"parent_record_offset":17540,"ancestor_record_offset":17296,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2887,"record_offset":17484,"parent_record_offset":17440,"ancestor_record_offset":17296,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":2887,"record_offset":15108,"parent_record_offset":15036,"ancestor_record_offset":15012,"inlinee":7150,"source_line":1543,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] tg_text::shape_stream::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/shape_stream.rs
//     evidence={"module":2887,"record_offset":17440,"parent_record_offset":17408,"ancestor_record_offset":17296,"inlinee":111761,"source_line":86,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B49C
// 首条位置: {"module":2887,"inlinee":111772}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2887,"record_offset":79108,"parent_record_offset":79056,"ancestor_record_offset":79056,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":2887,"record_offset":79080,"parent_record_offset":79056,"ancestor_record_offset":79056,"inlinee":6479,"source_line":2056,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::_::impl$0::deserialize<ref_mut$<ron::de::Deserializer> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4228,"record_offset":1708,"rva":43598624,"va":5412307744}
// 编译实例 RVAs: 0x2994320
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14040c690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-39353583","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412307744,"raw_hex":"e81183a7fd","size":5,"soft_fail":false,"target_va":5372954256,"va":5412307834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406c6ac0
//     owner=unresolved_namespace; count=1
//     target_aliases=15 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-36493640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412307744,"raw_hex":"e8b826d3fd","size":5,"soft_fail":false,"target_va":5375814336,"va":5412307971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4749177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412307744,"raw_hex":"e879774800","size":5,"soft_fail":false,"target_va":5417057056,"va":5412307874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4312787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412307744,"raw_hex":"e8d3ce4100","size":5,"soft_fail":false,"target_va":5416620794,"va":5412308002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::string
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-29113948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412307744,"raw_hex":"e8a4c143fe","size":5,"soft_fail":false,"target_va":5383193824,"va":5412307767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4228,"record_offset":2408,"parent_record_offset":1708,"ancestor_record_offset":1708,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/04228.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":4228,"record_offset":1876,"parent_record_offset":1708,"ancestor_record_offset":1708,"inlinee":7665,"source_line":689,"archive":"_pdb/C/modules/04228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4228,"record_offset":2236,"parent_record_offset":2208,"ancestor_record_offset":1708,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/04228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4228,"record_offset":2264,"parent_record_offset":2236,"ancestor_record_offset":1708,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/04228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4228,"record_offset":2116,"parent_record_offset":2088,"ancestor_record_offset":1708,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/04228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4228,"record_offset":2144,"parent_record_offset":2116,"ancestor_record_offset":1708,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/04228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4228,"record_offset":2172,"parent_record_offset":2144,"ancestor_record_offset":1708,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/04228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":4228,"record_offset":2088,"parent_record_offset":2060,"ancestor_record_offset":1708,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/04228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":4228,"record_offset":2060,"parent_record_offset":2032,"ancestor_record_offset":1708,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/04228.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4228,"record_offset":2208,"parent_record_offset":2172,"ancestor_record_offset":1708,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/04228.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":4228,"record_offset":2436,"parent_record_offset":2408,"ancestor_record_offset":1708,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/04228.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":4228,"record_offset":2312,"parent_record_offset":2172,"ancestor_record_offset":1708,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/04228.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4228,"record_offset":2336,"parent_record_offset":2312,"ancestor_record_offset":1708,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/04228.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4228,"record_offset":1968,"parent_record_offset":1932,"ancestor_record_offset":1708,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/04228.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4228,"record_offset":1932,"parent_record_offset":1904,"ancestor_record_offset":1708,"inlinee":7149,"source_line":461,"archive":"_pdb/C/modules/04228.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":4228,"record_offset":1904,"parent_record_offset":1876,"ancestor_record_offset":1708,"inlinee":7148,"source_line":471,"archive":"_pdb/C/modules/04228.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":4228,"record_offset":2004,"parent_record_offset":1932,"ancestor_record_offset":1708,"inlinee":7150,"source_line":1543,"archive":"_pdb/C/modules/04228.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$6::visit_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":4228,"record_offset":2032,"parent_record_offset":2004,"ancestor_record_offset":1708,"inlinee":7668,"source_line":592,"archive":"_pdb/C/modules/04228.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::_::impl$0::deserialize<ref_mut$<ron::de::tag::Deserializer> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4228,"record_offset":1392,"rva":43598480,"va":5412307600}
// 编译实例 RVAs: 0x2994290
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1406c6ac0
//     owner=unresolved_namespace; count=1
//     target_aliases=15 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-36493329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412307600,"raw_hex":"e8ef27d3fd","size":5,"soft_fail":false,"target_va":5375814336,"va":5412307660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1423392d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6664152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412307600,"raw_hex":"e828509aff","size":5,"soft_fail":false,"target_va":5405643472,"va":5412307619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4228,"record_offset":1648,"parent_record_offset":1392,"ancestor_record_offset":1392,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/04228.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":4228,"record_offset":1564,"parent_record_offset":1392,"ancestor_record_offset":1392,"inlinee":7665,"source_line":689,"archive":"_pdb/C/modules/04228.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":4228,"record_offset":1676,"parent_record_offset":1648,"ancestor_record_offset":1392,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/04228.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/tag.rs
//     evidence={"module":4228,"record_offset":1612,"parent_record_offset":1588,"ancestor_record_offset":1392,"inlinee":5916,"source_line":18,"archive":"_pdb/C/modules/04228.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/tag.rs
//     evidence={"module":4228,"record_offset":1588,"parent_record_offset":1564,"ancestor_record_offset":1392,"inlinee":7156,"source_line":25,"archive":"_pdb/C/modules/04228.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::impl$20::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1140E
// 首条位置: {"module":1267,"inlinee":70670}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::borrow::impl$2::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\borrow.rs
//     evidence={"module":1267,"record_offset":11788,"parent_record_offset":11752,"ancestor_record_offset":11752,"inlinee":18893,"source_line":197,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1267,"record_offset":12228,"parent_record_offset":12204,"ancestor_record_offset":11752,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1267,"record_offset":12252,"parent_record_offset":12228,"ancestor_record_offset":11752,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1267,"record_offset":12276,"parent_record_offset":12252,"ancestor_record_offset":11752,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1267,"record_offset":12204,"parent_record_offset":12056,"ancestor_record_offset":11752,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1267,"record_offset":11996,"parent_record_offset":11968,"ancestor_record_offset":11752,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1267,"record_offset":12056,"parent_record_offset":12024,"ancestor_record_offset":11752,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1267,"record_offset":12024,"parent_record_offset":11996,"ancestor_record_offset":11752,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1267,"record_offset":12092,"parent_record_offset":12056,"ancestor_record_offset":11752,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1267,"record_offset":11876,"parent_record_offset":11848,"ancestor_record_offset":11752,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1267,"record_offset":11904,"parent_record_offset":11876,"ancestor_record_offset":11752,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1267,"record_offset":11932,"parent_record_offset":11904,"ancestor_record_offset":11752,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1267,"record_offset":11848,"parent_record_offset":11820,"ancestor_record_offset":11752,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1267,"record_offset":11820,"parent_record_offset":11788,"ancestor_record_offset":11752,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1267,"record_offset":11968,"parent_record_offset":11932,"ancestor_record_offset":11752,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1267,"record_offset":12116,"parent_record_offset":12092,"ancestor_record_offset":11752,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1267,"record_offset":12164,"parent_record_offset":12140,"ancestor_record_offset":11752,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1267,"record_offset":12140,"parent_record_offset":12116,"ancestor_record_offset":11752,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1267,"record_offset":12332,"parent_record_offset":11932,"ancestor_record_offset":11752,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1267,"record_offset":12356,"parent_record_offset":12332,"ancestor_record_offset":11752,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::impl$1::from
// 记录形式: compiled_procedure; instances=1
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":633,"record_offset":612,"rva":13315600,"va":5382024720}
// 编译实例 RVAs: 0xcb2e10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::impl$2::from
// 记录形式: compiled_procedure; instances=1
// 源码行: 75
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":633,"record_offset":4404,"rva":13316784,"va":5382025904}
// 编译实例 RVAs: 0xcb32b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::impl$3::from
// 记录形式: compiled_procedure; instances=1
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":633,"record_offset":500,"rva":7105216,"va":5375814336}
// 编译实例 RVAs: 0x6c6ac0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 86
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B491
// 首条位置: {"module":2887,"inlinee":111761}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2887,"record_offset":17484,"parent_record_offset":17440,"ancestor_record_offset":17440,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2887,"record_offset":17512,"parent_record_offset":17484,"ancestor_record_offset":17440,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2887,"record_offset":17540,"parent_record_offset":17512,"ancestor_record_offset":17440,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2887,"record_offset":17576,"parent_record_offset":17540,"ancestor_record_offset":17440,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 86
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B498
// 首条位置: {"module":2887,"inlinee":111768}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2887,"record_offset":21076,"parent_record_offset":21036,"ancestor_record_offset":21036,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2887,"record_offset":21100,"parent_record_offset":21076,"ancestor_record_offset":21036,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2887,"record_offset":21124,"parent_record_offset":21100,"ancestor_record_offset":21036,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2887,"record_offset":21152,"parent_record_offset":21124,"ancestor_record_offset":21036,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 86
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B497
// 首条位置: {"module":2887,"inlinee":111767}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::impl$2::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":2887,"record_offset":20932,"parent_record_offset":20904,"ancestor_record_offset":20904,"inlinee":9935,"source_line":759,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2887,"record_offset":21100,"parent_record_offset":21076,"ancestor_record_offset":20904,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2887,"record_offset":20972,"parent_record_offset":20932,"ancestor_record_offset":20904,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2887,"record_offset":21124,"parent_record_offset":21100,"ancestor_record_offset":20904,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2887,"record_offset":21152,"parent_record_offset":21124,"ancestor_record_offset":20904,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2887,"record_offset":21076,"parent_record_offset":21036,"ancestor_record_offset":20904,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":2887,"record_offset":21008,"parent_record_offset":20932,"ancestor_record_offset":20904,"inlinee":7150,"source_line":1543,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/shape_stream.rs
//     evidence={"module":2887,"record_offset":21036,"parent_record_offset":21008,"ancestor_record_offset":20904,"inlinee":111768,"source_line":86,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 86
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B495
// 首条位置: {"module":2887,"inlinee":111765}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":2887,"record_offset":21424,"parent_record_offset":20216,"ancestor_record_offset":20216,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2887,"record_offset":23232,"parent_record_offset":20216,"ancestor_record_offset":20216,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2887,"record_offset":21332,"parent_record_offset":20216,"ancestor_record_offset":20216,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2887,"record_offset":23856,"parent_record_offset":20216,"ancestor_record_offset":20216,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":2887,"record_offset":20264,"parent_record_offset":20216,"ancestor_record_offset":20216,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":2887,"record_offset":21368,"parent_record_offset":20216,"ancestor_record_offset":20216,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2887,"record_offset":22796,"parent_record_offset":22772,"ancestor_record_offset":20216,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2887,"record_offset":22820,"parent_record_offset":22796,"ancestor_record_offset":20216,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2887,"record_offset":22844,"parent_record_offset":22820,"ancestor_record_offset":20216,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2887,"record_offset":22772,"parent_record_offset":22744,"ancestor_record_offset":20216,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2887,"record_offset":22744,"parent_record_offset":22720,"ancestor_record_offset":20216,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2887,"record_offset":22720,"parent_record_offset":22696,"ancestor_record_offset":20216,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\any.rs
//     evidence={"module":2887,"record_offset":20480,"parent_record_offset":20456,"ancestor_record_offset":20216,"inlinee":4331,"source_line":744,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::eq::runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":2887,"record_offset":20508,"parent_record_offset":20480,"ancestor_record_offset":20216,"inlinee":4333,"source_line":2434,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":2887,"record_offset":21100,"parent_record_offset":21076,"ancestor_record_offset":20216,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2887,"record_offset":20548,"parent_record_offset":20304,"ancestor_record_offset":20216,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2887,"record_offset":20812,"parent_record_offset":20304,"ancestor_record_offset":20216,"inlinee":7142,"source_line":2300,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2887,"record_offset":22592,"parent_record_offset":22068,"ancestor_record_offset":20216,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2887,"record_offset":20780,"parent_record_offset":20304,"ancestor_record_offset":20216,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2887,"record_offset":20420,"parent_record_offset":20304,"ancestor_record_offset":20216,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2887,"record_offset":21124,"parent_record_offset":21100,"ancestor_record_offset":20216,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2887,"record_offset":21152,"parent_record_offset":21124,"ancestor_record_offset":20216,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":2887,"record_offset":21076,"parent_record_offset":21036,"ancestor_record_offset":20216,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::id::Deserializer::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/id.rs
//     evidence={"module":2887,"record_offset":20580,"parent_record_offset":20304,"ancestor_record_offset":20216,"inlinee":9971,"source_line":11,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":2887,"record_offset":20932,"parent_record_offset":20904,"ancestor_record_offset":20216,"inlinee":9935,"source_line":759,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":2887,"record_offset":20304,"parent_record_offset":20264,"ancestor_record_offset":20216,"inlinee":9956,"source_line":852,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_value_seed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":2887,"record_offset":21504,"parent_record_offset":21452,"ancestor_record_offset":20216,"inlinee":9908,"source_line":873,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::is_serde_content
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":2887,"record_offset":21664,"parent_record_offset":21504,"ancestor_record_offset":20216,"inlinee":9910,"source_line":1050,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::is_serde_tag_or_content
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/mod.rs
//     evidence={"module":2887,"record_offset":20456,"parent_record_offset":20304,"ancestor_record_offset":20216,"inlinee":9920,"source_line":1109,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::Deserializer::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.12.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ron-0.12.0/src/de/tag.rs
//     evidence={"module":2887,"record_offset":22428,"parent_record_offset":22068,"ancestor_record_offset":20216,"inlinee":10096,"source_line":10,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":2887,"record_offset":21008,"parent_record_offset":20932,"ancestor_record_offset":20216,"inlinee":7150,"source_line":1543,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":2887,"record_offset":20608,"parent_record_offset":20304,"ancestor_record_offset":20216,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] tg_text::shape_stream::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/shape_stream.rs
//     evidence={"module":2887,"record_offset":21812,"parent_record_offset":21788,"ancestor_record_offset":20216,"inlinee":111770,"source_line":8,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/shape_stream.rs
//     evidence={"module":2887,"record_offset":21036,"parent_record_offset":21008,"ancestor_record_offset":20216,"inlinee":111768,"source_line":86,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   [pdb_inline_transitive] tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/shape_stream.rs
//     evidence={"module":2887,"record_offset":20632,"parent_record_offset":20608,"ancestor_record_offset":20216,"inlinee":111767,"source_line":86,"archive":"_pdb/C/modules/02887.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum<ron::de::Enum>
// 记录形式: compiled_procedure; instances=1
// 源码行: 86
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4228,"record_offset":264,"rva":43597936,"va":5412307056}
// 编译实例 RVAs: 0x2994070
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142330cc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6697922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412307056,"raw_hex":"e83ecc99ff","size":5,"soft_fail":false,"target_va":5405609152,"va":5412307069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1423315a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6695703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412307056,"raw_hex":"e8e9d499ff","size":5,"soft_fail":false,"target_va":5405611424,"va":5412307122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1423327a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6691197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412307056,"raw_hex":"e883e699ff","size":5,"soft_fail":false,"target_va":5405616032,"va":5412307224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1423331f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_stream.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6688597","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5412307056,"raw_hex":"e8abf099ff","size":5,"soft_fail":false,"target_va":5405618672,"va":5412307264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4228,"record_offset":564,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/04228.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4228,"record_offset":472,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/04228.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":4228,"record_offset":508,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/04228.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::EnumAccess::variant
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":4228,"record_offset":444,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":6479,"source_line":2056,"archive":"_pdb/C/modules/04228.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::VariantAccess::newtype_variant
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":4228,"record_offset":536,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":6481,"source_line":2180,"archive":"_pdb/C/modules/04228.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::impl$22::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 86
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1140C
// 首条位置: {"module":1267,"inlinee":70668}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] tg_text::shape_stream::impl$20::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/shape_stream.rs
//     evidence={"module":1267,"record_offset":11752,"parent_record_offset":11712,"ancestor_record_offset":11712,"inlinee":70670,"source_line":61,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1267,"record_offset":12228,"parent_record_offset":12204,"ancestor_record_offset":11712,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1267,"record_offset":12252,"parent_record_offset":12228,"ancestor_record_offset":11712,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1267,"record_offset":12276,"parent_record_offset":12252,"ancestor_record_offset":11712,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1267,"record_offset":12204,"parent_record_offset":12056,"ancestor_record_offset":11712,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::impl$2::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\borrow.rs
//     evidence={"module":1267,"record_offset":11788,"parent_record_offset":11752,"ancestor_record_offset":11712,"inlinee":18893,"source_line":197,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1267,"record_offset":11996,"parent_record_offset":11968,"ancestor_record_offset":11712,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1267,"record_offset":12056,"parent_record_offset":12024,"ancestor_record_offset":11712,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1267,"record_offset":12024,"parent_record_offset":11996,"ancestor_record_offset":11712,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1267,"record_offset":12092,"parent_record_offset":12056,"ancestor_record_offset":11712,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1267,"record_offset":11876,"parent_record_offset":11848,"ancestor_record_offset":11712,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1267,"record_offset":11904,"parent_record_offset":11876,"ancestor_record_offset":11712,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1267,"record_offset":11932,"parent_record_offset":11904,"ancestor_record_offset":11712,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1267,"record_offset":11848,"parent_record_offset":11820,"ancestor_record_offset":11712,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1267,"record_offset":11820,"parent_record_offset":11788,"ancestor_record_offset":11712,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1267,"record_offset":11968,"parent_record_offset":11932,"ancestor_record_offset":11712,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1267,"record_offset":12116,"parent_record_offset":12092,"ancestor_record_offset":11712,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1267,"record_offset":12164,"parent_record_offset":12140,"ancestor_record_offset":11712,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1267,"record_offset":12140,"parent_record_offset":12116,"ancestor_record_offset":11712,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1267,"record_offset":12332,"parent_record_offset":11932,"ancestor_record_offset":11712,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1267,"record_offset":12356,"parent_record_offset":12332,"ancestor_record_offset":11712,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01267.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::impl$23::default
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 96
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9AA8
// 首条位置: {"module":633,"inlinee":39592}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::impl$4::image_name_to_index
// 记录形式: compiled_procedure; instances=1
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":633,"record_offset":372,"rva":47815952,"va":5416525072}
// 编译实例 RVAs: 0x2d99d10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t97108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416525072,"raw_hex":"e8547b0100","size":5,"soft_fail":false,"target_va":5416622208,"va":5416525095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::RawShapeStream::new
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9AA6
// 首条位置: {"module":633,"inlinee":39590}
// 编译实例 RVAs: 0xcb3040
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] tg_text::shape_stream::impl$23::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-text/src/shape_stream.rs
//     evidence={"module":633,"record_offset":1600,"parent_record_offset":1576,"ancestor_record_offset":1576,"inlinee":39592,"source_line":96,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::RawShapeStream::clear
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 165, 166
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9AA9
// 首条位置: {"module":633,"inlinee":39593}
// 编译实例 RVAs: 0x698b40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":633,"record_offset":1652,"parent_record_offset":1628,"ancestor_record_offset":1628,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::FillColor::with_color
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 185
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9AAC
// 首条位置: {"module":633,"inlinee":39596}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::Image::with_scaling
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 193
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9AAE
// 首条位置: {"module":633,"inlinee":39598}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_text::shape_stream::RawShapeStream::encode
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 206
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9AAA
// 首条位置: {"module":633,"inlinee":39594}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$22::extend
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":633,"record_offset":2864,"parent_record_offset":2836,"ancestor_record_offset":2836,"inlinee":11091,"source_line":4166,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":633,"record_offset":3112,"parent_record_offset":3080,"ancestor_record_offset":2836,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":633,"record_offset":2984,"parent_record_offset":2956,"ancestor_record_offset":2836,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":633,"record_offset":3612,"parent_record_offset":3584,"ancestor_record_offset":3364,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":633,"record_offset":3040,"parent_record_offset":3012,"ancestor_record_offset":2836,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":633,"record_offset":3160,"parent_record_offset":3136,"ancestor_record_offset":2836,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":633,"record_offset":3136,"parent_record_offset":3112,"ancestor_record_offset":2836,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":633,"record_offset":3012,"parent_record_offset":2984,"ancestor_record_offset":2836,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":633,"record_offset":2920,"parent_record_offset":2892,"ancestor_record_offset":2836,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":633,"record_offset":3080,"parent_record_offset":2920,"ancestor_record_offset":2836,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":633,"record_offset":3200,"parent_record_offset":2920,"ancestor_record_offset":2836,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":633,"record_offset":2956,"parent_record_offset":2920,"ancestor_record_offset":2836,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00633.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":633,"record_offset":2892,"parent_record_offset":2864,"ancestor_record_offset":2836,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00633.jsonl.gz"}
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
// 完整结构及原始类型记录: shape_stream.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_text::shape_stream::FillColor
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_text::shape_stream::Image
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_text::shape_stream::RawShapeStream
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 6; 过程引用数: 3
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_text::shape_stream::ShapeStream
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 7; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tg_text::shape_stream::ShapeStream>", "constructor": "core::ptr::drop_in_place", "arguments": ["tg_text::shape_stream::ShapeStream"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_text::shape_stream::ShapeStreamItem
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "enum2$<tg_text::shape_stream::ShapeStreamItem>", "constructor": "enum2$", "arguments": ["tg_text::shape_stream::ShapeStreamItem"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<enum2$<tg_text::shape_stream::ShapeStreamItem>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["enum2$<tg_text::shape_stream::ShapeStreamItem>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<enum2$<tg_text::shape_stream::ShapeStreamItem>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["enum2$<tg_text::shape_stream::ShapeStreamItem>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<enum2$<tg_text::shape_stream::ShapeStreamItem>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<enum2$<tg_text::shape_stream::ShapeStreamItem>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$12::clone<enum2$<tg_text::shape_stream::ShapeStreamItem>,alloc::alloc::Global>", "constructor": "alloc::vec::impl$12::clone", "arguments": ["enum2$<tg_text::shape_stream::ShapeStreamItem>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<enum2$<tg_text::shape_stream::ShapeStreamItem> >", "constructor": "core::marker::PhantomData", "arguments": ["enum2$<tg_text::shape_stream::ShapeStreamItem>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$5::next_element_seed<core::marker::PhantomData<enum2$<tg_text::shape_stream::ShapeStreamItem> > >", "constructor": "ron::de::impl$5::next_element_seed", "arguments": ["core::marker::PhantomData<enum2$<tg_text::shape_stream::ShapeStreamItem> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$19::deserialize::VecVisitor<enum2$<tg_text::shape_stream::ShapeStreamItem> >", "constructor": "serde_core::de::impls::impl$19::deserialize::VecVisitor", "arguments": ["enum2$<tg_text::shape_stream::ShapeStreamItem>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$2::deserialize_seq<serde_core::de::impls::impl$19::deserialize::VecVisitor<enum2$<tg_text::shape_stream::ShapeStreamItem> > >", "constructor": "ron::de::impl$2::deserialize_seq", "arguments": ["serde_core::de::impls::impl$19::deserialize::VecVisitor<enum2$<tg_text::shape_stream::ShapeStreamItem> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq<enum2$<tg_text::shape_stream::ShapeStreamItem>,ron::de::CommaSeparated>", "constructor": "serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq", "arguments": ["enum2$<tg_text::shape_stream::ShapeStreamItem>", "ron::de::CommaSeparated"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: shape_stream.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: shape_stream.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::save::save_backup_list::BackupSaveFile> > (*)(ref_mut$<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::Flatten<std::fs::ReadDir>,country_core::systems::save::save_backup_list::impl$1::iter_backup_files::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] tuple$<bool,tuple$<> > (*)(ref_mut$<bevy_ecs::storage::sparse_set::SparseSets>,enum2$<core::option::Option<ref_mut$<bevy_ecs::storage::table::Table> > >,ref$<bevy_ecs::component::info::Components>,ref$<slice2$<bevy_ecs::component::info::ComponentId> >)
// 候选参数类型: ["ref_mut$<bevy_ecs::storage::sparse_set::SparseSets>", "enum2$<core::option::Option<ref_mut$<bevy_ecs::storage::table::Table> > >", "ref$<bevy_ecs::component::info::Components>", "ref$<slice2$<bevy_ecs::component::info::ComponentId> >"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2DFollowMouse> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2DFollowMouse> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<country_core::components::Ui2DFollowMouse> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::eraser::ui_bulldozer::DeleteDecoratorCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::eraser::ui_bulldozer::ChangeDecoratorTypeCmd>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::eraser::ui_bulldozer::HandleStairAttachedDecoratorsOnStairPointDeletion>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::eraser::ui_bulldozer::DeleteDecoratorCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::eraser::ui_bulldozer::ChangeDecoratorTypeCmd>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::eraser::ui_bulldozer::HandleStairAttachedDecoratorsOnStairPointDeletion>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::eraser::ui_bulldozer::DeleteDecoratorCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::eraser::ui_bulldozer::ChangeDecoratorTypeCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::eraser::ui_bulldozer::HandleStairAttachedDecoratorsOnStairPointDeletion>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::FillColor::with_color
// 信息 ID: f721ba4ccf66dc6455d6ccf2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_text::shape_stream::RawShapeStream::append; module=633, inline_record=3336; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::Image::with_scaling
// 信息 ID: ca274ab29607cb6ae7e71afc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_text::shape_stream::RawShapeStream::append; module=633, inline_record=3924; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::RawShapeStream::append
// 信息 ID: aa10d00051de589760b24def
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 10 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::impl$1::prepare_layout @ 0x67011e -> 0xcb3060 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::impl$1::prepare_layout @ 0x670279 -> 0xcb3060 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$12::prepare_layout @ 0x7214cf -> 0xcb3060 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$9::prepare_layout @ 0x721748 -> 0xcb3060 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$26::prepare_layout @ 0x7239f0 -> 0xcb3060 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$26::prepare_layout @ 0x723b7a -> 0xcb3060 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_text::shape_stream::ShapeStream::encode_no_images @ 0xcb2fcf -> 0xcb3060 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::quick_tool_palette::quick_tool_palette_draw_reticle @ 0x11d6143 -> 0xcb3060 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::credits::impl$2::render @ 0x22f58f5 -> 0xcb3060 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::report_bug::impl$8::prepare_layout @ 0x2324b64 -> 0xcb3060 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::RawShapeStream::clear
// 信息 ID: 88f74427237596cf17eb6592
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::impl$1::prepare_layout @ 0x670079 -> 0x698b40 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::impl$1::prepare_layout @ 0x6701d9 -> 0x698b40 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$12::prepare_layout @ 0x7214b9 -> 0x698b40 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$9::prepare_layout @ 0x72166c -> 0x698b40 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$26::prepare_layout @ 0x723997 -> 0x698b40 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$26::prepare_layout @ 0x723a57 -> 0x698b40 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::report_bug::impl$8::prepare_layout @ 0x2324ac3 -> 0x698b40 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] tg_text::shape_stream::ShapeStream::encode_no_images; module=633, inline_record=1628; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::RawShapeStream::encode
// 信息 ID: 427f69a2d8dbdaed21caab33
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_text::shape_stream::RawShapeStream::append; module=633, inline_record=2836; 不是独立调用边。
// [PDB 内联宿主] tg_text::shape_stream::RawShapeStream::append; module=633, inline_record=3364; 不是独立调用边。
// [PDB 内联宿主] tg_text::shape_stream::RawShapeStream::append; module=633, inline_record=3952; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::RawShapeStream::new
// 信息 ID: a3654ce5632e2f1e2e5f4c81
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::quick_tool_palette::quick_tool_palette_draw_reticle @ 0x11d6098 -> 0xcb3040 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::credits::impl$2::render @ 0x22f58bc -> 0xcb3040 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] tg_text::shape_stream::ShapeStream::encode_no_images; module=633, inline_record=1576; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::ShapeStream::corner_radius
// 信息 ID: f8c03456aa44867a62b64317
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 14 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$5::closure$0 @ 0x31439f -> 0xcb2ee0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::text_pill_button @ 0x327bd3 -> 0xcb2ee0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::impl$2::render @ 0x674083 -> 0xcb2ee0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::impl$2::render @ 0x6741b1 -> 0xcb2ee0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::impl$2::render @ 0x6742a3 -> 0xcb2ee0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::impl$2::render @ 0x674443 -> 0xcb2ee0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::impl$2::render @ 0x674546 -> 0xcb2ee0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::impl$2::render @ 0x674320 -> 0xcb2ee0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$26::prepare_layout @ 0x7239d7 -> 0xcb2ee0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$19::render @ 0x724328 -> 0xcb2ee0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$19::render @ 0x72443d -> 0xcb2ee0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::core::mei_post_update @ 0x72b43c -> 0xcb2ee0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::ScrollingListView::with_items<toolbar::menu::mod_checklist::replacement_mod_checklist::closure_env$0> @ 0x231aec9 -> 0xcb2ee0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::report_bug::ui @ 0x2321bd8 -> 0xcb2ee0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::ShapeStream::encode_no_images
// 信息 ID: fe6c1a390591aedb2d6194ac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 11 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_fog::impl$2::render @ 0x674093 -> 0xcb2f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::impl$2::render @ 0x6741c1 -> 0xcb2f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::impl$2::render @ 0x6742b0 -> 0xcb2f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::impl$2::render @ 0x674453 -> 0xcb2f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::impl$2::render @ 0x674556 -> 0xcb2f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::impl$2::render @ 0x67432d -> 0xcb2f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$19::render @ 0x724334 -> 0xcb2f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$19::render @ 0x72444d -> 0xcb2f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::core::mei_post_update @ 0x72b450 -> 0xcb2f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::quick_tool_palette::quick_tool_palette_draw_reticle @ 0x11d6398 -> 0xcb2f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::quick_tool_palette::quick_tool_palette_draw_reticle @ 0x11d6429 -> 0xcb2f90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::ShapeStream::fill_color
// 信息 ID: 0aedbfb5983033fcd532693c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 22 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$5::closure$0 @ 0x31438b -> 0xcb2e30 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::text_pill_button @ 0x327bbf -> 0xcb2e30 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::impl$1::prepare_layout @ 0x670095 -> 0xcb2e30 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::impl$1::prepare_layout @ 0x6701f4 -> 0xcb2e30 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::impl$2::render @ 0x674070 -> 0xcb2e30 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::impl$2::render @ 0x67419a -> 0xcb2e30 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::impl$2::render @ 0x674290 -> 0xcb2e30 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::impl$2::render @ 0x674430 -> 0xcb2e30 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::impl$2::render @ 0x674533 -> 0xcb2e30 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::impl$2::render @ 0x67430d -> 0xcb2e30 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$9::prepare_layout @ 0x72172f -> 0xcb2e30 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$26::prepare_layout @ 0x7239c4 -> 0xcb2e30 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$26::prepare_layout @ 0x723b4f -> 0xcb2e30 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$19::render @ 0x72431a -> 0xcb2e30 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$19::render @ 0x72442b -> 0xcb2e30 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::core::mei_post_update @ 0x72b41c -> 0xcb2e30 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::ShapeStream::image
// 信息 ID: e47d32391e96d56f1b4e4037
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::gui::wall_selection_tags::impl$1::prepare_layout; module=246, inline_record=400; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::gui::wall_selection_tags::impl$1::prepare_layout; module=246, inline_record=1872; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::decorate::impl$1::decorate_clutter_selector; module=347, inline_record=8396; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::impl$9::prepare_layout; module=347, inline_record=36812; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::impl$26::prepare_layout; module=347, inline_record=58072; 不是独立调用边。
// [PDB 内联宿主] toolbar::quick_tool_palette::quick_tool_palette_draw_reticle; module=1289, inline_record=37344; 不是独立调用边。
// [PDB 内联宿主] toolbar::menu::report_bug::impl$8::prepare_layout; module=2874, inline_record=36180; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::ShapeStream::is_empty
// 信息 ID: 3085e1a7a25cbae46137ad8a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::impl$12::render @ 0x7214fc -> 0xcb3030 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::save_error::ui @ 0x23c8e0d -> 0xcb3030 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::ShapeStream::new
// 信息 ID: 9a6c1698c249e169e13a98bb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 27 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::settings::do_gui_escape_menu_settings::closure$5::closure$0 @ 0x314376 -> 0xb8f4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::text_pill_button @ 0x327a9d -> 0xb8f4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::settings::text_pill_button @ 0x327ba1 -> 0xb8f4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::app_systems::impl$0::add_wall_events @ 0x51d04c -> 0xb8f4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::impl$1::prepare_layout @ 0x670085 -> 0xb8f4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::impl$1::prepare_layout @ 0x6701e5 -> 0xb8f4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::impl$2::render @ 0x67405b -> 0xb8f4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::impl$2::render @ 0x674185 -> 0xb8f4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::impl$2::render @ 0x67427b -> 0xb8f4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::impl$2::render @ 0x67441b -> 0xb8f4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::impl$2::render @ 0x67451e -> 0xb8f4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_fog::impl$2::render @ 0x6742f8 -> 0xb8f4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::decorate::impl$1::decorate_clutter_selector @ 0x71f851 -> 0xb8f4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$9::prepare_layout @ 0x721678 -> 0xb8f4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$26::prepare_layout @ 0x7239a3 -> 0xb8f4d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$26::prepare_layout @ 0x723a60 -> 0xb8f4d0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::_::impl$0::deserialize
// 信息 ID: 3219bca85e985252eda6d750
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$9::struct_variant<tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=2887, inline_record=21812; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$5::next_element_seed<core::marker::PhantomData<enum2$<tg_text::shape_stream::ShapeStreamItem> > >; module=2887, inline_record=38888; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<toolbar::mei::config::_::impl$0::deserialize::__Visitor>; module=2887, inline_record=104096; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 1cdfc7d37b89b367b3b13ac9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<tg_text::shape_stream::_::impl$0::deserialize::__Field> >; module=2887, inline_record=17440; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >
// 信息 ID: ff5dedafaf353e0c67c2c765
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["ron::error::Error"]; 字段布局 = unknown。
// [类型关系] tg_text::shape_stream::_::impl$0::deserialize::impl$0::visit_str -> ["enum2$<ron::error::Error>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$8::variant_seed<core::marker::PhantomData<tg_text::shape_stream::_::impl$0::deserialize::__Field> > @ 0x2330736 -> 0x29941b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 19f0a1fa5d00b2d1b9acf35f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<tg_text::shape_stream::_::impl$0::deserialize::__Field> >; module=2887, inline_record=15012; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$8::variant_seed<core::marker::PhantomData<tg_text::shape_stream::_::impl$0::deserialize::__Field> >; module=2887, inline_record=17296; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum
// 信息 ID: 99aaf582cf7f74607e40977a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_enum<tg_text::shape_stream::_::impl$0::deserialize::__Visitor>; module=2887, inline_record=79056; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::impl$0::visit_str
// 信息 ID: ba3a868610ccaf43f01902e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$9::struct_variant<tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=2887, inline_record=21036; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_str<tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::__FieldVisitor>; module=2887, inline_record=74464; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_str<tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::__FieldVisitor>; module=2887, inline_record=74812; 不是独立调用边。
// [PDB 内联宿主] ron::de::id::impl$1::deserialize_identifier<tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::__FieldVisitor>; module=4229, inline_record=1196; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::impl$1::deserialize
// 信息 ID: 1f586c2b87e9067b4ef5cede
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$9::struct_variant<tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=2887, inline_record=20632; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$9::struct_variant<tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=2887, inline_record=20748; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$9::struct_variant<tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=2887, inline_record=20904; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::impl$2::visit_map
// 信息 ID: c11b223098cc8f56fc64be3a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$9::struct_variant<tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::__Visitor>; module=2887, inline_record=20216; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum<ron::de::Enum>
// 信息 ID: d35ba9ad1d0f4f3bad5d486b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum -> ["ron::de::Enum"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_enum<tg_text::shape_stream::_::impl$0::deserialize::__Visitor> @ 0x233a693 -> 0x2994070 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::_::impl$0::deserialize<ref_mut$<ron::de::Deserializer> >
// 信息 ID: 7df6e20bd4473567aeac9466
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["ron::de::Deserializer"]; 字段布局 = unknown。
// [类型关系] tg_text::shape_stream::_::impl$0::deserialize -> ["ref_mut$<ron::de::Deserializer>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$9::struct_variant<tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x2331ca9 -> 0x2994320 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::_::impl$0::deserialize<ref_mut$<ron::de::tag::Deserializer> >
// 信息 ID: 9d0d7d1a9f0a88af2fe243f1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["ron::de::tag::Deserializer"]; 字段布局 = unknown。
// [类型关系] tg_text::shape_stream::_::impl$0::deserialize -> ["ref_mut$<ron::de::tag::Deserializer>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$9::struct_variant<tg_text::shape_stream::_::impl$0::deserialize::impl$2::visit_enum::__Visitor> @ 0x2331ce6 -> 0x2994290 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::impl$1::from
// 信息 ID: e7bbf969ccacbee57bbb8f42
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::impl$9::prepare_layout @ 0x72168a -> 0xcb2e10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::impl$20::clone
// 信息 ID: a70a042a30b61bb12c42f95f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::impl$12::clone<enum2$<tg_text::shape_stream::ShapeStreamItem>,alloc::alloc::Global>; module=1267, inline_record=11752; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::impl$22::clone
// 信息 ID: 3b21a6bb354bf79a3d4c4a05
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::impl$12::clone<enum2$<tg_text::shape_stream::ShapeStreamItem>,alloc::alloc::Global>; module=1267, inline_record=11712; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::impl$23::default
// 信息 ID: 9261341177874b159be7c37d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_text::shape_stream::ShapeStream::encode_no_images; module=633, inline_record=1600; 不是独立调用边。
// [PDB 内联宿主] tg_text::shape_stream::RawShapeStream::new; module=633, inline_record=2392; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::impl$2::from
// 信息 ID: ae548ac86ebbaf2567fd0700
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::impl$1::prepare_layout @ 0x6700a5 -> 0xcb32b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::impl$1::prepare_layout @ 0x670204 -> 0xcb32b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::decorate::impl$1::decorate_clutter_selector @ 0x71f861 -> 0xcb32b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$26::prepare_layout @ 0x723a71 -> 0xcb32b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::report_bug::impl$8::prepare_layout @ 0x2324add -> 0xcb32b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::impl$3::from
// 信息 ID: e808d6fa99cd37fba0013889
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 42 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x2245f7 -> 0x6c6ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x2248d5 -> 0x6c6ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x224cc3 -> 0x6c6ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x224ec7 -> 0x6c6ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::render::render_config::UpscalingMode> > @ 0x225253 -> 0x6c6ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::render::render_config::UpscalingMode> > @ 0x225457 -> 0x6c6ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x2257e3 -> 0x6c6ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x2259c9 -> 0x6c6ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::render::render_config::UpscalingPreset> > @ 0x225dd3 -> 0x6c6ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::render::render_config::UpscalingPreset> > @ 0x225fd7 -> 0x6c6ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableBool> @ 0x226365 -> 0x6c6ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableBool> @ 0x226501 -> 0x6c6ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x226844 -> 0x6c6ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x226a45 -> 0x6c6ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x226d53 -> 0x6c6ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x227026 -> 0x6c6ac0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::impl$4::image_name_to_index
// 信息 ID: 0982cbe3965f7aa80ab49919
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_text::shape_stream::impl$8::clone
// 信息 ID: baf1e3e9c7558615e773894f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::config::instantiate_ui_template_impl; module=356, inline_record=5480; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
