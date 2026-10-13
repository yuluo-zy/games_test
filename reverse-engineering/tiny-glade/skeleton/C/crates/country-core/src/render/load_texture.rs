// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/render/load_texture.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: dfd42be88f723c58f9db84cd5e1ee97ddd20430db26a71b87c41d394c493b453
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: load_texture.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 576
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use memmap2; // namespace candidate
//   package identities: [{"name":"memmap2","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.8"},{"name":"memmap2","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.4"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use rkyv; // namespace candidate
//   package identities: [{"name":"rkyv","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.16"}]
// use std; // namespace candidate
//   package identities: []
// use turbosloth; // namespace candidate
//   package identities: [{"name":"turbosloth","source":"local_or_path","version":"0.2.0"}]
// use ustr; // namespace candidate
//   package identities: [{"name":"ustr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::render::load_texture::load_packaged_texture_image
// 记录形式: compiled_procedure; instances=1
// 源码行: 8
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":576,"record_offset":2372,"rva":12127392,"va":5380836512}
// 编译实例 RVAs: 0xb90ca0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::fs::OpenOptions>::_open
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3866171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380836512,"raw_hex":"e83bfe3a00","size":5,"soft_fail":false,"target_va":5384703536,"va":5380837360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3819161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380836512,"raw_hex":"e899463a00","size":5,"soft_fail":false,"target_va":5384656240,"va":5380837074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3772103","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380836512,"raw_hex":"e8c78e3900","size":5,"soft_fail":false,"target_va":5384609408,"va":5380837300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$memmap2..os..MmapInner$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t15705198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380836512,"raw_hex":"e86ea4ef00","size":5,"soft_fail":false,"target_va":5396542816,"va":5380837613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-5158403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380836512,"raw_hex":"e8fd49b1ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380837102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: load_texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11499733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380836512,"raw_hex":"e82b8750ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5380838576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f4de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11942604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380836512,"raw_hex":"e8cc3ab600","size":5,"soft_fail":false,"target_va":5392780768,"va":5380838159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35547209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380836512,"raw_hex":"e849681e02","size":5,"soft_fail":false,"target_va":5416385024,"va":5380837810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3785435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380836512,"raw_hex":"e8dbc23900","size":5,"soft_fail":false,"target_va":5384624080,"va":5380838640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35783603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380836512,"raw_hex":"e8b3032202","size":5,"soft_fail":false,"target_va":5416621408,"va":5380837800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35783842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380836512,"raw_hex":"e8a2042202","size":5,"soft_fail":false,"target_va":5416621632,"va":5380837785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380836512,"raw_hex":"e8516b0000","size":5,"soft_fail":false,"target_va":5380864096,"va":5380836618,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35783425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380836512,"raw_hex":"e801032202","size":5,"soft_fail":false,"target_va":5416621376,"va":5380837946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::load_texture::create_game_texture
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380836512,"raw_hex":"e8d5f9ffff","size":5,"soft_fail":false,"target_va":5380836016,"va":5380837590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::load_texture::load_packaged_texture_image::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t35552903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380836512,"raw_hex":"e8877e1e02","size":5,"soft_fail":false,"target_va":5416390816,"va":5380837908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] memmap2::os::MmapInner::map
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"memmap2","version":"0.9.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t15705520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380836512,"raw_hex":"e8b0a5ef00","size":5,"soft_fail":false,"target_va":5396542944,"va":5380837419,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] memmap2::os::file_len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"memmap2","version":"0.9.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1286193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380836512,"raw_hex":"e831a01300","size":5,"soft_fail":false,"target_va":5382123584,"va":5380837386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3126809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380836512,"raw_hex":"e819b62f00","size":5,"soft_fail":false,"target_va":5383963792,"va":5380836978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3135315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380836512,"raw_hex":"e853d72f00","size":5,"soft_fail":false,"target_va":5383972224,"va":5380836904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3127200","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380836512,"raw_hex":"e8a0b72f00","size":5,"soft_fail":false,"target_va":5383964912,"va":5380837707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3126233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380836512,"raw_hex":"e8d9b32f00","size":5,"soft_fail":false,"target_va":5383962816,"va":5380836578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35781503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380836512,"raw_hex":"e87ffb2102","size":5,"soft_fail":false,"target_va":5416619360,"va":5380837852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] turbosloth::store::AssetRecipeId::package_file_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t16298112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380836512,"raw_hex":"e880b0f800","size":5,"soft_fail":false,"target_va":5397135152,"va":5380837035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*45323544(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5380836512,"raw_hex":"ff151895b302","size":6,"soft_fail":false,"va":5380837626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":576,"record_offset":5472,"parent_record_offset":2372,"ancestor_record_offset":2372,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":576,"record_offset":6484,"parent_record_offset":2372,"ancestor_record_offset":2372,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":576,"record_offset":6024,"parent_record_offset":2372,"ancestor_record_offset":2372,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":576,"record_offset":6416,"parent_record_offset":2372,"ancestor_record_offset":2372,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":576,"record_offset":5784,"parent_record_offset":2372,"ancestor_record_offset":2372,"inlinee":6444,"source_line":1616,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":576,"record_offset":5528,"parent_record_offset":2372,"ancestor_record_offset":2372,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":576,"record_offset":5388,"parent_record_offset":2372,"ancestor_record_offset":2372,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] memmap2::Mmap::map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":576,"record_offset":5820,"parent_record_offset":2372,"ancestor_record_offset":2372,"inlinee":15362,"source_line":579,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":576,"record_offset":3976,"parent_record_offset":2372,"ancestor_record_offset":2372,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":576,"record_offset":2576,"parent_record_offset":2372,"ancestor_record_offset":2372,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":576,"record_offset":2964,"parent_record_offset":2372,"ancestor_record_offset":2372,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] rkyv::api::access_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/api/mod.rs
//     evidence={"module":576,"record_offset":6064,"parent_record_offset":2372,"ancestor_record_offset":2372,"inlinee":36604,"source_line":266,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] rkyv::vec::ArchivedVec::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":576,"record_offset":6180,"parent_record_offset":2372,"ancestor_record_offset":2372,"inlinee":28518,"source_line":55,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] std::fs::File::open
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":576,"record_offset":5640,"parent_record_offset":2372,"ancestor_record_offset":2372,"inlinee":7533,"source_line":570,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":576,"record_offset":4784,"parent_record_offset":2372,"ancestor_record_offset":2372,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":576,"record_offset":4992,"parent_record_offset":4968,"ancestor_record_offset":2372,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":576,"record_offset":5016,"parent_record_offset":4992,"ancestor_record_offset":2372,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":576,"record_offset":5040,"parent_record_offset":5016,"ancestor_record_offset":2372,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":576,"record_offset":4968,"parent_record_offset":4912,"ancestor_record_offset":2372,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":576,"record_offset":4940,"parent_record_offset":4912,"ancestor_record_offset":2372,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":576,"record_offset":4912,"parent_record_offset":4888,"ancestor_record_offset":2372,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":576,"record_offset":4888,"parent_record_offset":4864,"ancestor_record_offset":2372,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":576,"record_offset":4304,"parent_record_offset":4276,"ancestor_record_offset":2372,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":576,"record_offset":4332,"parent_record_offset":4304,"ancestor_record_offset":2372,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":576,"record_offset":4252,"parent_record_offset":4224,"ancestor_record_offset":2372,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":576,"record_offset":4276,"parent_record_offset":4252,"ancestor_record_offset":2372,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":576,"record_offset":4392,"parent_record_offset":4276,"ancestor_record_offset":2372,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":576,"record_offset":4504,"parent_record_offset":4480,"ancestor_record_offset":2372,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":576,"record_offset":5724,"parent_record_offset":5640,"ancestor_record_offset":2372,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":576,"record_offset":4356,"parent_record_offset":4332,"ancestor_record_offset":2372,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":576,"record_offset":6116,"parent_record_offset":6088,"ancestor_record_offset":2372,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":576,"record_offset":4124,"parent_record_offset":4084,"ancestor_record_offset":2372,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":576,"record_offset":5496,"parent_record_offset":5472,"ancestor_record_offset":2372,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":576,"record_offset":5896,"parent_record_offset":5872,"ancestor_record_offset":2372,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":576,"record_offset":2912,"parent_record_offset":2884,"ancestor_record_offset":2372,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":576,"record_offset":4740,"parent_record_offset":4084,"ancestor_record_offset":2372,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":576,"record_offset":4456,"parent_record_offset":4224,"ancestor_record_offset":2372,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":576,"record_offset":6336,"parent_record_offset":6304,"ancestor_record_offset":2372,"inlinee":4729,"source_line":1258,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":576,"record_offset":3384,"parent_record_offset":3360,"ancestor_record_offset":2372,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":576,"record_offset":3360,"parent_record_offset":3336,"ancestor_record_offset":2372,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":576,"record_offset":3568,"parent_record_offset":3540,"ancestor_record_offset":2372,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":576,"record_offset":6360,"parent_record_offset":6336,"ancestor_record_offset":2372,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":576,"record_offset":5984,"parent_record_offset":5844,"ancestor_record_offset":2372,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":576,"record_offset":5944,"parent_record_offset":5920,"ancestor_record_offset":2372,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":576,"record_offset":3668,"parent_record_offset":3640,"ancestor_record_offset":2372,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":576,"record_offset":3640,"parent_record_offset":3612,"ancestor_record_offset":2372,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":576,"record_offset":3312,"parent_record_offset":3288,"ancestor_record_offset":2372,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":576,"record_offset":3336,"parent_record_offset":3312,"ancestor_record_offset":2372,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":576,"record_offset":3696,"parent_record_offset":3668,"ancestor_record_offset":2372,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":576,"record_offset":3288,"parent_record_offset":3172,"ancestor_record_offset":2372,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":576,"record_offset":2704,"parent_record_offset":2680,"ancestor_record_offset":2372,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":576,"record_offset":3000,"parent_record_offset":2964,"ancestor_record_offset":2372,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":576,"record_offset":3140,"parent_record_offset":3108,"ancestor_record_offset":2372,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":576,"record_offset":3108,"parent_record_offset":3072,"ancestor_record_offset":2372,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":576,"record_offset":3048,"parent_record_offset":3024,"ancestor_record_offset":2372,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":576,"record_offset":3072,"parent_record_offset":3048,"ancestor_record_offset":2372,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":576,"record_offset":3024,"parent_record_offset":3000,"ancestor_record_offset":2372,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":576,"record_offset":3612,"parent_record_offset":3072,"ancestor_record_offset":2372,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":576,"record_offset":2680,"parent_record_offset":2648,"ancestor_record_offset":2372,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":576,"record_offset":2648,"parent_record_offset":2620,"ancestor_record_offset":2372,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":576,"record_offset":2804,"parent_record_offset":2772,"ancestor_record_offset":2372,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":576,"record_offset":2860,"parent_record_offset":2804,"ancestor_record_offset":2372,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":576,"record_offset":2772,"parent_record_offset":2744,"ancestor_record_offset":2372,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":576,"record_offset":2620,"parent_record_offset":2576,"ancestor_record_offset":2372,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":576,"record_offset":2884,"parent_record_offset":2860,"ancestor_record_offset":2372,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":576,"record_offset":3172,"parent_record_offset":3140,"ancestor_record_offset":2372,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":576,"record_offset":3456,"parent_record_offset":3172,"ancestor_record_offset":2372,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":576,"record_offset":3428,"parent_record_offset":3172,"ancestor_record_offset":2372,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":576,"record_offset":3232,"parent_record_offset":3172,"ancestor_record_offset":2372,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":576,"record_offset":5412,"parent_record_offset":5388,"ancestor_record_offset":2372,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":576,"record_offset":5436,"parent_record_offset":5412,"ancestor_record_offset":2372,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":576,"record_offset":5576,"parent_record_offset":5552,"ancestor_record_offset":2372,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":576,"record_offset":5552,"parent_record_offset":5528,"ancestor_record_offset":2372,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":576,"record_offset":5600,"parent_record_offset":5576,"ancestor_record_offset":2372,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] memmap2::MmapOptions::get_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":576,"record_offset":5872,"parent_record_offset":5844,"ancestor_record_offset":2372,"inlinee":11999,"source_line":228,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] memmap2::MmapOptions::map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":576,"record_offset":5844,"parent_record_offset":5820,"ancestor_record_offset":2372,"inlinee":12011,"source_line":338,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] memmap2::impl$2::get_len::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":576,"record_offset":5920,"parent_record_offset":5896,"ancestor_record_offset":2372,"inlinee":12001,"source_line":229,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":576,"record_offset":4012,"parent_record_offset":3976,"ancestor_record_offset":2372,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":576,"record_offset":7112,"parent_record_offset":7080,"ancestor_record_offset":2372,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":576,"record_offset":7552,"parent_record_offset":7348,"ancestor_record_offset":2372,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":576,"record_offset":4152,"parent_record_offset":4124,"ancestor_record_offset":2372,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":576,"record_offset":4224,"parent_record_offset":4084,"ancestor_record_offset":2372,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":576,"record_offset":4428,"parent_record_offset":4224,"ancestor_record_offset":2372,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::api::root_position
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/api/mod.rs
//     evidence={"module":576,"record_offset":6088,"parent_record_offset":6064,"ancestor_record_offset":2372,"inlinee":36605,"source_line":100,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":576,"record_offset":6256,"parent_record_offset":6232,"ancestor_record_offset":2372,"inlinee":28522,"source_line":290,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr_raw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":576,"record_offset":6280,"parent_record_offset":6256,"ancestor_record_offset":2372,"inlinee":28523,"source_line":219,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::offset_raw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":576,"record_offset":6304,"parent_record_offset":6280,"ancestor_record_offset":2372,"inlinee":28279,"source_line":205,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RelPtr::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":576,"record_offset":6232,"parent_record_offset":6204,"ancestor_record_offset":2372,"inlinee":28521,"source_line":575,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":576,"record_offset":6204,"parent_record_offset":6180,"ancestor_record_offset":2372,"inlinee":28520,"source_line":40,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":576,"record_offset":5664,"parent_record_offset":5640,"ancestor_record_offset":2372,"inlinee":7535,"source_line":1692,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":576,"record_offset":5752,"parent_record_offset":5640,"ancestor_record_offset":2372,"inlinee":7538,"source_line":1915,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\os\windows\io\handle.rs
//     evidence={"module":576,"record_offset":6636,"parent_record_offset":6612,"ancestor_record_offset":2372,"inlinee":4731,"source_line":385,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::OpenOptions::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":576,"record_offset":5692,"parent_record_offset":5664,"ancestor_record_offset":2372,"inlinee":7537,"source_line":215,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":576,"record_offset":4180,"parent_record_offset":4152,"ancestor_record_offset":2372,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":576,"record_offset":4084,"parent_record_offset":4048,"ancestor_record_offset":2372,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":576,"record_offset":4048,"parent_record_offset":4012,"ancestor_record_offset":2372,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::load_texture::load_packaged_texture_image::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":576,"record_offset":8596,"rva":47681696,"va":5416390816}
// 编译实例 RVAs: 0x2d790a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: load_texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-47052087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416390816,"raw_hex":"e8c90a32fd","size":5,"soft_fail":false,"target_va":5369338848,"va":5416390930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t231345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416390816,"raw_hex":"e8b1870300","size":5,"soft_fail":false,"target_va":5416622240,"va":5416390890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::load_texture::create_game_texture
// 记录形式: compiled_procedure; instances=1
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":576,"record_offset":556,"rva":12126896,"va":5380836016}
// 编译实例 RVAs: 0xb90ab0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1415e7450
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10840127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380836016,"raw_hex":"e83f68a500","size":5,"soft_fail":false,"target_va":5391676496,"va":5380836364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d78f90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35554044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380836016,"raw_hex":"e8fc821e02","size":5,"soft_fail":false,"target_va":5416390544,"va":5380836495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd5430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35932148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380836016,"raw_hex":"e8f4472402","size":5,"soft_fail":false,"target_va":5416768560,"va":5380836407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::array_elements
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t2652721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380836016,"raw_hex":"e8317a2800","size":5,"soft_fail":false,"target_va":5383488928,"va":5380836202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ustr::Ustr::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3123786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380836016,"raw_hex":"e84aaa2f00","size":5,"soft_fail":false,"target_va":5383960112,"va":5380836321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$6::try_into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":576,"record_offset":720,"parent_record_offset":556,"ancestor_record_offset":556,"inlinee":6246,"source_line":818,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":576,"record_offset":1224,"parent_record_offset":556,"ancestor_record_offset":556,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":576,"record_offset":1260,"parent_record_offset":556,"ancestor_record_offset":556,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::starts_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":576,"record_offset":804,"parent_record_offset":556,"ancestor_record_offset":556,"inlinee":8324,"source_line":1405,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::TextureDesc::mip_levels
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/texture.rs
//     evidence={"module":576,"record_offset":776,"parent_record_offset":556,"ancestor_record_offset":556,"inlinee":36602,"source_line":560,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] ustr::Ustr::as_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":576,"record_offset":1168,"parent_record_offset":556,"ancestor_record_offset":556,"inlinee":13928,"source_line":239,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] ustr::ustr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":576,"record_offset":1140,"parent_record_offset":556,"ancestor_record_offset":556,"inlinee":27983,"source_line":469,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":576,"record_offset":876,"parent_record_offset":852,"ancestor_record_offset":556,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$136::try_from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\num.rs
//     evidence={"module":576,"record_offset":744,"parent_record_offset":720,"ancestor_record_offset":556,"inlinee":36601,"source_line":296,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":576,"record_offset":900,"parent_record_offset":876,"ancestor_record_offset":556,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":576,"record_offset":924,"parent_record_offset":900,"ancestor_record_offset":556,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":576,"record_offset":852,"parent_record_offset":828,"ancestor_record_offset":556,"inlinee":8287,"source_line":2619,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":576,"record_offset":828,"parent_record_offset":804,"ancestor_record_offset":556,"inlinee":8286,"source_line":981,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] ustr::Ustr::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":576,"record_offset":1192,"parent_record_offset":1168,"ancestor_record_offset":556,"inlinee":13929,"source_line":308,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::load_texture::create_game_texture::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 84
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":576,"record_offset":1292,"rva":47681424,"va":5416390544}
// 编译实例 RVAs: 0x2d78f90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-40712000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416390544,"raw_hex":"e8c0c892fd","size":5,"soft_fail":false,"target_va":5375678704,"va":5416390699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009e6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: load_texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-47032597","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416390544,"raw_hex":"e8eb5632fd","size":5,"soft_fail":false,"target_va":5369358064,"va":5416390656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140103f50
//     owner=unresolved_namespace; count=1
//     target_aliases=17 candidates; complete list: load_texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-46616889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416390544,"raw_hex":"e8c7ae38fd","size":5,"soft_fail":false,"target_va":5369773904,"va":5416390788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77b70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416390544,"raw_hex":"e89aebffff","size":5,"soft_fail":false,"target_va":5416385392,"va":5416390609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":576,"record_offset":1444,"parent_record_offset":1292,"ancestor_record_offset":1292,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":576,"record_offset":1664,"parent_record_offset":1640,"ancestor_record_offset":1292,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":576,"record_offset":1688,"parent_record_offset":1664,"ancestor_record_offset":1292,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":576,"record_offset":1712,"parent_record_offset":1688,"ancestor_record_offset":1292,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":576,"record_offset":1640,"parent_record_offset":1584,"ancestor_record_offset":1292,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":576,"record_offset":1612,"parent_record_offset":1584,"ancestor_record_offset":1292,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":576,"record_offset":1584,"parent_record_offset":1560,"ancestor_record_offset":1292,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":576,"record_offset":1560,"parent_record_offset":1536,"ancestor_record_offset":1292,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":576,"record_offset":1468,"parent_record_offset":1444,"ancestor_record_offset":1292,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00576.jsonl.gz"}
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
// 完整结构及原始类型记录: load_texture.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: load_texture.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: load_texture.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::render::load_texture::create_game_texture
// 信息 ID: daaabf73165299acfe07d09b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_mod_textures::reload_mod_textures @ 0x9e4391 -> 0xb90ab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::load_texture::load_packaged_texture_image @ 0xb910d6 -> 0xb90ab0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::bake_mod_textures::bake_and_apply_mod_texture_sources @ 0xc807c0 -> 0xb90ab0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::load_texture::create_game_texture::closure$0
// 信息 ID: 8ab745f0574d8fdf7c880943
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::load_texture::create_game_texture", "captures": "unknown"}
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::load_texture::create_game_texture @ 0xb90c8f -> 0x2d78f90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::load_texture::load_packaged_texture_image
// 信息 ID: 35b86576061e9f7d5a7c4aba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 11 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::texture_export::export_packaged_textures @ 0xa93396 -> 0xb90ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_textures::reload_packaged_textures_by_name @ 0xa97321 -> 0xb90ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons @ 0xb93c3a -> 0xb90ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::grass2::load_grass2_textures @ 0xc8364f -> 0xb90ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::grass2::load_grass2_textures @ 0xc8381a -> 0xb90ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::grass2::load_grass2_textures @ 0xc838fc -> 0xb90ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::grass2::load_grass2_textures @ 0xc839cd -> 0xb90ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::grass2::load_grass2_textures @ 0xc83aa2 -> 0xb90ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>,country_core::startup::load_textures::load_packaged_textures::closure$0::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures> > > @ 0x1607a00 -> 0xb90ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>,country_core::startup::load_textures::load_packaged_textures::closure$0::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures> > > @ 0x160f850 -> 0xb90ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_clutter_packs::apply_built_clutter_packs @ 0x1adfa32 -> 0xb90ca0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::load_texture::load_packaged_texture_image::closure$0
// 信息 ID: 681959495a422416ccfcbfcf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::load_texture::load_packaged_texture_image", "captures": "unknown"}
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::load_texture::load_packaged_texture_image @ 0xb91214 -> 0x2d790a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
