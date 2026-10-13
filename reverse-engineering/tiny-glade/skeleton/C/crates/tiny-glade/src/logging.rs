// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/logging.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: c268ec6855b24e77415396256748ac57114b08dd2c80dc5ce934de3de58d3fcb
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: logging.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 242
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use fern; // namespace candidate
//   package identities: [{"name":"fern","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.6.1"}]
// use std; // namespace candidate
//   package identities: []
// use time; // namespace candidate
//   package identities: [{"name":"time","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.3.36"}]
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::logging::formatted_date_time
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4503
// 首条位置: {"module":242,"inlinee":17667}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":242,"record_offset":34616,"parent_record_offset":34528,"ancestor_record_offset":34528,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_direct] time::offset_date_time::OffsetDateTime::format
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/time-0.3.36/src/offset_date_time.rs
//     evidence={"module":242,"record_offset":34588,"parent_record_offset":34528,"ancestor_record_offset":34528,"inlinee":13552,"source_line":1271,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":242,"record_offset":34652,"parent_record_offset":34616,"ancestor_record_offset":34528,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::logging::formatted_time
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4504
// 首条位置: {"module":242,"inlinee":17668}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":242,"record_offset":36504,"parent_record_offset":36424,"ancestor_record_offset":36424,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_direct] time::offset_date_time::OffsetDateTime::format
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/time-0.3.36/src/offset_date_time.rs
//     evidence={"module":242,"record_offset":36476,"parent_record_offset":36424,"ancestor_record_offset":36424,"inlinee":13552,"source_line":1271,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":242,"record_offset":36544,"parent_record_offset":36504,"ancestor_record_offset":36424,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::logging::set_up_logging
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x44FE
// 首条位置: {"module":242,"inlinee":17662}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":242,"record_offset":22232,"parent_record_offset":19488,"ancestor_record_offset":19488,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":242,"record_offset":19664,"parent_record_offset":19488,"ancestor_record_offset":19488,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":242,"record_offset":21632,"parent_record_offset":19488,"ancestor_record_offset":19488,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_direct] fern::builders::Dispatch::chain
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"fern","version":"0.6.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fern-0.6.1/src/builders.rs
//     evidence={"module":242,"record_offset":20016,"parent_record_offset":19488,"ancestor_record_offset":19488,"inlinee":17650,"source_line":198,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_direct] fern::builders::Dispatch::format
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fern","version":"0.6.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fern-0.6.1/src/builders.rs
//     evidence={"module":242,"record_offset":19748,"parent_record_offset":19488,"ancestor_record_offset":19488,"inlinee":17658,"source_line":171,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_direct] fern::builders::Dispatch::level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fern","version":"0.6.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fern-0.6.1/src/builders.rs
//     evidence={"module":242,"record_offset":19988,"parent_record_offset":19488,"ancestor_record_offset":19488,"inlinee":17649,"source_line":224,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_direct] fern::builders::Dispatch::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"fern","version":"0.6.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fern-0.6.1/src/builders.rs
//     evidence={"module":242,"record_offset":19720,"parent_record_offset":19488,"ancestor_record_offset":19488,"inlinee":17648,"source_line":127,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_direct] fern::colors::ColoredLevelConfig::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fern","version":"0.6.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fern-0.6.1/src/colors.rs
//     evidence={"module":242,"record_offset":19692,"parent_record_offset":19488,"ancestor_record_offset":19488,"inlinee":17664,"source_line":158,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_direct] std::fs::OpenOptions::create
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":242,"record_offset":20720,"parent_record_offset":19488,"ancestor_record_offset":19488,"inlinee":17653,"source_line":1828,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_direct] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":242,"record_offset":20636,"parent_record_offset":19488,"ancestor_record_offset":19488,"inlinee":7535,"source_line":1692,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_direct] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":242,"record_offset":20860,"parent_record_offset":19488,"ancestor_record_offset":19488,"inlinee":7538,"source_line":1915,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_direct] std::fs::OpenOptions::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":242,"record_offset":20776,"parent_record_offset":19488,"ancestor_record_offset":19488,"inlinee":17654,"source_line":1803,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_direct] std::fs::OpenOptions::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":242,"record_offset":20664,"parent_record_offset":19488,"ancestor_record_offset":19488,"inlinee":17652,"source_line":1730,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":242,"record_offset":20832,"parent_record_offset":19488,"ancestor_record_offset":19488,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":242,"record_offset":19884,"parent_record_offset":19860,"ancestor_record_offset":19488,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":242,"record_offset":19908,"parent_record_offset":19884,"ancestor_record_offset":19488,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":242,"record_offset":21140,"parent_record_offset":21116,"ancestor_record_offset":19488,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":242,"record_offset":21164,"parent_record_offset":21140,"ancestor_record_offset":19488,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":242,"record_offset":19932,"parent_record_offset":19908,"ancestor_record_offset":19488,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":242,"record_offset":21188,"parent_record_offset":21164,"ancestor_record_offset":19488,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":242,"record_offset":19860,"parent_record_offset":19828,"ancestor_record_offset":19488,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":242,"record_offset":21116,"parent_record_offset":21060,"ancestor_record_offset":19488,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":242,"record_offset":19828,"parent_record_offset":19792,"ancestor_record_offset":19488,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":242,"record_offset":19792,"parent_record_offset":19748,"ancestor_record_offset":19488,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":242,"record_offset":20164,"parent_record_offset":20132,"ancestor_record_offset":19488,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":242,"record_offset":21088,"parent_record_offset":21060,"ancestor_record_offset":19488,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":242,"record_offset":21060,"parent_record_offset":21036,"ancestor_record_offset":19488,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":242,"record_offset":20212,"parent_record_offset":20188,"ancestor_record_offset":19488,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":242,"record_offset":20188,"parent_record_offset":20164,"ancestor_record_offset":19488,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":242,"record_offset":21036,"parent_record_offset":21012,"ancestor_record_offset":19488,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":242,"record_offset":20132,"parent_record_offset":20088,"ancestor_record_offset":19488,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":242,"record_offset":20060,"parent_record_offset":20016,"ancestor_record_offset":19488,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":242,"record_offset":20088,"parent_record_offset":20060,"ancestor_record_offset":19488,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":242,"record_offset":20892,"parent_record_offset":20860,"ancestor_record_offset":19488,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":242,"record_offset":20252,"parent_record_offset":20088,"ancestor_record_offset":19488,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::OpenOptions::create
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":242,"record_offset":20744,"parent_record_offset":20720,"ancestor_record_offset":19488,"inlinee":7545,"source_line":227,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::OpenOptions::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":242,"record_offset":20800,"parent_record_offset":20776,"ancestor_record_offset":19488,"inlinee":7546,"source_line":224,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::fs::windows::OpenOptions::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\fs\windows.rs
//     evidence={"module":242,"record_offset":20688,"parent_record_offset":20664,"ancestor_record_offset":19488,"inlinee":7544,"source_line":218,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::logging::set_up_logging::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":242,"record_offset":36284,"rva":6576624,"va":5375285744}
// 编译实例 RVAs: 0x6459f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t392471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375285744,"raw_hex":"e817fd0500","size":5,"soft_fail":false,"target_va":5375678704,"va":5375286228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: logging.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5947558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375285744,"raw_hex":"e85a3fa5ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5375286401,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ef9c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: logging.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-352451","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375285744,"raw_hex":"e83d9ffaff","size":5,"soft_fail":false,"target_va":5374933440,"va":5375285886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t41335328","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375285744,"raw_hex":"e820ba7602","size":5,"soft_fail":false,"target_va":5416621632,"va":5375286299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fern::log_impl::FormatCallback::finish
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"fern","version":"0.6.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t867264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375285744,"raw_hex":"e8c03b0d00","size":5,"soft_fail":false,"target_va":5376153472,"va":5375286203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] time::offset_date_time::OffsetDateTime::now_utc
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8101066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375285744,"raw_hex":"e8ca9c7b00","size":5,"soft_fail":false,"target_va":5383386864,"va":5375285793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] time::offset_date_time::OffsetDateTime::to_offset
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8101131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375285744,"raw_hex":"e80b9d7b00","size":5,"soft_fail":false,"target_va":5383386976,"va":5375285840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":242,"record_offset":36756,"parent_record_offset":36284,"ancestor_record_offset":36284,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_direct] fern::colors::ColoredLevelConfig::color
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fern","version":"0.6.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fern-0.6.1/src/colors.rs
//     evidence={"module":242,"record_offset":36680,"parent_record_offset":36284,"ancestor_record_offset":36284,"inlinee":17669,"source_line":230,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::logging::formatted_time
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/logging.rs
//     evidence={"module":242,"record_offset":36424,"parent_record_offset":36284,"ancestor_record_offset":36284,"inlinee":17668,"source_line":30,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":242,"record_offset":36932,"parent_record_offset":36908,"ancestor_record_offset":36284,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":242,"record_offset":36956,"parent_record_offset":36932,"ancestor_record_offset":36284,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":242,"record_offset":36980,"parent_record_offset":36956,"ancestor_record_offset":36284,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":242,"record_offset":36908,"parent_record_offset":36852,"ancestor_record_offset":36284,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":242,"record_offset":36880,"parent_record_offset":36852,"ancestor_record_offset":36284,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":242,"record_offset":36852,"parent_record_offset":36828,"ancestor_record_offset":36284,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":242,"record_offset":36828,"parent_record_offset":36804,"ancestor_record_offset":36284,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":242,"record_offset":36544,"parent_record_offset":36504,"ancestor_record_offset":36284,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":242,"record_offset":36504,"parent_record_offset":36424,"ancestor_record_offset":36284,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] fern::colors::ColoredLevelConfig::get_color
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fern","version":"0.6.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fern-0.6.1/src/colors.rs
//     evidence={"module":242,"record_offset":36708,"parent_record_offset":36680,"ancestor_record_offset":36284,"inlinee":17670,"source_line":235,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] time::offset_date_time::OffsetDateTime::format
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/time-0.3.36/src/offset_date_time.rs
//     evidence={"module":242,"record_offset":36476,"parent_record_offset":36424,"ancestor_record_offset":36284,"inlinee":13552,"source_line":1271,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::logging::set_up_logging::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 93
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":242,"record_offset":34384,"rva":6575808,"va":5375284928}
// 编译实例 RVAs: 0x6456c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t393326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375284928,"raw_hex":"e86e000600","size":5,"soft_fail":false,"target_va":5375678704,"va":5375285373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: logging.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5946708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375284928,"raw_hex":"e8ac42a5ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5375285551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ef9c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: logging.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-351627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375284928,"raw_hex":"e875a2faff","size":5,"soft_fail":false,"target_va":5374933440,"va":5375285062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9509511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375284928,"raw_hex":"e8871a9100","size":5,"soft_fail":false,"target_va":5384794864,"va":5375285348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t41336503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375284928,"raw_hex":"e8b7be7602","size":5,"soft_fail":false,"target_va":5416621632,"va":5375285124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fern::log_impl::FormatCallback::finish
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"fern","version":"0.6.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t868051","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375284928,"raw_hex":"e8d33e0d00","size":5,"soft_fail":false,"target_va":5376153472,"va":5375285416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] time::offset_date_time::OffsetDateTime::now_utc
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8101890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375284928,"raw_hex":"e802a07b00","size":5,"soft_fail":false,"target_va":5383386864,"va":5375284969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] time::offset_date_time::OffsetDateTime::to_offset
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8101955","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375284928,"raw_hex":"e843a07b00","size":5,"soft_fail":false,"target_va":5383386976,"va":5375285016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":242,"record_offset":34816,"parent_record_offset":34384,"ancestor_record_offset":34384,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":242,"record_offset":35184,"parent_record_offset":34384,"ancestor_record_offset":34384,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":242,"record_offset":34900,"parent_record_offset":34384,"ancestor_record_offset":34384,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::logging::formatted_date_time
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/logging.rs
//     evidence={"module":242,"record_offset":34528,"parent_record_offset":34384,"ancestor_record_offset":34384,"inlinee":17667,"source_line":11,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":242,"record_offset":35076,"parent_record_offset":35052,"ancestor_record_offset":34384,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":242,"record_offset":35100,"parent_record_offset":35076,"ancestor_record_offset":34384,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":242,"record_offset":35124,"parent_record_offset":35100,"ancestor_record_offset":34384,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":242,"record_offset":35052,"parent_record_offset":34996,"ancestor_record_offset":34384,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":242,"record_offset":34864,"parent_record_offset":34840,"ancestor_record_offset":34384,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":242,"record_offset":35024,"parent_record_offset":34996,"ancestor_record_offset":34384,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":242,"record_offset":34996,"parent_record_offset":34972,"ancestor_record_offset":34384,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":242,"record_offset":34972,"parent_record_offset":34948,"ancestor_record_offset":34384,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":242,"record_offset":34840,"parent_record_offset":34816,"ancestor_record_offset":34384,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":242,"record_offset":34652,"parent_record_offset":34616,"ancestor_record_offset":34384,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":242,"record_offset":34616,"parent_record_offset":34528,"ancestor_record_offset":34384,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] time::offset_date_time::OffsetDateTime::format
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/time-0.3.36/src/offset_date_time.rs
//     evidence={"module":242,"record_offset":34588,"parent_record_offset":34528,"ancestor_record_offset":34384,"inlinee":13552,"source_line":1271,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::logging::set_up_logging::closure$2
// 记录形式: compiled_procedure; instances=1
// 源码行: 128
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":242,"record_offset":36068,"rva":47653232,"va":5416362352}
// 编译实例 RVAs: 0x2d72170
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d66e80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: logging.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-45868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416362352,"raw_hex":"e8d44cffff","size":5,"soft_fail":false,"target_va":5416316544,"va":5416362407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-31567540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416362352,"raw_hex":"e84c511efe","size":5,"soft_fail":false,"target_va":5384794864,"va":5416362399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":242,"record_offset":36200,"parent_record_offset":36068,"ancestor_record_offset":36068,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":242,"record_offset":36248,"parent_record_offset":36224,"ancestor_record_offset":36068,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00242.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":242,"record_offset":36224,"parent_record_offset":36200,"ancestor_record_offset":36068,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00242.jsonl.gz"}
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
// 完整结构及原始类型记录: logging.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: logging.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: logging.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: tiny_glade::logging::formatted_date_time
// 信息 ID: ea22e129904e8ab5db0f066c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::logging::set_up_logging::closure$1; module=242, inline_record=34528; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::logging::formatted_time
// 信息 ID: 3757202bf51498cd9d18dc81
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::logging::set_up_logging::closure$0; module=242, inline_record=36424; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::logging::set_up_logging
// 信息 ID: 94f7a964cbcee039a66f7d66
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::main; module=242, inline_record=19488; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::logging::set_up_logging::closure$0
// 信息 ID: cfb029c53ba51d1ff919fb16
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::logging::set_up_logging", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::logging::set_up_logging::closure_env$0 <- core::ops::function::FnOnce::call_once<tiny_glade::logging::set_up_logging::closure_env$0,tuple$<fern::log_impl::FormatCallback,ref$<core::fmt::Arguments>,ref$<log::Record> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::logging::set_up_logging::closure_env$0,tuple$<fern::log_impl::FormatCallback,ref$<core::fmt::Arguments>,ref$<log::Record> > > @ 0x646687 -> 0x6459f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::logging::set_up_logging::closure$1
// 信息 ID: 310921c0c5673e5929d34691
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::logging::set_up_logging", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::logging::set_up_logging::closure_env$1 <- core::ops::function::FnOnce::call_once<tiny_glade::logging::set_up_logging::closure_env$1,tuple$<fern::log_impl::FormatCallback,ref$<core::fmt::Arguments>,ref$<log::Record> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::logging::set_up_logging::closure_env$1,tuple$<fern::log_impl::FormatCallback,ref$<core::fmt::Arguments>,ref$<log::Record> > > @ 0x6466c4 -> 0x6456c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::logging::set_up_logging::closure$2
// 信息 ID: 81176604420498a1eb7ec1b7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::logging::set_up_logging", "captures": "unknown"}
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::main @ 0x6433ac -> 0x2d72170 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
