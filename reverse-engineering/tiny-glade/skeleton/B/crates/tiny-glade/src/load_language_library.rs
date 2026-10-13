// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tiny-glade/src/load_language_library.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 3cbf4d8ddbd489796147bb337ec85aa5c07d6805ef334adc17185f35fa02327e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: load_language_library.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 131
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use core; // namespace candidate
//   package identities: []
// use lang_lib; // namespace candidate
//   package identities: [{"name":"lang-lib","source":"local_or_path","version":"0.1.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use steamworks; // namespace candidate
//   package identities: [{"name":"steamworks","source":"git+https://github.com/PounceLight/steamworks-rs.git?rev=d05eaa60db328156aeee172b0b157f3f69c70b30#d05eaa60db328156aeee172b0b157f3f69c70b30","version":"0.11.0"}]
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::load_language_library::load_language_library
// 记录形式: compiled_procedure; instances=1
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":131,"record_offset":71432,"rva":3289648,"va":5371998768}
// 编译实例 RVAs: 0x323230
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1404b1430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1629712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371998768,"raw_hex":"e810de1800","size":5,"soft_fail":false,"target_va":5373629488,"va":5371999771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: load_language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3082656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371998768,"raw_hex":"e8a0092f00","size":5,"soft_fail":false,"target_va":5375081488,"va":5371998827,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: load_language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3082390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371998768,"raw_hex":"e896082f00","size":5,"soft_fail":false,"target_va":5375081504,"va":5371999109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=2
//     target_aliases=69 candidates; complete list: load_language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3517154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371998768,"raw_hex":"e8e2aa3500","size":5,"soft_fail":false,"target_va":5375516032,"va":5371998873,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1429234c0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39846397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371998768,"raw_hex":"e8fd016002","size":5,"soft_fail":false,"target_va":5411845312,"va":5371998910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923540
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39846554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371998768,"raw_hex":"e89a026002","size":5,"soft_fail":false,"target_va":5411845440,"va":5371998881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39848101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371998768,"raw_hex":"e8a5086002","size":5,"soft_fail":false,"target_va":5411847168,"va":5371999062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142924010
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39849282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371998768,"raw_hex":"e8420d6002","size":5,"soft_fail":false,"target_va":5411848208,"va":5371998921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::ffi::c_str::_$LT$impl$u20$core..ffi..c_str..CStr$GT$::to_string_lossy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10841234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371998768,"raw_hex":"e8926ca500","size":5,"soft_fail":false,"target_va":5382840176,"va":5371998937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39540991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371998768,"raw_hex":"e8ff585b02","size":5,"soft_fail":false,"target_va":5411540963,"va":5371999967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::str::_$LT$impl$u20$str$GT$::to_lowercase
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10841566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371998768,"raw_hex":"e8de6da500","size":5,"soft_fail":false,"target_va":5382840656,"va":5371999085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t8789940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371998768,"raw_hex":"e8b41f8600","size":5,"soft_fail":false,"target_va":5380789072,"va":5371999127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10654653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371998768,"raw_hex":"e8bd93a200","size":5,"soft_fail":false,"target_va":5382655024,"va":5372000366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39542864","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371998768,"raw_hex":"e850605b02","size":5,"soft_fail":false,"target_va":5411542880,"va":5372000011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::load_locale
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8883298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371998768,"raw_hex":"e8628c8700","size":5,"soft_fail":false,"target_va":5380883200,"va":5371999897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":131,"record_offset":71604,"parent_record_offset":71432,"ancestor_record_offset":71432,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":131,"record_offset":76216,"parent_record_offset":71432,"ancestor_record_offset":71432,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":131,"record_offset":76248,"parent_record_offset":71432,"ancestor_record_offset":71432,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":131,"record_offset":72076,"parent_record_offset":71432,"ancestor_record_offset":71432,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":131,"record_offset":76276,"parent_record_offset":71432,"ancestor_record_offset":71432,"inlinee":4666,"source_line":998,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":131,"record_offset":76372,"parent_record_offset":71432,"ancestor_record_offset":71432,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":131,"record_offset":76304,"parent_record_offset":71432,"ancestor_record_offset":71432,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":131,"record_offset":74432,"parent_record_offset":71432,"ancestor_record_offset":71432,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":131,"record_offset":76132,"parent_record_offset":71432,"ancestor_record_offset":71432,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":131,"record_offset":71920,"parent_record_offset":71896,"ancestor_record_offset":71432,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":131,"record_offset":71944,"parent_record_offset":71920,"ancestor_record_offset":71432,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":131,"record_offset":73320,"parent_record_offset":73296,"ancestor_record_offset":71432,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":131,"record_offset":71896,"parent_record_offset":71868,"ancestor_record_offset":71432,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":131,"record_offset":73296,"parent_record_offset":73240,"ancestor_record_offset":71432,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::Cow::into_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\borrow.rs
//     evidence={"module":131,"record_offset":72456,"parent_record_offset":72324,"ancestor_record_offset":71432,"inlinee":10890,"source_line":320,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":131,"record_offset":71808,"parent_record_offset":71780,"ancestor_record_offset":71432,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":131,"record_offset":73268,"parent_record_offset":73240,"ancestor_record_offset":71432,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":131,"record_offset":73240,"parent_record_offset":73216,"ancestor_record_offset":71432,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":131,"record_offset":71868,"parent_record_offset":71836,"ancestor_record_offset":71432,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":131,"record_offset":71836,"parent_record_offset":71808,"ancestor_record_offset":71432,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":131,"record_offset":73216,"parent_record_offset":73192,"ancestor_record_offset":71432,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":131,"record_offset":72816,"parent_record_offset":72780,"ancestor_record_offset":71432,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":131,"record_offset":71744,"parent_record_offset":71716,"ancestor_record_offset":71432,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":131,"record_offset":71716,"parent_record_offset":71688,"ancestor_record_offset":71432,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":131,"record_offset":71660,"parent_record_offset":71632,"ancestor_record_offset":71432,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":131,"record_offset":71688,"parent_record_offset":71660,"ancestor_record_offset":71432,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":131,"record_offset":71632,"parent_record_offset":71604,"ancestor_record_offset":71432,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":131,"record_offset":72496,"parent_record_offset":72456,"ancestor_record_offset":71432,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":131,"record_offset":72224,"parent_record_offset":72196,"ancestor_record_offset":71432,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":131,"record_offset":72196,"parent_record_offset":72168,"ancestor_record_offset":71432,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":131,"record_offset":73420,"parent_record_offset":73396,"ancestor_record_offset":71432,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":131,"record_offset":71780,"parent_record_offset":71744,"ancestor_record_offset":71432,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":131,"record_offset":72840,"parent_record_offset":72816,"ancestor_record_offset":71432,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":131,"record_offset":72864,"parent_record_offset":72840,"ancestor_record_offset":71432,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":131,"record_offset":74460,"parent_record_offset":74432,"ancestor_record_offset":71432,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::CStr::from_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ffi\c_str.rs
//     evidence={"module":131,"record_offset":72360,"parent_record_offset":72324,"ancestor_record_offset":71432,"inlinee":12585,"source_line":264,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::strlen
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ffi\c_str.rs
//     evidence={"module":131,"record_offset":72388,"parent_record_offset":72360,"ancestor_record_offset":71432,"inlinee":12587,"source_line":721,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::strlen::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":131,"record_offset":72416,"parent_record_offset":72388,"ancestor_record_offset":71432,"inlinee":12589,"source_line":3874,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":131,"record_offset":72020,"parent_record_offset":71996,"ancestor_record_offset":71432,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":131,"record_offset":72912,"parent_record_offset":72888,"ancestor_record_offset":71432,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":131,"record_offset":72888,"parent_record_offset":72864,"ancestor_record_offset":71432,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":131,"record_offset":71996,"parent_record_offset":71744,"ancestor_record_offset":71432,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":131,"record_offset":73144,"parent_record_offset":72132,"ancestor_record_offset":71432,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":131,"record_offset":72256,"parent_record_offset":72224,"ancestor_record_offset":71432,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":131,"record_offset":74488,"parent_record_offset":74460,"ancestor_record_offset":71432,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":131,"record_offset":74516,"parent_record_offset":74488,"ancestor_record_offset":71432,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":131,"record_offset":76156,"parent_record_offset":76132,"ancestor_record_offset":71432,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":131,"record_offset":72284,"parent_record_offset":72224,"ancestor_record_offset":71432,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":131,"record_offset":76180,"parent_record_offset":76156,"ancestor_record_offset":71432,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":131,"record_offset":73516,"parent_record_offset":73456,"ancestor_record_offset":71432,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":131,"record_offset":73548,"parent_record_offset":73420,"ancestor_record_offset":71432,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] steamworks::Client::apps
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"steamworks","version":"0.11.0","source":"git+https://github.com/PounceLight/steamworks-rs.git?rev=d05eaa60db328156aeee172b0b157f3f69c70b30#d05eaa60db328156aeee172b0b157f3f69c70b30"}]
//     definition_source=/home/h3/.cargo/git/checkouts/steamworks-rs-f13bba748a20c02a/d05eaa6/src/lib.rs
//     evidence={"module":131,"record_offset":72168,"parent_record_offset":72132,"ancestor_record_offset":71432,"inlinee":8454,"source_line":321,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] steamworks::app::Apps::current_game_language
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"steamworks","version":"0.11.0","source":"git+https://github.com/PounceLight/steamworks-rs.git?rev=d05eaa60db328156aeee172b0b157f3f69c70b30#d05eaa60db328156aeee172b0b157f3f69c70b30"}]
//     definition_source=/home/h3/.cargo/git/checkouts/steamworks-rs-f13bba748a20c02a/d05eaa6/src/app.rs
//     evidence={"module":131,"record_offset":72324,"parent_record_offset":72132,"ancestor_record_offset":71432,"inlinee":12583,"source_line":114,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::load_language_library::load_language_library::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/load_language_library.rs
//     evidence={"module":131,"record_offset":72132,"parent_record_offset":72076,"ancestor_record_offset":71432,"inlinee":12581,"source_line":8,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::load_language_library::load_language_library::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 8
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3125
// 首条位置: {"module":131,"inlinee":12581}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":131,"record_offset":73144,"parent_record_offset":72132,"ancestor_record_offset":72132,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_direct] steamworks::Client::apps
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"steamworks","version":"0.11.0","source":"git+https://github.com/PounceLight/steamworks-rs.git?rev=d05eaa60db328156aeee172b0b157f3f69c70b30#d05eaa60db328156aeee172b0b157f3f69c70b30"}]
//     definition_source=/home/h3/.cargo/git/checkouts/steamworks-rs-f13bba748a20c02a/d05eaa6/src/lib.rs
//     evidence={"module":131,"record_offset":72168,"parent_record_offset":72132,"ancestor_record_offset":72132,"inlinee":8454,"source_line":321,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_direct] steamworks::app::Apps::current_game_language
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"steamworks","version":"0.11.0","source":"git+https://github.com/PounceLight/steamworks-rs.git?rev=d05eaa60db328156aeee172b0b157f3f69c70b30#d05eaa60db328156aeee172b0b157f3f69c70b30"}]
//     definition_source=/home/h3/.cargo/git/checkouts/steamworks-rs-f13bba748a20c02a/d05eaa6/src/app.rs
//     evidence={"module":131,"record_offset":72324,"parent_record_offset":72132,"ancestor_record_offset":72132,"inlinee":12583,"source_line":114,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":131,"record_offset":72980,"parent_record_offset":72956,"ancestor_record_offset":72132,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":131,"record_offset":73004,"parent_record_offset":72980,"ancestor_record_offset":72132,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":131,"record_offset":73320,"parent_record_offset":73296,"ancestor_record_offset":72132,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":131,"record_offset":72956,"parent_record_offset":72780,"ancestor_record_offset":72132,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":131,"record_offset":73296,"parent_record_offset":73240,"ancestor_record_offset":72132,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::Cow::into_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\borrow.rs
//     evidence={"module":131,"record_offset":72456,"parent_record_offset":72324,"ancestor_record_offset":72132,"inlinee":10890,"source_line":320,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":131,"record_offset":72720,"parent_record_offset":72692,"ancestor_record_offset":72132,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":131,"record_offset":73268,"parent_record_offset":73240,"ancestor_record_offset":72132,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":131,"record_offset":73240,"parent_record_offset":73216,"ancestor_record_offset":72132,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":131,"record_offset":72780,"parent_record_offset":72748,"ancestor_record_offset":72132,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":131,"record_offset":72748,"parent_record_offset":72720,"ancestor_record_offset":72132,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":131,"record_offset":73216,"parent_record_offset":73192,"ancestor_record_offset":72132,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":131,"record_offset":72816,"parent_record_offset":72780,"ancestor_record_offset":72132,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":131,"record_offset":72656,"parent_record_offset":72624,"ancestor_record_offset":72132,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":131,"record_offset":72624,"parent_record_offset":72592,"ancestor_record_offset":72132,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":131,"record_offset":72560,"parent_record_offset":72528,"ancestor_record_offset":72132,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":131,"record_offset":72592,"parent_record_offset":72560,"ancestor_record_offset":72132,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":131,"record_offset":72528,"parent_record_offset":72496,"ancestor_record_offset":72132,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":131,"record_offset":72496,"parent_record_offset":72456,"ancestor_record_offset":72132,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":131,"record_offset":72224,"parent_record_offset":72196,"ancestor_record_offset":72132,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":131,"record_offset":72196,"parent_record_offset":72168,"ancestor_record_offset":72132,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":131,"record_offset":73420,"parent_record_offset":73396,"ancestor_record_offset":72132,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":131,"record_offset":72692,"parent_record_offset":72656,"ancestor_record_offset":72132,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":131,"record_offset":72840,"parent_record_offset":72816,"ancestor_record_offset":72132,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":131,"record_offset":72864,"parent_record_offset":72840,"ancestor_record_offset":72132,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::CStr::from_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ffi\c_str.rs
//     evidence={"module":131,"record_offset":72360,"parent_record_offset":72324,"ancestor_record_offset":72132,"inlinee":12585,"source_line":264,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::strlen
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ffi\c_str.rs
//     evidence={"module":131,"record_offset":72388,"parent_record_offset":72360,"ancestor_record_offset":72132,"inlinee":12587,"source_line":721,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::strlen::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":131,"record_offset":72416,"parent_record_offset":72388,"ancestor_record_offset":72132,"inlinee":12589,"source_line":3874,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":131,"record_offset":73080,"parent_record_offset":73056,"ancestor_record_offset":72132,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":131,"record_offset":72912,"parent_record_offset":72888,"ancestor_record_offset":72132,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":131,"record_offset":72888,"parent_record_offset":72864,"ancestor_record_offset":72132,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":131,"record_offset":73056,"parent_record_offset":72656,"ancestor_record_offset":72132,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":131,"record_offset":73168,"parent_record_offset":73144,"ancestor_record_offset":72132,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":131,"record_offset":72256,"parent_record_offset":72224,"ancestor_record_offset":72132,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":131,"record_offset":72284,"parent_record_offset":72224,"ancestor_record_offset":72132,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":131,"record_offset":73516,"parent_record_offset":73456,"ancestor_record_offset":72132,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00131.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":131,"record_offset":73548,"parent_record_offset":73420,"ancestor_record_offset":72132,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00131.jsonl.gz"}
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
// 完整结构及原始类型记录: load_language_library.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: load_language_library.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: load_language_library.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>", "bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>", "bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>

// ------------------------------------------------------------------
// 函数: tiny_glade::load_language_library::load_language_library
// 信息 ID: 7455e31757abd5da8344ea55
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::main @ 0x3dd611 -> 0x323230 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::load_language_library::load_language_library::closure$0
// 信息 ID: 8632197a5716473ef775a2df
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::load_language_library::load_language_library", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::load_language_library::load_language_library; module=131, inline_record=72132; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
