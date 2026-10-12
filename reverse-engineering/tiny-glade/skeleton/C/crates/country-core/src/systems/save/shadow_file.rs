// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/save/shadow_file.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 24782b2394db3be66260201fbbd22a6893613f5c812762133b285204054b977a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: shadow_file.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 515, 543, 552, 1253, 1643, 1800
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use bitvec; // namespace candidate
//   package identities: [{"name":"bitvec","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use radium; // namespace candidate
//   package identities: [{"name":"radium","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.7.0"}]
// use std; // namespace candidate
//   package identities: []
// use tap; // namespace candidate
//   package identities: [{"name":"tap","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use tempfile; // namespace candidate
//   package identities: [{"name":"tempfile","source":"registry+https://github.com/rust-lang/crates.io-index","version":"3.25.0"}]
// use wyz; // namespace candidate
//   package identities: [{"name":"wyz","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.1"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::shadow_file::ShadowFile::create
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":543,"record_offset":11812,"rva":11012208,"va":5379721328}
// 编译实例 RVAs: 0xa80870
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::backtrace::Backtrace>::capture
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t4937440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379721328,"raw_hex":"e8e0564b00","size":5,"soft_fail":false,"target_va":5384659264,"va":5379721819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4934821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379721328,"raw_hex":"e8a54c4b00","size":5,"soft_fail":false,"target_va":5384656240,"va":5379721414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::to_path_buf
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4931101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379721328,"raw_hex":"e81d3e4b00","size":5,"soft_fail":false,"target_va":5384653728,"va":5379722622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t4887624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379721328,"raw_hex":"e848944a00","size":5,"soft_fail":false,"target_va":5384609408,"va":5379721779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$tempfile..file..TempPath$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2269294","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379721328,"raw_hex":"e86ea02200","size":5,"soft_fail":false,"target_va":5381991760,"va":5379722461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t-4043793","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379721328,"raw_hex":"e8ef4bc2ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379722492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=6
//     target_aliases=3 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10384705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379721328,"raw_hex":"e8bf8a61ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5379723548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009f5b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10361801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379721328,"raw_hex":"e837e461ff","size":5,"soft_fail":false,"target_va":5369361840,"va":5379723636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14044ea20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6498935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379721328,"raw_hex":"e889d59cff","size":5,"soft_fail":false,"target_va":5373225504,"va":5379724434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406805e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4196453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379721328,"raw_hex":"e89bf7bfff","size":5,"soft_fail":false,"target_va":5375526368,"va":5379722816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a0fa80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-462989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379721328,"raw_hex":"e873eff8ff","size":5,"soft_fail":false,"target_va":5379259008,"va":5379721992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f35c30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4935936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379721328,"raw_hex":"e800514b00","size":5,"soft_fail":false,"target_va":5384657968,"va":5379722027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411b1da0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7541843","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379721328,"raw_hex":"e853147300","size":5,"soft_fail":false,"target_va":5387263392,"va":5379721544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd4bb0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37044459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379721328,"raw_hex":"e8eb403502","size":5,"soft_fail":false,"target_va":5416766384,"va":5379721920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4899538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379721328,"raw_hex":"e8d2c24a00","size":5,"soft_fail":false,"target_va":5384624080,"va":5379724537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bitvec::vec::BitVec::resize<usize,bitvec::order::Lsb0>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379721328,"raw_hex":"e842260000","size":5,"soft_fail":false,"target_va":5379732352,"va":5379722553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t*46438666(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5379721328,"raw_hex":"ff150a99c402","size":6,"soft_fail":false,"va":5379722504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":543,"record_offset":12364,"parent_record_offset":11812,"ancestor_record_offset":11812,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] bitvec::vec::traits::impl$24::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec/traits.rs
//     evidence={"module":543,"record_offset":13652,"parent_record_offset":11812,"ancestor_record_offset":11812,"inlinee":33640,"source_line":346,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":543,"record_offset":12196,"parent_record_offset":11812,"ancestor_record_offset":11812,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::div_ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":543,"record_offset":13708,"parent_record_offset":11812,"ancestor_record_offset":11812,"inlinee":10418,"source_line":3657,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":543,"record_offset":12900,"parent_record_offset":11812,"ancestor_record_offset":11812,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":543,"record_offset":12476,"parent_record_offset":11812,"ancestor_record_offset":11812,"inlinee":7484,"source_line":1488,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":543,"record_offset":12252,"parent_record_offset":11812,"ancestor_record_offset":11812,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":543,"record_offset":12112,"parent_record_offset":11812,"ancestor_record_offset":11812,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":543,"record_offset":12000,"parent_record_offset":11812,"ancestor_record_offset":11812,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$59::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":543,"record_offset":13736,"parent_record_offset":11812,"ancestor_record_offset":11812,"inlinee":15680,"source_line":2206,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] tempfile::Builder::make_in
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.25.0/src/lib.rs
//     evidence={"module":543,"record_offset":12084,"parent_record_offset":11812,"ancestor_record_offset":11812,"inlinee":33634,"source_line":739,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] tempfile::Builder::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.25.0/src/lib.rs
//     evidence={"module":543,"record_offset":12028,"parent_record_offset":11812,"ancestor_record_offset":11812,"inlinee":14919,"source_line":307,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":13036,"parent_record_offset":13012,"ancestor_record_offset":11812,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":13060,"parent_record_offset":13036,"ancestor_record_offset":11812,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":13084,"parent_record_offset":13060,"ancestor_record_offset":11812,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":13012,"parent_record_offset":12984,"ancestor_record_offset":11812,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":543,"record_offset":12984,"parent_record_offset":12956,"ancestor_record_offset":11812,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":543,"record_offset":14940,"parent_record_offset":14912,"ancestor_record_offset":11812,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":543,"record_offset":14912,"parent_record_offset":14888,"ancestor_record_offset":11812,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":543,"record_offset":14888,"parent_record_offset":14864,"ancestor_record_offset":11812,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_context
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":543,"record_offset":12436,"parent_record_offset":12396,"ancestor_record_offset":11812,"inlinee":6277,"source_line":245,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":543,"record_offset":12396,"parent_record_offset":12364,"ancestor_record_offset":11812,"inlinee":6326,"source_line":23,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::vec::BitVec::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec/api.rs
//     evidence={"module":543,"record_offset":13676,"parent_record_offset":13652,"ancestor_record_offset":11812,"inlinee":33641,"source_line":64,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":543,"record_offset":12220,"parent_record_offset":12196,"ancestor_record_offset":11812,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=37
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":543,"record_offset":12928,"parent_record_offset":12900,"ancestor_record_offset":11812,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":543,"record_offset":12136,"parent_record_offset":12112,"ancestor_record_offset":11812,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":543,"record_offset":12160,"parent_record_offset":12136,"ancestor_record_offset":11812,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::shadow_file::impl$1::create::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":543,"record_offset":12504,"parent_record_offset":12476,"ancestor_record_offset":11812,"inlinee":33638,"source_line":48,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":543,"record_offset":12300,"parent_record_offset":12276,"ancestor_record_offset":11812,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":543,"record_offset":12276,"parent_record_offset":12252,"ancestor_record_offset":11812,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":543,"record_offset":12324,"parent_record_offset":12300,"ancestor_record_offset":11812,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\os\windows\io\handle.rs
//     evidence={"module":543,"record_offset":13436,"parent_record_offset":13412,"ancestor_record_offset":11812,"inlinee":4731,"source_line":385,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] tempfile::impl$0::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.25.0/src/lib.rs
//     evidence={"module":543,"record_offset":12052,"parent_record_offset":12028,"ancestor_record_offset":11812,"inlinee":14921,"source_line":228,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::shadow_file::ShadowBackendFile::unwrap_open_mut
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x836B
// 首条位置: {"module":543,"inlinee":33643}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::shadow_file::impl$1::create::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x112E7
// 首条位置: {"module":1253,"inlinee":70375}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::fs::OpenOptions::create_new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1253,"record_offset":56304,"parent_record_offset":56188,"ancestor_record_offset":56188,"inlinee":39220,"source_line":1866,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_direct] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1253,"record_offset":56220,"parent_record_offset":56188,"ancestor_record_offset":56188,"inlinee":7535,"source_line":1692,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_direct] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1253,"record_offset":56360,"parent_record_offset":56188,"ancestor_record_offset":56188,"inlinee":7538,"source_line":1915,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_direct] std::fs::OpenOptions::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1253,"record_offset":56248,"parent_record_offset":56188,"ancestor_record_offset":56188,"inlinee":17652,"source_line":1730,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::OpenOptions::create_new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":1253,"record_offset":56328,"parent_record_offset":56304,"ancestor_record_offset":56188,"inlinee":39221,"source_line":230,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::OpenOptions::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":1253,"record_offset":56272,"parent_record_offset":56248,"ancestor_record_offset":56188,"inlinee":7544,"source_line":218,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::shadow_file::impl$1::create::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8366
// 首条位置: {"module":543,"inlinee":33638}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::shadow_file::ShadowFile::write_at
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 93
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x839F
// 首条位置: {"module":1800,"inlinee":33695}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1800,"record_offset":60652,"parent_record_offset":60472,"ancestor_record_offset":60472,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] std::io::cursor::impl$4::seek
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":1800,"record_offset":60508,"parent_record_offset":60472,"ancestor_record_offset":60472,"inlinee":15004,"source_line":293,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1800,"record_offset":60616,"parent_record_offset":60532,"ancestor_record_offset":60472,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_add_signed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":60532,"parent_record_offset":60508,"ancestor_record_offset":60472,"inlinee":14986,"source_line":863,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":60584,"parent_record_offset":60560,"ancestor_record_offset":60472,"inlinee":14988,"source_line":2828,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_add_signed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":60560,"parent_record_offset":60532,"ancestor_record_offset":60472,"inlinee":14987,"source_line":2905,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::shadow_file::ShadowFile::truncate
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 103
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x839E
// 首条位置: {"module":1800,"inlinee":33694}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bitvec::vec::BitVec::truncate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec/api.rs
//     evidence={"module":1800,"record_offset":63540,"parent_record_offset":63388,"ancestor_record_offset":63388,"inlinee":33636,"source_line":359,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::mem::take
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1800,"record_offset":63440,"parent_record_offset":63388,"ancestor_record_offset":63388,"inlinee":4795,"source_line":820,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$9::div_ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1800,"record_offset":63508,"parent_record_offset":63388,"ancestor_record_offset":63388,"inlinee":14756,"source_line":3657,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":63688,"parent_record_offset":63388,"ancestor_record_offset":63388,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":63852,"parent_record_offset":63824,"ancestor_record_offset":63388,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":63880,"parent_record_offset":63852,"ancestor_record_offset":63388,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":63908,"parent_record_offset":63880,"ancestor_record_offset":63388,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1800,"record_offset":63824,"parent_record_offset":63764,"ancestor_record_offset":63388,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":63792,"parent_record_offset":63764,"ancestor_record_offset":63388,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":63764,"parent_record_offset":63736,"ancestor_record_offset":63388,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1800,"record_offset":63736,"parent_record_offset":63712,"ancestor_record_offset":63388,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::span::BitSpan::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/span.rs
//     evidence={"module":1800,"record_offset":63592,"parent_record_offset":63568,"ancestor_record_offset":63388,"inlinee":27267,"source_line":333,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::span::BitSpan::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/span.rs
//     evidence={"module":1800,"record_offset":63648,"parent_record_offset":63624,"ancestor_record_offset":63388,"inlinee":27254,"source_line":350,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::vec::BitVec::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec/api.rs
//     evidence={"module":1800,"record_offset":63568,"parent_record_offset":63540,"ancestor_record_offset":63388,"inlinee":27318,"source_line":774,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::vec::BitVec::set_len_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec.rs
//     evidence={"module":1800,"record_offset":63624,"parent_record_offset":63540,"ancestor_record_offset":63388,"inlinee":27253,"source_line":585,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1800,"record_offset":63468,"parent_record_offset":63440,"ancestor_record_offset":63388,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1800,"record_offset":63712,"parent_record_offset":63688,"ancestor_record_offset":63388,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01800.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::shadow_file::ShadowFile::data
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 112
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8399
// 首条位置: {"module":1800,"inlinee":33689}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::shadow_file::ShadowFile::flush
// 记录形式: compiled_procedure; instances=1
// 源码行: 116
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":543,"record_offset":7076,"rva":11008240,"va":5379717360}
// 编译实例 RVAs: 0xa7f8f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::backtrace::Backtrace>::capture
//     owner=unresolved_namespace; count=9
//     evidence={"assembly":"\tcallq\t4941174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379717360,"raw_hex":"e876654b00","size":5,"soft_fail":false,"target_va":5384659264,"va":5379718085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::File as std::io::Seek>::seek
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5054670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379717360,"raw_hex":"e8ce204d00","size":5,"soft_fail":false,"target_va":5384772320,"va":5379717645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::File as std::io::Write>::write
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5054458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379717360,"raw_hex":"e8fa1f4d00","size":5,"soft_fail":false,"target_va":5384772192,"va":5379717729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::File>::set_len
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4937731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379717360,"raw_hex":"e803584b00","size":5,"soft_fail":false,"target_va":5384657760,"va":5379720024,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4889395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379717360,"raw_hex":"e8339b4a00","size":5,"soft_fail":false,"target_va":5384609408,"va":5379720008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$tempfile..file..TempPath$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2273065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379717360,"raw_hex":"e829af2200","size":5,"soft_fail":false,"target_va":5381991760,"va":5379718690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-4040016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379717360,"raw_hex":"e8b05ac2ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379718715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=10
//     target_aliases=3 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10379025","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379717360,"raw_hex":"e8efa061ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5379717868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009e390
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10363317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379717360,"raw_hex":"e84bde61ff","size":5,"soft_fail":false,"target_va":5369357200,"va":5379720512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009f5b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10359471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379717360,"raw_hex":"e851ed61ff","size":5,"soft_fail":false,"target_va":5369361840,"va":5379721306,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140523d10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5620545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379717360,"raw_hex":"e8bf3caaff","size":5,"soft_fail":false,"target_va":5374098704,"va":5379719244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406805e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4192725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379717360,"raw_hex":"e82b06c0ff","size":5,"soft_fail":false,"target_va":5375526368,"va":5379719088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a0fa80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-459219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379717360,"raw_hex":"e82dfef8ff","size":5,"soft_fail":false,"target_va":5379259008,"va":5379718222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f35c30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4939417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379717360,"raw_hex":"e8995e4b00","size":5,"soft_fail":false,"target_va":5384657968,"va":5379718546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411b1da0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7545334","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379717360,"raw_hex":"e8f6217300","size":5,"soft_fail":false,"target_va":5387263392,"va":5379718053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd4bb0
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37048217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379717360,"raw_hex":"e8994f3502","size":5,"soft_fail":false,"target_va":5416766384,"va":5379718162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd50e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37048059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379717360,"raw_hex":"e8fb4e3502","size":5,"soft_fail":false,"target_va":5416767712,"va":5379719648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4903844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379717360,"raw_hex":"e8a4d34a00","size":5,"soft_fail":false,"target_va":5384624080,"va":5379720231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bitvec::slice::specialization::lsb0::_$LT$impl$u20$bitvec..slice..BitSlice$LT$T$GT$$GT$::sp_first_one
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16505647","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379717360,"raw_hex":"e82fdbfb00","size":5,"soft_fail":false,"target_va":5396223120,"va":5379717468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::systems::save::shadow_file::ShadowBackendFile> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379717360,"raw_hex":"e85f330000","size":5,"soft_fail":false,"target_va":5379732272,"va":5379719116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t36902012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379717360,"raw_hex":"e87c143302","size":5,"soft_fail":false,"target_va":5416622320,"va":5379720303,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*46442446(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5379717360,"raw_hex":"ff15cea7c402","size":6,"soft_fail":false,"va":5379718724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":543,"record_offset":11560,"parent_record_offset":7076,"ancestor_record_offset":7076,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":543,"record_offset":11628,"parent_record_offset":7076,"ancestor_record_offset":7076,"inlinee":5676,"source_line":593,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::shadow_file::ShadowFile::flush_impl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":543,"record_offset":7228,"parent_record_offset":7076,"ancestor_record_offset":7076,"inlinee":33629,"source_line":121,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":10012,"parent_record_offset":9988,"ancestor_record_offset":7076,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":10036,"parent_record_offset":10012,"ancestor_record_offset":7076,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":10060,"parent_record_offset":10036,"ancestor_record_offset":7076,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":9988,"parent_record_offset":9960,"ancestor_record_offset":7076,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":543,"record_offset":9960,"parent_record_offset":9932,"ancestor_record_offset":7076,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_context
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":543,"record_offset":7720,"parent_record_offset":7672,"ancestor_record_offset":7076,"inlinee":6277,"source_line":245,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":543,"record_offset":7672,"parent_record_offset":7636,"ancestor_record_offset":7076,"inlinee":6326,"source_line":23,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":543,"record_offset":7636,"parent_record_offset":7500,"ancestor_record_offset":7076,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::span::BitSpan::len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/span.rs
//     evidence={"module":543,"record_offset":10908,"parent_record_offset":10884,"ancestor_record_offset":7076,"inlinee":27267,"source_line":333,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::span::BitSpan::set_len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/span.rs
//     evidence={"module":543,"record_offset":10964,"parent_record_offset":10940,"ancestor_record_offset":7076,"inlinee":27254,"source_line":350,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::slice::iter::impl$15::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/slice/iter.rs
//     evidence={"module":543,"record_offset":7460,"parent_record_offset":7228,"ancestor_record_offset":7076,"inlinee":33631,"source_line":2158,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::vec::BitVec::clear
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec/api.rs
//     evidence={"module":543,"record_offset":10832,"parent_record_offset":7228,"ancestor_record_offset":7076,"inlinee":33635,"source_line":759,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::vec::BitVec::len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec/api.rs
//     evidence={"module":543,"record_offset":10884,"parent_record_offset":10856,"ancestor_record_offset":7076,"inlinee":27318,"source_line":774,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::vec::BitVec::set_len_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec.rs
//     evidence={"module":543,"record_offset":10940,"parent_record_offset":10856,"ancestor_record_offset":7076,"inlinee":27253,"source_line":585,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::vec::BitVec::truncate
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec/api.rs
//     evidence={"module":543,"record_offset":10856,"parent_record_offset":10832,"ancestor_record_offset":7076,"inlinee":33636,"source_line":359,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":543,"record_offset":7760,"parent_record_offset":7500,"ancestor_record_offset":7076,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":543,"record_offset":10776,"parent_record_offset":7228,"ancestor_record_offset":7076,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":543,"record_offset":7604,"parent_record_offset":7580,"ancestor_record_offset":7076,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$1::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":543,"record_offset":8524,"parent_record_offset":8496,"ancestor_record_offset":7076,"inlinee":14985,"source_line":730,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":543,"record_offset":7856,"parent_record_offset":7820,"ancestor_record_offset":7076,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::Range::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":543,"record_offset":7580,"parent_record_offset":7500,"ancestor_record_offset":7076,"inlinee":33612,"source_line":151,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":543,"record_offset":10800,"parent_record_offset":10776,"ancestor_record_offset":7076,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":543,"record_offset":9876,"parent_record_offset":7228,"ancestor_record_offset":7076,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":543,"record_offset":8680,"parent_record_offset":7228,"ancestor_record_offset":7076,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":543,"record_offset":7884,"parent_record_offset":7820,"ancestor_record_offset":7076,"inlinee":6161,"source_line":82,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":543,"record_offset":7788,"parent_record_offset":7500,"ancestor_record_offset":7076,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":543,"record_offset":7820,"parent_record_offset":7788,"ancestor_record_offset":7076,"inlinee":6157,"source_line":435,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":543,"record_offset":8236,"parent_record_offset":8208,"ancestor_record_offset":7076,"inlinee":6159,"source_line":567,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":543,"record_offset":10716,"parent_record_offset":10692,"ancestor_record_offset":7076,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":543,"record_offset":10740,"parent_record_offset":10716,"ancestor_record_offset":7076,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::save::shadow_file::impl$1::flush_impl::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":543,"record_offset":7500,"parent_record_offset":7228,"ancestor_record_offset":7076,"inlinee":33633,"source_line":157,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":543,"record_offset":11196,"parent_record_offset":11172,"ancestor_record_offset":7076,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":543,"record_offset":11148,"parent_record_offset":7228,"ancestor_record_offset":7076,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":543,"record_offset":11172,"parent_record_offset":11148,"ancestor_record_offset":7076,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":543,"record_offset":11220,"parent_record_offset":11196,"ancestor_record_offset":7076,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":543,"record_offset":10692,"parent_record_offset":7228,"ancestor_record_offset":7076,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] std::io::Write::write_all
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\mod.rs
//     evidence={"module":543,"record_offset":7920,"parent_record_offset":7500,"ancestor_record_offset":7076,"inlinee":8737,"source_line":1875,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::Error::is_interrupted
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error.rs
//     evidence={"module":543,"record_offset":7984,"parent_record_offset":7920,"ancestor_record_offset":7076,"inlinee":8738,"source_line":1015,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::impl$21::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error.rs
//     evidence={"module":543,"record_offset":8120,"parent_record_offset":7984,"ancestor_record_offset":7076,"inlinee":8741,"source_line":230,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::Repr::data
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":543,"record_offset":8024,"parent_record_offset":7984,"ancestor_record_offset":7076,"inlinee":8739,"source_line":208,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":543,"record_offset":8052,"parent_record_offset":8024,"ancestor_record_offset":7076,"inlinee":5225,"source_line":244,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::kind_from_prim
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":543,"record_offset":8084,"parent_record_offset":8052,"ancestor_record_offset":7076,"inlinee":8742,"source_line":290,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\os\windows\io\handle.rs
//     evidence={"module":543,"record_offset":10384,"parent_record_offset":10360,"ancestor_record_offset":7076,"inlinee":4731,"source_line":385,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] tempfile::Builder::make_in
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.25.0/src/lib.rs
//     evidence={"module":543,"record_offset":8496,"parent_record_offset":7228,"ancestor_record_offset":7076,"inlinee":33634,"source_line":739,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] tempfile::Builder::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.25.0/src/lib.rs
//     evidence={"module":543,"record_offset":8440,"parent_record_offset":7228,"ancestor_record_offset":7076,"inlinee":14919,"source_line":307,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] tempfile::impl$0::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.25.0/src/lib.rs
//     evidence={"module":543,"record_offset":8464,"parent_record_offset":8440,"ancestor_record_offset":7076,"inlinee":14921,"source_line":228,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::shadow_file::ShadowFile::flush_impl
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 121
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x835D
// 首条位置: {"module":543,"inlinee":33629}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":543,"record_offset":8556,"parent_record_offset":7228,"ancestor_record_offset":7228,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] bitvec::slice::iter::impl$15::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/slice/iter.rs
//     evidence={"module":543,"record_offset":7460,"parent_record_offset":7228,"ancestor_record_offset":7228,"inlinee":33631,"source_line":2158,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] bitvec::vec::BitVec::clear
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec/api.rs
//     evidence={"module":543,"record_offset":10832,"parent_record_offset":7228,"ancestor_record_offset":7228,"inlinee":33635,"source_line":759,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":543,"record_offset":10776,"parent_record_offset":7228,"ancestor_record_offset":7228,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":543,"record_offset":9876,"parent_record_offset":7228,"ancestor_record_offset":7228,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":543,"record_offset":8680,"parent_record_offset":7228,"ancestor_record_offset":7228,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::shadow_file::impl$1::flush_impl::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":543,"record_offset":7500,"parent_record_offset":7228,"ancestor_record_offset":7228,"inlinee":33633,"source_line":157,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":543,"record_offset":11148,"parent_record_offset":7228,"ancestor_record_offset":7228,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":543,"record_offset":10692,"parent_record_offset":7228,"ancestor_record_offset":7228,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] tempfile::Builder::make_in
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.25.0/src/lib.rs
//     evidence={"module":543,"record_offset":8496,"parent_record_offset":7228,"ancestor_record_offset":7228,"inlinee":33634,"source_line":739,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] tempfile::Builder::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.25.0/src/lib.rs
//     evidence={"module":543,"record_offset":8440,"parent_record_offset":7228,"ancestor_record_offset":7228,"inlinee":14919,"source_line":307,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":10012,"parent_record_offset":9988,"ancestor_record_offset":7228,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":10036,"parent_record_offset":10012,"ancestor_record_offset":7228,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":10060,"parent_record_offset":10036,"ancestor_record_offset":7228,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":543,"record_offset":9988,"parent_record_offset":9960,"ancestor_record_offset":7228,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":543,"record_offset":9960,"parent_record_offset":9932,"ancestor_record_offset":7228,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_context
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":543,"record_offset":7720,"parent_record_offset":7672,"ancestor_record_offset":7228,"inlinee":6277,"source_line":245,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":543,"record_offset":7672,"parent_record_offset":7636,"ancestor_record_offset":7228,"inlinee":6326,"source_line":23,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":543,"record_offset":7636,"parent_record_offset":7500,"ancestor_record_offset":7228,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::span::BitSpan::len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/span.rs
//     evidence={"module":543,"record_offset":10908,"parent_record_offset":10884,"ancestor_record_offset":7228,"inlinee":27267,"source_line":333,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::span::BitSpan::set_len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/span.rs
//     evidence={"module":543,"record_offset":10964,"parent_record_offset":10940,"ancestor_record_offset":7228,"inlinee":27254,"source_line":350,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::vec::BitVec::len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec/api.rs
//     evidence={"module":543,"record_offset":10884,"parent_record_offset":10856,"ancestor_record_offset":7228,"inlinee":27318,"source_line":774,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::vec::BitVec::set_len_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec.rs
//     evidence={"module":543,"record_offset":10940,"parent_record_offset":10856,"ancestor_record_offset":7228,"inlinee":27253,"source_line":585,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::vec::BitVec::truncate
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec/api.rs
//     evidence={"module":543,"record_offset":10856,"parent_record_offset":10832,"ancestor_record_offset":7228,"inlinee":33636,"source_line":359,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":543,"record_offset":7760,"parent_record_offset":7500,"ancestor_record_offset":7228,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":543,"record_offset":7604,"parent_record_offset":7580,"ancestor_record_offset":7228,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$1::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":543,"record_offset":8524,"parent_record_offset":8496,"ancestor_record_offset":7228,"inlinee":14985,"source_line":730,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":543,"record_offset":7856,"parent_record_offset":7820,"ancestor_record_offset":7228,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::Range::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":543,"record_offset":7580,"parent_record_offset":7500,"ancestor_record_offset":7228,"inlinee":33612,"source_line":151,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":543,"record_offset":10800,"parent_record_offset":10776,"ancestor_record_offset":7228,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":543,"record_offset":9904,"parent_record_offset":9876,"ancestor_record_offset":7228,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":543,"record_offset":7884,"parent_record_offset":7820,"ancestor_record_offset":7228,"inlinee":6161,"source_line":82,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":543,"record_offset":7788,"parent_record_offset":7500,"ancestor_record_offset":7228,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":543,"record_offset":7820,"parent_record_offset":7788,"ancestor_record_offset":7228,"inlinee":6157,"source_line":435,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":543,"record_offset":8236,"parent_record_offset":8208,"ancestor_record_offset":7228,"inlinee":6159,"source_line":567,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":543,"record_offset":10716,"parent_record_offset":10692,"ancestor_record_offset":7228,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":543,"record_offset":10740,"parent_record_offset":10716,"ancestor_record_offset":7228,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":543,"record_offset":11196,"parent_record_offset":11172,"ancestor_record_offset":7228,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":543,"record_offset":11172,"parent_record_offset":11148,"ancestor_record_offset":7228,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":543,"record_offset":11220,"parent_record_offset":11196,"ancestor_record_offset":7228,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] std::io::Write::write_all
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\mod.rs
//     evidence={"module":543,"record_offset":7920,"parent_record_offset":7500,"ancestor_record_offset":7228,"inlinee":8737,"source_line":1875,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::Error::is_interrupted
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error.rs
//     evidence={"module":543,"record_offset":7984,"parent_record_offset":7920,"ancestor_record_offset":7228,"inlinee":8738,"source_line":1015,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::impl$21::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error.rs
//     evidence={"module":543,"record_offset":8120,"parent_record_offset":7984,"ancestor_record_offset":7228,"inlinee":8741,"source_line":230,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::Repr::data
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":543,"record_offset":8024,"parent_record_offset":7984,"ancestor_record_offset":7228,"inlinee":8739,"source_line":208,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":543,"record_offset":8052,"parent_record_offset":8024,"ancestor_record_offset":7228,"inlinee":5225,"source_line":244,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::kind_from_prim
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":543,"record_offset":8084,"parent_record_offset":8052,"ancestor_record_offset":7228,"inlinee":8742,"source_line":290,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\os\windows\io\handle.rs
//     evidence={"module":543,"record_offset":10384,"parent_record_offset":10360,"ancestor_record_offset":7228,"inlinee":4731,"source_line":385,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] tempfile::impl$0::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tempfile","version":"3.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tempfile-3.25.0/src/lib.rs
//     evidence={"module":543,"record_offset":8464,"parent_record_offset":8440,"ancestor_record_offset":7228,"inlinee":14921,"source_line":228,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::shadow_file::impl$1::flush_impl::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 157
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8361
// 首条位置: {"module":543,"inlinee":33633}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":543,"record_offset":7636,"parent_record_offset":7500,"ancestor_record_offset":7500,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":543,"record_offset":7760,"parent_record_offset":7500,"ancestor_record_offset":7500,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":543,"record_offset":7580,"parent_record_offset":7500,"ancestor_record_offset":7500,"inlinee":33612,"source_line":151,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":543,"record_offset":7788,"parent_record_offset":7500,"ancestor_record_offset":7500,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] std::io::Write::write_all
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\mod.rs
//     evidence={"module":543,"record_offset":7920,"parent_record_offset":7500,"ancestor_record_offset":7500,"inlinee":8737,"source_line":1875,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_context
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":543,"record_offset":7720,"parent_record_offset":7672,"ancestor_record_offset":7500,"inlinee":6277,"source_line":245,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":543,"record_offset":7672,"parent_record_offset":7636,"ancestor_record_offset":7500,"inlinee":6326,"source_line":23,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":543,"record_offset":7604,"parent_record_offset":7580,"ancestor_record_offset":7500,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":543,"record_offset":7856,"parent_record_offset":7820,"ancestor_record_offset":7500,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":543,"record_offset":7884,"parent_record_offset":7820,"ancestor_record_offset":7500,"inlinee":6161,"source_line":82,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":543,"record_offset":8208,"parent_record_offset":7920,"ancestor_record_offset":7500,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":543,"record_offset":7820,"parent_record_offset":7788,"ancestor_record_offset":7500,"inlinee":6157,"source_line":435,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":543,"record_offset":8236,"parent_record_offset":8208,"ancestor_record_offset":7500,"inlinee":6159,"source_line":567,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::Error::is_interrupted
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error.rs
//     evidence={"module":543,"record_offset":7984,"parent_record_offset":7920,"ancestor_record_offset":7500,"inlinee":8738,"source_line":1015,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::impl$21::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error.rs
//     evidence={"module":543,"record_offset":8120,"parent_record_offset":7984,"ancestor_record_offset":7500,"inlinee":8741,"source_line":230,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::Repr::data
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":543,"record_offset":8024,"parent_record_offset":7984,"ancestor_record_offset":7500,"inlinee":8739,"source_line":208,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":543,"record_offset":8052,"parent_record_offset":8024,"ancestor_record_offset":7500,"inlinee":5225,"source_line":244,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::kind_from_prim
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":543,"record_offset":8084,"parent_record_offset":8052,"ancestor_record_offset":7500,"inlinee":8742,"source_line":290,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::shadow_file::ShadowFile::sync_all
// 记录形式: compiled_procedure; instances=1
// 源码行: 211
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":543,"record_offset":16152,"rva":11015600,"va":5379724720}
// 编译实例 RVAs: 0xa815b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::backtrace::Backtrace>::capture
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4934317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379724720,"raw_hex":"e8ad4a4b00","size":5,"soft_fail":false,"target_va":5384659264,"va":5379724942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4884523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379724720,"raw_hex":"e82b884a00","size":5,"soft_fail":false,"target_va":5384609408,"va":5379724880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10386330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379724720,"raw_hex":"e8668461ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5379725173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f35c30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4933046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379724720,"raw_hex":"e8b6454b00","size":5,"soft_fail":false,"target_va":5384657968,"va":5379724917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd4bb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37041372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379724720,"raw_hex":"e8dc343502","size":5,"soft_fail":false,"target_va":5416766384,"va":5379725007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4899010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379724720,"raw_hex":"e8c2c04a00","size":5,"soft_fail":false,"target_va":5384624080,"va":5379725065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36897117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379724720,"raw_hex":"e85d013302","size":5,"soft_fail":false,"target_va":5416622240,"va":5379725118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::shadow_file::ShadowFile::flush
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379724720,"raw_hex":"e893e2ffff","size":5,"soft_fail":false,"target_va":5379717360,"va":5379724888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":543,"record_offset":16608,"parent_record_offset":16152,"ancestor_record_offset":16152,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":543,"record_offset":16408,"parent_record_offset":16152,"ancestor_record_offset":16152,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":543,"record_offset":16728,"parent_record_offset":16152,"ancestor_record_offset":16152,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":543,"record_offset":16788,"parent_record_offset":16152,"ancestor_record_offset":16152,"inlinee":5676,"source_line":593,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::save::shadow_file::ShadowBackendFile::unwrap_open_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":543,"record_offset":16572,"parent_record_offset":16152,"ancestor_record_offset":16152,"inlinee":33643,"source_line":24,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":543,"record_offset":16464,"parent_record_offset":16152,"ancestor_record_offset":16152,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":543,"record_offset":16324,"parent_record_offset":16152,"ancestor_record_offset":16152,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":543,"record_offset":16688,"parent_record_offset":16640,"ancestor_record_offset":16152,"inlinee":6277,"source_line":245,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":543,"record_offset":16640,"parent_record_offset":16608,"ancestor_record_offset":16152,"inlinee":6326,"source_line":23,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":543,"record_offset":16432,"parent_record_offset":16408,"ancestor_record_offset":16152,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":543,"record_offset":16756,"parent_record_offset":16728,"ancestor_record_offset":16152,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":543,"record_offset":16348,"parent_record_offset":16324,"ancestor_record_offset":16152,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":543,"record_offset":16372,"parent_record_offset":16348,"ancestor_record_offset":16152,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":543,"record_offset":16512,"parent_record_offset":16488,"ancestor_record_offset":16152,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":543,"record_offset":16488,"parent_record_offset":16464,"ancestor_record_offset":16152,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":543,"record_offset":16536,"parent_record_offset":16512,"ancestor_record_offset":16152,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::shadow_file::ShadowFile::mark_dirty
// 记录形式: compiled_procedure; instances=1
// 源码行: 227
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":543,"record_offset":4944,"rva":11007472,"va":5379716592}
// 编译实例 RVAs: 0xa7f5f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4892528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379716592,"raw_hex":"e870a74a00","size":5,"soft_fail":false,"target_va":5384609408,"va":5379716875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shadow_file.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37339846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379716592,"raw_hex":"e8c6c23902","size":5,"soft_fail":false,"target_va":5417057088,"va":5379717237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bitvec::vec::BitVec::resize<usize,bitvec::order::Lsb0>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t15438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379716592,"raw_hex":"e84e3c0000","size":5,"soft_fail":false,"target_va":5379732352,"va":5379716909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36904891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379716592,"raw_hex":"e8bb1f3302","size":5,"soft_fail":false,"target_va":5416622240,"va":5379717344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%r10","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5379716592,"raw_hex":"41ffd2","size":3,"soft_fail":false,"va":5379717180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bitvec::slice::BitSlice::fill
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/slice/api.rs
//     evidence={"module":543,"record_offset":6224,"parent_record_offset":4944,"ancestor_record_offset":4944,"inlinee":33625,"source_line":2129,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] bitvec::vec::BitVec::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec/api.rs
//     evidence={"module":543,"record_offset":5456,"parent_record_offset":4944,"ancestor_record_offset":4944,"inlinee":27318,"source_line":774,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] bitvec::vec::ops::impl$16::index_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/vec/ops.rs
//     evidence={"module":543,"record_offset":5512,"parent_record_offset":4944,"ancestor_record_offset":4944,"inlinee":33614,"source_line":249,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":543,"record_offset":5288,"parent_record_offset":4944,"ancestor_record_offset":4944,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$9::div_ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":543,"record_offset":5172,"parent_record_offset":4944,"ancestor_record_offset":4944,"inlinee":14756,"source_line":3657,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":543,"record_offset":5120,"parent_record_offset":4944,"ancestor_record_offset":4944,"inlinee":33612,"source_line":151,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":543,"record_offset":5344,"parent_record_offset":4944,"ancestor_record_offset":4944,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":543,"record_offset":5204,"parent_record_offset":4944,"ancestor_record_offset":4944,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::access::BitAccess::set_bits
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/access.rs
//     evidence={"module":543,"record_offset":6616,"parent_record_offset":6592,"ancestor_record_offset":4944,"inlinee":27361,"source_line":72,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::domain::Domain::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/domain.rs
//     evidence={"module":543,"record_offset":6300,"parent_record_offset":6276,"ancestor_record_offset":4944,"inlinee":27353,"source_line":377,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::domain::PartialElement::store_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/domain.rs
//     evidence={"module":543,"record_offset":6592,"parent_record_offset":6224,"ancestor_record_offset":4944,"inlinee":27357,"source_line":827,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::index::BitEnd::span
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/index.rs
//     evidence={"module":543,"record_offset":6464,"parent_record_offset":6436,"ancestor_record_offset":4944,"inlinee":27343,"source_line":545,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::index::BitIdx::offset
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/index.rs
//     evidence={"module":543,"record_offset":5880,"parent_record_offset":5852,"ancestor_record_offset":4944,"inlinee":27269,"source_line":259,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::index::BitIdx::span
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/index.rs
//     evidence={"module":543,"record_offset":6436,"parent_record_offset":6372,"ancestor_record_offset":4944,"inlinee":27341,"source_line":295,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::index::impl$53::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/index.rs
//     evidence={"module":543,"record_offset":6524,"parent_record_offset":6300,"ancestor_record_offset":4944,"inlinee":27355,"source_line":409,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::mem::elts
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/mem.rs
//     evidence={"module":543,"record_offset":6408,"parent_record_offset":6372,"ancestor_record_offset":4944,"inlinee":27249,"source_line":51,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::single::BitPtr::offset
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/single.rs
//     evidence={"module":543,"record_offset":5852,"parent_record_offset":5784,"ancestor_record_offset":4944,"inlinee":27268,"source_line":545,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::single::BitPtr::span_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/single.rs
//     evidence={"module":543,"record_offset":6084,"parent_record_offset":5760,"ancestor_record_offset":4944,"inlinee":27282,"source_line":245,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::span::BitSpan::address
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/span.rs
//     evidence={"module":543,"record_offset":5816,"parent_record_offset":5784,"ancestor_record_offset":4944,"inlinee":4818,"source_line":251,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::span::BitSpan::head
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/span.rs
//     evidence={"module":543,"record_offset":6372,"parent_record_offset":6300,"ancestor_record_offset":4944,"inlinee":27252,"source_line":293,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::span::BitSpan::len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/span.rs
//     evidence={"module":543,"record_offset":5480,"parent_record_offset":5456,"ancestor_record_offset":4944,"inlinee":27267,"source_line":333,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::span::BitSpan::new_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/span.rs
//     evidence={"module":543,"record_offset":6108,"parent_record_offset":6084,"ancestor_record_offset":4944,"inlinee":27283,"source_line":200,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::ptr::span::BitSpan::to_bitptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/ptr/span.rs
//     evidence={"module":543,"record_offset":5784,"parent_record_offset":5760,"ancestor_record_offset":4944,"inlinee":27266,"source_line":532,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::slice::BitSlice::domain_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/slice.rs
//     evidence={"module":543,"record_offset":6276,"parent_record_offset":6224,"ancestor_record_offset":4944,"inlinee":27351,"source_line":1038,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::slice::BitSlice::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/slice/api.rs
//     evidence={"module":543,"record_offset":5616,"parent_record_offset":5576,"ancestor_record_offset":4944,"inlinee":27275,"source_line":87,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::slice::api::impl$4::get_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/slice/api.rs
//     evidence={"module":543,"record_offset":5680,"parent_record_offset":5576,"ancestor_record_offset":4944,"inlinee":33619,"source_line":2637,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::slice::api::impl$4::get_mut::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/slice/api.rs
//     evidence={"module":543,"record_offset":5708,"parent_record_offset":5680,"ancestor_record_offset":4944,"inlinee":33621,"source_line":2682,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::slice::api::impl$4::get_unchecked_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/slice/api.rs
//     evidence={"module":543,"record_offset":5736,"parent_record_offset":5680,"ancestor_record_offset":4944,"inlinee":33622,"source_line":2654,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::slice::api::impl$4::get_unchecked_mut::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/slice/api.rs
//     evidence={"module":543,"record_offset":5760,"parent_record_offset":5736,"ancestor_record_offset":4944,"inlinee":33624,"source_line":2687,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::slice::api::impl$4::index_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/slice/api.rs
//     evidence={"module":543,"record_offset":5576,"parent_record_offset":5544,"ancestor_record_offset":4944,"inlinee":33618,"source_line":2670,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::slice::api::impl$4::index_mut::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/slice/api.rs
//     evidence={"module":543,"record_offset":6180,"parent_record_offset":6156,"ancestor_record_offset":4944,"inlinee":33627,"source_line":2673,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::slice::ops::impl$6::index_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/slice/ops.rs
//     evidence={"module":543,"record_offset":5544,"parent_record_offset":5512,"ancestor_record_offset":4944,"inlinee":33616,"source_line":190,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] bitvec::store::impl$12::store_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitvec","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitvec-1.0.1/src/store.rs
//     evidence={"module":543,"record_offset":6900,"parent_record_offset":6224,"ancestor_record_offset":4944,"inlinee":27363,"source_line":121,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":543,"record_offset":6664,"parent_record_offset":6640,"ancestor_record_offset":4944,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$66::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":543,"record_offset":5144,"parent_record_offset":5120,"ancestor_record_offset":4944,"inlinee":5649,"source_line":1916,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":543,"record_offset":6688,"parent_record_offset":6664,"ancestor_record_offset":4944,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$5::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\int_macros.rs
//     evidence={"module":543,"record_offset":5904,"parent_record_offset":5880,"ancestor_record_offset":4944,"inlinee":27270,"source_line":2100,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":543,"record_offset":5312,"parent_record_offset":5288,"ancestor_record_offset":4944,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":543,"record_offset":6156,"parent_record_offset":5576,"ancestor_record_offset":4944,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":543,"record_offset":6860,"parent_record_offset":6836,"ancestor_record_offset":4944,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":543,"record_offset":6032,"parent_record_offset":6008,"ancestor_record_offset":4944,"inlinee":7420,"source_line":352,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":543,"record_offset":6756,"parent_record_offset":6732,"ancestor_record_offset":4944,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":543,"record_offset":6812,"parent_record_offset":6788,"ancestor_record_offset":4944,"inlinee":6145,"source_line":1060,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":543,"record_offset":6836,"parent_record_offset":6812,"ancestor_record_offset":4944,"inlinee":6147,"source_line":221,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":543,"record_offset":6732,"parent_record_offset":6224,"ancestor_record_offset":4944,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$2::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":543,"record_offset":6788,"parent_record_offset":6224,"ancestor_record_offset":4944,"inlinee":19670,"source_line":35,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":543,"record_offset":5228,"parent_record_offset":5204,"ancestor_record_offset":4944,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":543,"record_offset":5252,"parent_record_offset":5228,"ancestor_record_offset":4944,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":543,"record_offset":5392,"parent_record_offset":5368,"ancestor_record_offset":4944,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":543,"record_offset":5368,"parent_record_offset":5344,"ancestor_record_offset":4944,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":543,"record_offset":5416,"parent_record_offset":5392,"ancestor_record_offset":4944,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] radium::impl$32::fetch_or
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"radium","version":"0.7.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/radium-0.7.0/src/lib.rs
//     evidence={"module":543,"record_offset":6640,"parent_record_offset":6616,"ancestor_record_offset":4944,"inlinee":14493,"source_line":563,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] tap::pipe::Pipe::pipe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"tap","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tap-1.0.1/src/pipe.rs
//     evidence={"module":543,"record_offset":5984,"parent_record_offset":5960,"ancestor_record_offset":4944,"inlinee":13907,"source_line":73,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] wyz::comu::Address::offset
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"wyz","version":"0.5.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wyz-0.5.1/src/comu.rs
//     evidence={"module":543,"record_offset":5936,"parent_record_offset":5852,"ancestor_record_offset":4944,"inlinee":27271,"source_line":256,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] wyz::comu::Address::with_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"wyz","version":"0.5.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wyz-0.5.1/src/comu.rs
//     evidence={"module":543,"record_offset":5960,"parent_record_offset":5936,"ancestor_record_offset":4944,"inlinee":16744,"source_line":328,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   [pdb_inline_transitive] wyz::comu::impl$10::offset::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"wyz","version":"0.5.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wyz-0.5.1/src/comu.rs
//     evidence={"module":543,"record_offset":6008,"parent_record_offset":5984,"ancestor_record_offset":4944,"inlinee":16735,"source_line":259,"archive":"_pdb/C/modules/00543.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::shadow_file::ShadowFile::path
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 247
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x839A
// 首条位置: {"module":1643,"inlinee":33690}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::shadow_file::impl$2::drop
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 257
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x75A6
// 首条位置: {"module":515,"inlinee":30118}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::shadow_file::ShadowFileWriter::write_all
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 281
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x83A1
// 首条位置: {"module":552,"inlinee":33697}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::save::shadow_file::ShadowFileWriter::stream_position
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/save/shadow_file.rs
//     evidence={"module":552,"record_offset":9408,"parent_record_offset":8552,"ancestor_record_offset":8552,"inlinee":33645,"source_line":289,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_direct] std::io::cursor::impl$9::write_all
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":552,"record_offset":8584,"parent_record_offset":8552,"ancestor_record_offset":8552,"inlinee":29134,"source_line":674,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":552,"record_offset":8740,"parent_record_offset":8716,"ancestor_record_offset":8552,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":552,"record_offset":9096,"parent_record_offset":9064,"ancestor_record_offset":8552,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":552,"record_offset":8884,"parent_record_offset":8856,"ancestor_record_offset":8552,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":552,"record_offset":8764,"parent_record_offset":8740,"ancestor_record_offset":8552,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":552,"record_offset":8940,"parent_record_offset":8912,"ancestor_record_offset":8552,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":552,"record_offset":9144,"parent_record_offset":9120,"ancestor_record_offset":8552,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":552,"record_offset":9120,"parent_record_offset":9096,"ancestor_record_offset":8552,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":552,"record_offset":8912,"parent_record_offset":8884,"ancestor_record_offset":8552,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":552,"record_offset":9064,"parent_record_offset":9040,"ancestor_record_offset":8552,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::capacity
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":552,"record_offset":8716,"parent_record_offset":8660,"ancestor_record_offset":8552,"inlinee":6564,"source_line":1421,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":552,"record_offset":8828,"parent_record_offset":8660,"ancestor_record_offset":8552,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":552,"record_offset":8856,"parent_record_offset":8660,"ancestor_record_offset":8552,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":552,"record_offset":9244,"parent_record_offset":8660,"ancestor_record_offset":8552,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::spare_capacity_mut
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":552,"record_offset":9040,"parent_record_offset":8660,"ancestor_record_offset":8552,"inlinee":9137,"source_line":3195,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":552,"record_offset":8800,"parent_record_offset":8660,"ancestor_record_offset":8552,"inlinee":7142,"source_line":2300,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":552,"record_offset":8964,"parent_record_offset":8940,"ancestor_record_offset":8552,"inlinee":4931,"source_line":2493,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":552,"record_offset":9332,"parent_record_offset":9304,"ancestor_record_offset":8552,"inlinee":4801,"source_line":623,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":552,"record_offset":11192,"parent_record_offset":11168,"ancestor_record_offset":10432,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":552,"record_offset":9304,"parent_record_offset":9276,"ancestor_record_offset":8552,"inlinee":9139,"source_line":1359,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::fill
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":552,"record_offset":9188,"parent_record_offset":8660,"ancestor_record_offset":8552,"inlinee":8428,"source_line":4166,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::specialize::impl$1::spec_fill
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\specialize.rs
//     evidence={"module":552,"record_offset":9212,"parent_record_offset":9188,"ancestor_record_offset":8552,"inlinee":7122,"source_line":21,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::reserve_and_pad
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":552,"record_offset":8660,"parent_record_offset":8612,"ancestor_record_offset":8552,"inlinee":9136,"source_line":464,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write_all
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":552,"record_offset":8612,"parent_record_offset":8584,"ancestor_record_offset":8552,"inlinee":9135,"source_line":528,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::vec_write_all_unchecked
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\cursor.rs
//     evidence={"module":552,"record_offset":9276,"parent_record_offset":8612,"ancestor_record_offset":8552,"inlinee":9138,"source_line":510,"archive":"_pdb/C/modules/00552.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::save::shadow_file::ShadowFileWriter::stream_position
// 记录形式: inline_instance, inlinee_declaration; instances=11
// 源码行: 289
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x836D
// 首条位置: {"module":543,"inlinee":33645}
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
// 完整结构及原始类型记录: shadow_file.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::save::shadow_file::ShadowBackendFile
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "enum2$<country_core::systems::save::shadow_file::ShadowBackendFile>", "constructor": "enum2$", "arguments": ["country_core::systems::save::shadow_file::ShadowBackendFile"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<country_core::systems::save::shadow_file::ShadowBackendFile> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<country_core::systems::save::shadow_file::ShadowBackendFile>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::save::shadow_file::ShadowFile
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 11; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::save::shadow_file::ShadowFile>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::save::shadow_file::ShadowFile"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<country_core::systems::save::shadow_file::ShadowFile,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["country_core::systems::save::shadow_file::ShadowFile", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec::retain_mut<country_core::systems::save::shadow_file::ShadowFile,alloc::alloc::Global,country_core::systems::save::save_thread::save_thread::closure_env$0>", "constructor": "alloc::vec::Vec::retain_mut", "arguments": ["country_core::systems::save::shadow_file::ShadowFile", "alloc::alloc::Global", "country_core::systems::save::save_thread::save_thread::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::systems::save::shadow_file::ShadowFile,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::systems::save::shadow_file::ShadowFile", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::save::shadow_file::ShadowFile,alloc::alloc::Global> >", "constructor": "lock_api::mutex::MutexGuard", "arguments": ["parking_lot::raw_mutex::RawMutex", "alloc::vec::Vec<country_core::systems::save::shadow_file::ShadowFile,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::save::shadow_file::ShadowFile,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::save::shadow_file::ShadowFile,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::save::shadow_file::ShadowFileWriter
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 0
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: shadow_file.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: shadow_file.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 90; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<country_core::resources::walls::plaster::Plaster> >,tuple$<bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::PlasterInstanceData> >,bevy_ecs::query::filter::Or<tuple$<bevy_ecs::query::filter::Changed<country_core::resources::walls::plaster::Plaster>,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniActiveInstance> > > > >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::change_detection::Res<country_core::resources::walls::plaster::AnimatedPlasterPeel>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<country_core::resources::walls::plaster::Plaster> >,tuple$<bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::PlasterInstanceData> >,bevy_ecs::query::filter::Or<tuple$<bevy_ecs::query::filter::Changed<country_core::resources::walls::plaster::Plaster>,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniActiveInstance> > > > >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::change_detection::Res<country_core::resources::walls::plaster::AnimatedPlasterPeel>"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::PlasterInstanceData> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<country_core::resources::walls::plaster::Plaster> >,tuple$<bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::PlasterInstanceData> >,bevy_ecs::query::filter::Or<tuple$<bevy_ecs::query::filter::Changed<country_core::resources::walls::plaster::Plaster>,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniActiveInstance> > > > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::plaster::AnimatedPlasterPeel>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>,bevy_ecs::system::system_param::Local<country_core::systems::terrain::delayed_terrain_changed_msg::DelayedUpdateWallsMsgBuffer>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::delayed_terrain_changed_msg::OnTerrainChange>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::VisualTerrainHeightSsbo> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>", "bevy_ecs::system::system_param::Local<country_core::systems::terrain::delayed_terrain_changed_msg::DelayedUpdateWallsMsgBuffer>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::delayed_terrain_changed_msg::OnTerrainChange>", "bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::VisualTerrainHeightSsbo> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::terrain::delayed_terrain_changed_msg::DelayedUpdateWallsMsgBuffer>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::delayed_terrain_changed_msg::OnTerrainChange>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::VisualTerrainHeightSsbo> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::signifiers::signifier_create::GizmoPresence>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::system::query::Query<ref$<country_core::systems::ui::signifiers::SignifierId>,bevy_ecs::query::filter::Without<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> >,bevy_ecs::change_detection::ResMut<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::signifiers::signifier_create::GizmoPresence>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::system::commands::Commands", "bevy_ecs::system::query::Query<ref$<country_core::systems::ui::signifiers::SignifierId>,bevy_ecs::query::filter::Without<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> >", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::signifiers::signifier_create::GizmoPresence>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::systems::ui::signifiers::SignifierMarkedForDeletion>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::systems::ui::signifiers::SignifierId>,bevy_ecs::query::filter::Without<country_core::systems::ui::signifiers::SignifierMarkedForDeletion> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<country_core::components::SemiTransparent> > >,enum2$<core::option::Option<ref$<country_core::components::FlatLighting> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::AlbedoMap> > > >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::UberInstanceData> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::nani::uber_materials::UberMaterialPool>,bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<country_core::components::SemiTransparent> > >,enum2$<core::option::Option<ref$<country_core::components::FlatLighting> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::AlbedoMap> > > >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::UberInstanceData> > >", "bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::Res<country_core::render::nani::uber_materials::UberMaterialPool>", "bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::UberInstanceData> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<country_core::components::SemiTransparent> > >,enum2$<core::option::Option<ref$<country_core::components::FlatLighting> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::AlbedoMap> > > >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::UberInstanceData> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::nani::uber_materials::UberMaterialPool>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::prefab::SpawnPrefabCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_reader::MessageReader<country_core::systems::prefab::DespawnPrefabCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::prefab::PrefabsToDelete>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::prefab::SpawnPrefabCmd>", "bevy_ecs::system::commands::Commands", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::prefab::DespawnPrefabCmd>", "bevy_ecs::change_detection::ResMut<country_core::systems::prefab::PrefabsToDelete>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::prefab::PrefabLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::prefab::SpawnPrefabCmd>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::prefab::DespawnPrefabCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::prefab::PrefabsToDelete>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_shape::Roof> > >,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofAnimation> > >,enum2$<core::option::Option<ref$<country_core::resources::walls::Floor> > > >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::StoneFloorInstanceData> > >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds>,bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_shape::Roof> > >,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofAnimation> > >,enum2$<core::option::Option<ref$<country_core::resources::walls::Floor> > > >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::StoneFloorInstanceData> > >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>", "bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds>", "bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::StoneFloorInstanceData> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_shape::Roof> > >,enum2$<core::option::Option<ref$<country_core::resources::roofs::roof_animation::RoofAnimation> > >,enum2$<core::option::Option<ref$<country_core::resources::walls::Floor> > > >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::StoneFloorInstanceData> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::color_id::StoneFoundationColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::drawable::NaniActiveInstance> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<country_core::components::drawable::NaniActiveInstance> >,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniInstanceDesc> >,bevy_ecs::lifecycle::RemovedComponents<country_core::components::drawable::NaniActiveInstance>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::system::system_param::Local<country_core::render::nani::systems::NaniActiveInstanceCollection>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::drawable::NaniActiveInstance> >", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<country_core::components::drawable::NaniActiveInstance> >,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniInstanceDesc> >", "bevy_ecs::lifecycle::RemovedComponents<country_core::components::drawable::NaniActiveInstance>", "bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::system::system_param::Local<country_core::render::nani::systems::NaniActiveInstanceCollection>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::components::drawable::NaniActiveInstance>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::drawable::NaniActiveInstance> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<country_core::components::drawable::NaniActiveInstance> >,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniInstanceDesc> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::render::nani::systems::NaniActiveInstanceCollection>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::shadow_file::ShadowBackendFile::unwrap_open_mut
// 信息 ID: dfd1563e9d7cd7afcb091466
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::shadow_file::ShadowFile::sync_all; module=543, inline_record=16572; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::shadow_file::ShadowFile::create
// 信息 ID: a73e93bd3e38cb61f0042a9b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::save_thread::execute_save_cmd @ 0x1b73b9f -> 0xa80870 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::shadow_file::ShadowFile::data
// 信息 ID: 67da80bf272e9e28919770e7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_thread::CurrentSaveFile::change_section; module=1800, inline_record=19944; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_thread::execute_save_cmd; module=1800, inline_record=48764; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_thread::execute_save_cmd; module=1800, inline_record=71076; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_thread::execute_save_cmd; module=1800, inline_record=74160; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::shadow_file::ShadowFile::flush
// 信息 ID: 7f303b625740c3d2587a297f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::shadow_file::ShadowFile::sync_all @ 0xa81658 -> 0xa7f8f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::shadow_file::ShadowFile::flush_impl
// 信息 ID: 76a70a5f87a99550f407c9c3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::shadow_file::ShadowFile::flush; module=543, inline_record=7228; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::shadow_file::ShadowFile::mark_dirty
// 信息 ID: 1fabec059203c1498a791ef7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::save_game::add_history_edit_to_save_file @ 0xacd844 -> 0xa7f5f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::save_game::add_history_edit_to_save_file @ 0xacd93b -> 0xa7f5f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::save_thread::CurrentSaveFile::change_section @ 0x1b6f6e9 -> 0xa7f5f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::save_thread::CurrentSaveFile::change_section @ 0x1b6f8f9 -> 0xa7f5f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::shadow_file::ShadowFile::path
// 信息 ID: ee92b94596625d0e3ed56234
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_thread::save_thread::closure$0; module=1643, inline_record=145132; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_thread::save_thread; module=1800, inline_record=2888; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_thread::UnsavedFileList::contains_file; module=1800, inline_record=34584; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_thread::execute_save_cmd; module=1800, inline_record=73748; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::shadow_file::ShadowFile::sync_all
// 信息 ID: 672ac68c451d739a21dcfd08
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ptr::drop_in_place<country_core::systems::save::shadow_file::ShadowFile> @ 0x99a729 -> 0xa815b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::save_thread::save_thread::closure$0 @ 0x15f04a6 -> 0xa815b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::save_thread::save_thread @ 0x1b6e0e6 -> 0xa815b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::save_thread::CurrentSaveFile::flush_deferred_and_sync @ 0x1b70374 -> 0xa815b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::shadow_file::ShadowFile::truncate
// 信息 ID: e05bd97c368dc7d1e2d3443f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_thread::execute_save_cmd; module=1800, inline_record=63388; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::shadow_file::ShadowFile::write_at
// 信息 ID: 361051d93ab2e17508af9115
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_thread::execute_save_cmd; module=1800, inline_record=60472; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::shadow_file::ShadowFileWriter::stream_position
// 信息 ID: 95ec8d2943497c5ed4d14f4a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_game::add_history_edit_to_save_file; module=552, inline_record=8320; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_game::add_history_edit_to_save_file; module=552, inline_record=9408; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_game::add_history_edit_to_save_file; module=552, inline_record=11292; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_thread::CurrentSaveFile::change_section; module=1800, inline_record=22448; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_thread::execute_save_cmd; module=1800, inline_record=62492; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_thread::execute_save_cmd; module=1800, inline_record=63360; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::shadow_file::ShadowFileWriter::write_all
// 信息 ID: 03366418d95c4fedb27d3ae2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::save_game::add_history_edit_to_save_file; module=552, inline_record=8552; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_game::add_history_edit_to_save_file; module=552, inline_record=10432; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_thread::CurrentSaveFile::change_section; module=1800, inline_record=21532; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_thread::CurrentSaveFile::change_section; module=1800, inline_record=22412; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::save_thread::execute_save_cmd; module=1800, inline_record=62452; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::shadow_file::impl$1::create::closure$0
// 信息 ID: caf93e7e779328373b45871d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::save::shadow_file::impl$1::create", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::save::shadow_file::impl$1::create::closure_env$0 <- tempfile::util::create_helper<tempfile::file::NamedTempFile<std::fs::File>,tempfile::impl$1::make_in::closure_env$0<country_core::systems::save::shadow_file::impl$1::create::closure_env$0,std::fs::File,ref$<std::path::Path> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] tempfile::util::create_helper<tempfile::file::NamedTempFile<std::fs::File>,tempfile::impl$1::make_in::closure_env$0<country_core::systems::save::shadow_file::impl$1::create::closure_env$0,std::fs::File,ref$<std::path::Path> > >; module=1253, inline_record=56188; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::shadow_file::impl$1::create::closure$1
// 信息 ID: 57ce4ee220de5365e9ece1d3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::save::shadow_file::impl$1::create", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::shadow_file::ShadowFile::create; module=543, inline_record=12504; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::shadow_file::impl$1::flush_impl::closure$1
// 信息 ID: 0edcb7853c3e08052fa8a5c5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::save::shadow_file::impl$1::flush_impl", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::shadow_file::ShadowFile::flush; module=543, inline_record=7500; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::save::shadow_file::ShadowFile::flush; module=543, inline_record=8820; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::save::shadow_file::impl$2::drop
// 信息 ID: 65e9b6dfb1397f8a4dcedb40
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ptr::drop_in_place<country_core::systems::save::shadow_file::ShadowFile>; module=515, inline_record=1395640; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
