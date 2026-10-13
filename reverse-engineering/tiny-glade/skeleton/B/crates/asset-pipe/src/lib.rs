// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/asset-pipe/src/lib.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 5fe89e5c2b8caa9a6ddccfdb5fc9f121dcf66e6200f9613ec0b98d0a270e32d0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 3166, 3171, 3176, 3179, 4139, 4140, 4150, 4152, 4153, 4158, 4159, 4600, 4601, 4609, 4610, 4756, 4757
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use asset; // namespace candidate
//   package identities: [{"name":"asset","source":"local_or_path","version":"0.1.0"}]
// use asset_pipe; // namespace candidate
//   package identities: [{"name":"asset-pipe","source":"local_or_path","version":"0.1.0"}]
// use byteorder; // namespace candidate
//   package identities: [{"name":"byteorder","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.5.0"}]
// use core; // namespace candidate
//   package identities: []
// use ddsfile; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"},{"name":"ddsfile","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.1"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use image; // namespace candidate
//   package identities: [{"name":"image","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.24.9"},{"name":"image","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.25.1"}]
// use png; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"},{"name":"png","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.6"}]
// use relative_path; // namespace candidate
//   package identities: [{"name":"relative-path","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.9.0"}]
// use ron; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"2.9.0"},{"name":"ron","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968","version":"0.8.1"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.200"}]
// use std; // namespace candidate
//   package identities: []
// use unicode_ident; // namespace candidate
//   package identities: [{"name":"unicode-ident","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.18"}]
// use zune_core; // namespace candidate
//   package identities: [{"name":"zune-core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.12"}]
// use zune_jpeg; // namespace candidate
//   package identities: [{"name":"zune-jpeg","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.11"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: ddsfile::header::Header::read<ref_mut$<ref_mut$<std::io::cursor::Cursor<alloc::vec::Vec<u8,alloc::alloc::Global> > > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4153,"record_offset":256,"rva":38127760,"va":5406836880}
// 编译实例 RVAs: 0x245c890
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31755890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406836880,"raw_hex":"e88e711bfe","size":5,"soft_fail":false,"target_va":5375081488,"va":5406837373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4702730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406836880,"raw_hex":"e80ac24700","size":5,"soft_fail":false,"target_va":5411540963,"va":5406838228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ddsfile::format::pixel_format::PixelFormat::read
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ddsfile","version":"0.5.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2334882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406836880,"raw_hex":"e8a2a02300","size":5,"soft_fail":false,"target_va":5409172416,"va":5406837529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":4153,"record_offset":5080,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/04153.jsonl.gz"}
//   [pdb_inline_direct] byteorder::io::ReadBytesExt::read_u32
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"byteorder","version":"1.5.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/byteorder-1.5.0/src/io.rs
//     evidence={"module":4153,"record_offset":484,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":21404,"source_line":217,"archive":"_pdb/B/modules/04153.jsonl.gz"}
//   [pdb_inline_direct] byteorder::io::ReadBytesExt::read_u32_into
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"byteorder","version":"1.5.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/byteorder-1.5.0/src/io.rs
//     evidence={"module":4153,"record_offset":4468,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":125534,"source_line":597,"archive":"_pdb/B/modules/04153.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4153,"record_offset":5552,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04153.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4153,"record_offset":1056,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04153.jsonl.gz"}
//   [pdb_inline_direct] ddsfile::header::Caps2::from_bits_truncate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":4153,"record_offset":8656,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":125538,"source_line":563,"archive":"_pdb/B/modules/04153.jsonl.gz"}
//   [pdb_inline_direct] ddsfile::header::Caps::from_bits_truncate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":4153,"record_offset":8624,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":125536,"source_line":563,"archive":"_pdb/B/modules/04153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4153,"record_offset":5396,"parent_record_offset":5372,"ancestor_record_offset":256,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/04153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4153,"record_offset":5420,"parent_record_offset":5396,"ancestor_record_offset":256,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/04153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4153,"record_offset":5372,"parent_record_offset":5344,"ancestor_record_offset":256,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/04153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4153,"record_offset":736,"parent_record_offset":712,"ancestor_record_offset":256,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/04153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4153,"record_offset":5284,"parent_record_offset":5256,"ancestor_record_offset":256,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/04153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4153,"record_offset":784,"parent_record_offset":760,"ancestor_record_offset":256,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/04153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4153,"record_offset":760,"parent_record_offset":736,"ancestor_record_offset":256,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/04153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4153,"record_offset":5344,"parent_record_offset":5312,"ancestor_record_offset":256,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/04153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4153,"record_offset":5312,"parent_record_offset":5284,"ancestor_record_offset":256,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/04153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4153,"record_offset":5220,"parent_record_offset":5192,"ancestor_record_offset":256,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/04153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4153,"record_offset":5192,"parent_record_offset":5164,"ancestor_record_offset":256,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/04153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4153,"record_offset":5136,"parent_record_offset":5108,"ancestor_record_offset":256,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/04153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4153,"record_offset":5164,"parent_record_offset":5136,"ancestor_record_offset":256,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/04153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4153,"record_offset":5108,"parent_record_offset":5080,"ancestor_record_offset":256,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/04153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4153,"record_offset":712,"parent_record_offset":684,"ancestor_record_offset":256,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/04153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4153,"record_offset":684,"parent_record_offset":656,"ancestor_record_offset":256,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/04153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4153,"record_offset":5256,"parent_record_offset":5220,"ancestor_record_offset":256,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/04153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$30::as_ref
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4153,"record_offset":628,"parent_record_offset":596,"ancestor_record_offset":256,"inlinee":4954,"source_line":3825,"archive":"_pdb/B/modules/04153.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4153,"record_offset":656,"parent_record_offset":628,"ancestor_record_offset":256,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/04153.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4153,"record_offset":836,"parent_record_offset":596,"ancestor_record_offset":256,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/04153.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":4153,"record_offset":1004,"parent_record_offset":980,"ancestor_record_offset":256,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/04153.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":4153,"record_offset":5472,"parent_record_offset":5220,"ancestor_record_offset":256,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/04153.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4153,"record_offset":980,"parent_record_offset":952,"ancestor_record_offset":256,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/04153.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4153,"record_offset":864,"parent_record_offset":596,"ancestor_record_offset":256,"inlinee":6334,"source_line":1916,"archive":"_pdb/B/modules/04153.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_checked
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4153,"record_offset":888,"parent_record_offset":864,"ancestor_record_offset":256,"inlinee":6335,"source_line":2115,"archive":"_pdb/B/modules/04153.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_unchecked
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4153,"record_offset":912,"parent_record_offset":888,"ancestor_record_offset":256,"inlinee":6336,"source_line":2001,"archive":"_pdb/B/modules/04153.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::Cursor::split
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":4153,"record_offset":596,"parent_record_offset":568,"ancestor_record_offset":256,"inlinee":21422,"source_line":233,"archive":"_pdb/B/modules/04153.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$5::read_exact
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\cursor.rs
//     evidence={"module":4153,"record_offset":568,"parent_record_offset":540,"ancestor_record_offset":256,"inlinee":21967,"source_line":360,"archive":"_pdb/B/modules/04153.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$0::read_exact
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":4153,"record_offset":512,"parent_record_offset":484,"ancestor_record_offset":256,"inlinee":21651,"source_line":45,"archive":"_pdb/B/modules/04153.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$8::read_exact
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":4153,"record_offset":952,"parent_record_offset":568,"ancestor_record_offset":256,"inlinee":20267,"source_line":347,"archive":"_pdb/B/modules/04153.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: image::codecs::png::PngDecoder::with_limits<std::io::buffered::bufreader::BufReader<std::fs::File> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4158,"record_offset":1032,"rva":38134048,"va":5406843168}
// 编译实例 RVAs: 0x245e120
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140030a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-37935128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406843168,"raw_hex":"e8e827bdfd","size":5,"soft_fail":false,"target_va":5368908400,"va":5406843523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d8b70
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32005460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406843168,"raw_hex":"e8aca217fe","size":5,"soft_fail":false,"target_va":5374839664,"va":5406845119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140603eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31828436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406843168,"raw_hex":"e82c561afe","size":5,"soft_fail":false,"target_va":5375016624,"va":5406845055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31761762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406843168,"raw_hex":"e89e5a1bfe","size":5,"soft_fail":false,"target_va":5375081488,"va":5406843245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140878300
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29254458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406843168,"raw_hex":"e8c69c41fe","size":5,"soft_fail":false,"target_va":5377590016,"va":5406844469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140878720
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29252942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406843168,"raw_hex":"e8b2a241fe","size":5,"soft_fail":false,"target_va":5377591072,"va":5406844009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142698b90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2336356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406843168,"raw_hex":"e864a62300","size":5,"soft_fail":false,"target_va":5409180560,"va":5406844199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142699b50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2340796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406843168,"raw_hex":"e8bcb72300","size":5,"soft_fail":false,"target_va":5409184592,"va":5406843791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5003714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406843168,"raw_hex":"e8c2594c00","size":5,"soft_fail":false,"target_va":5411847168,"va":5406843449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4695950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406843168,"raw_hex":"e88ea74700","size":5,"soft_fail":false,"target_va":5411540963,"va":5406845008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::codecs::png::_$LT$impl$u20$image..error..ImageError$GT$::from_png
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"image","version":"0.24.9","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-25942659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406843168,"raw_hex":"e87d2574fe","size":5,"soft_fail":false,"target_va":5380900944,"va":5406843598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] png::decoder::stream::StreamingDecoder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-25837278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406843168,"raw_hex":"e822c175fe","size":5,"soft_fail":false,"target_va":5381006064,"va":5406843337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4158,"record_offset":1240,"parent_record_offset":1032,"ancestor_record_offset":1032,"inlinee":4666,"source_line":998,"archive":"_pdb/B/modules/04158.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4158,"record_offset":1660,"parent_record_offset":1032,"ancestor_record_offset":1032,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/04158.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4158,"record_offset":1776,"parent_record_offset":1032,"ancestor_record_offset":1032,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04158.jsonl.gz"}
//   [pdb_inline_direct] image::io::Limits::check_dimensions
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/io/mod.rs
//     evidence={"module":4158,"record_offset":1740,"parent_record_offset":1032,"ancestor_record_offset":1032,"inlinee":5190,"source_line":79,"archive":"_pdb/B/modules/04158.jsonl.gz"}
//   [pdb_inline_direct] png::decoder::Decoder::new_with_limits
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/png-0.17.6/src/decoder/mod.rs
//     evidence={"module":4158,"record_offset":1268,"parent_record_offset":1032,"ancestor_record_offset":1032,"inlinee":5356,"source_line":135,"archive":"_pdb/B/modules/04158.jsonl.gz"}
//   [pdb_inline_direct] png::decoder::Decoder::set_ignore_text_chunk
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/png-0.17.6/src/decoder/mod.rs
//     evidence={"module":4158,"record_offset":1604,"parent_record_offset":1032,"ancestor_record_offset":1032,"inlinee":14746,"source_line":221,"archive":"_pdb/B/modules/04158.jsonl.gz"}
//   [pdb_inline_direct] png::decoder::Decoder::set_transformations
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/png-0.17.6/src/decoder/mod.rs
//     evidence={"module":4158,"record_offset":1804,"parent_record_offset":1032,"ancestor_record_offset":1032,"inlinee":14749,"source_line":207,"archive":"_pdb/B/modules/04158.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4158,"record_offset":1516,"parent_record_offset":1492,"ancestor_record_offset":1032,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/04158.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4158,"record_offset":1540,"parent_record_offset":1516,"ancestor_record_offset":1032,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/04158.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4158,"record_offset":1492,"parent_record_offset":1464,"ancestor_record_offset":1032,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/04158.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$2::new_uninit_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":4158,"record_offset":1376,"parent_record_offset":1348,"ancestor_record_offset":1032,"inlinee":5319,"source_line":663,"archive":"_pdb/B/modules/04158.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4158,"record_offset":1404,"parent_record_offset":1376,"ancestor_record_offset":1032,"inlinee":5320,"source_line":123,"archive":"_pdb/B/modules/04158.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4158,"record_offset":1464,"parent_record_offset":1432,"ancestor_record_offset":1032,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/04158.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4158,"record_offset":1432,"parent_record_offset":1404,"ancestor_record_offset":1032,"inlinee":5321,"source_line":145,"archive":"_pdb/B/modules/04158.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":4158,"record_offset":1708,"parent_record_offset":1660,"ancestor_record_offset":1032,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/04158.jsonl.gz"}
//   [pdb_inline_transitive] png::decoder::stream::DecodeConfig::set_ignore_text_chunk
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/png-0.17.6/src/decoder/stream.rs
//     evidence={"module":4158,"record_offset":1628,"parent_record_offset":1604,"ancestor_record_offset":1032,"inlinee":14748,"source_line":392,"archive":"_pdb/B/modules/04158.jsonl.gz"}
//   [pdb_inline_transitive] std::io::buffered::bufreader::BufReader::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\buffered\bufreader.rs
//     evidence={"module":4158,"record_offset":1316,"parent_record_offset":1268,"ancestor_record_offset":1032,"inlinee":5316,"source_line":102,"archive":"_pdb/B/modules/04158.jsonl.gz"}
//   [pdb_inline_transitive] std::io::buffered::bufreader::buffer::Buffer::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\buffered\bufreader\buffer.rs
//     evidence={"module":4158,"record_offset":1348,"parent_record_offset":1316,"ancestor_record_offset":1032,"inlinee":5317,"source_line":34,"archive":"_pdb/B/modules/04158.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: image::dynimage::decoder_to_image<alloc::boxed::Box<dyn$<image::image::ImageDecoder>,alloc::alloc::Global> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4159,"record_offset":732,"rva":38137728,"va":5406846848}
// 编译实例 RVAs: 0x245ef80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14002e900
//     owner=unresolved_namespace; count=1
//     target_aliases=130 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-37948272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406846848,"raw_hex":"e890f4bcfd","size":5,"soft_fail":false,"target_va":5368899840,"va":5406848107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31766398","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406846848,"raw_hex":"e882481bfe","size":5,"soft_fail":false,"target_va":5375081504,"va":5406847897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::image::decoder_to_vec
//     owner=package_namespace_candidate; count=10
//     packages=[{"name":"image","version":"0.24.9","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406846848,"raw_hex":"e85f0d0000","size":5,"soft_fail":false,"target_va":5406850384,"va":5406846956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5406846848,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5406846897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4159,"record_offset":1492,"parent_record_offset":732,"ancestor_record_offset":732,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/04159.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4159,"record_offset":1000,"parent_record_offset":732,"ancestor_record_offset":732,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04159.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4159,"record_offset":4840,"parent_record_offset":732,"ancestor_record_offset":732,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04159.jsonl.gz"}
//   [pdb_inline_direct] image::buffer_::ImageBuffer::from_raw
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":4159,"record_offset":1028,"parent_record_offset":732,"ancestor_record_offset":732,"inlinee":7813,"source_line":674,"archive":"_pdb/B/modules/04159.jsonl.gz"}
//   [pdb_inline_direct] image::image::impl$2::color_type
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/image.rs
//     evidence={"module":4159,"record_offset":972,"parent_record_offset":732,"ancestor_record_offset":732,"inlinee":7812,"source_line":699,"archive":"_pdb/B/modules/04159.jsonl.gz"}
//   [pdb_inline_direct] image::image::impl$2::dimensions
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/image.rs
//     evidence={"module":4159,"record_offset":944,"parent_record_offset":732,"ancestor_record_offset":732,"inlinee":7811,"source_line":696,"archive":"_pdb/B/modules/04159.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4159,"record_offset":1444,"parent_record_offset":1420,"ancestor_record_offset":732,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/04159.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4159,"record_offset":1420,"parent_record_offset":1396,"ancestor_record_offset":732,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/04159.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4159,"record_offset":1396,"parent_record_offset":1372,"ancestor_record_offset":732,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/04159.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4159,"record_offset":1220,"parent_record_offset":1196,"ancestor_record_offset":732,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/04159.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4159,"record_offset":1244,"parent_record_offset":1220,"ancestor_record_offset":732,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/04159.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4159,"record_offset":1116,"parent_record_offset":1088,"ancestor_record_offset":732,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/04159.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4159,"record_offset":1288,"parent_record_offset":1060,"ancestor_record_offset":732,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/04159.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4159,"record_offset":1348,"parent_record_offset":1028,"ancestor_record_offset":732,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04159.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::check_image_fits
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":4159,"record_offset":1060,"parent_record_offset":1028,"ancestor_record_offset":732,"inlinee":7814,"source_line":802,"archive":"_pdb/B/modules/04159.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::image_buffer_len
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":4159,"record_offset":1088,"parent_record_offset":1060,"ancestor_record_offset":732,"inlinee":7815,"source_line":807,"archive":"_pdb/B/modules/04159.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::check_image_fits::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":4159,"record_offset":1312,"parent_record_offset":1288,"ancestor_record_offset":732,"inlinee":7820,"source_line":804,"archive":"_pdb/B/modules/04159.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$0
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":4159,"record_offset":1140,"parent_record_offset":1116,"ancestor_record_offset":732,"inlinee":7817,"source_line":809,"archive":"_pdb/B/modules/04159.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$1
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":4159,"record_offset":1196,"parent_record_offset":1172,"ancestor_record_offset":732,"inlinee":7818,"source_line":810,"archive":"_pdb/B/modules/04159.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: png::decoder::ReadDecoder::decode_next<std::io::buffered::bufreader::BufReader<std::fs::File> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4610,"record_offset":13460,"rva":200688,"va":5368909808}
// 编译实例 RVAs: 0x30ff0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c3f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4567904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368909808,"raw_hex":"e860b34500","size":5,"soft_fail":false,"target_va":5373477872,"va":5368909963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] png::decoder::stream::StreamingDecoder::update
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t12096325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368909808,"raw_hex":"e84593b800","size":5,"soft_fail":false,"target_va":5381006384,"va":5368910054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4610,"record_offset":13968,"parent_record_offset":13460,"ancestor_record_offset":13460,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4610,"record_offset":14120,"parent_record_offset":13460,"ancestor_record_offset":13460,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_direct] std::io::buffered::bufreader::impl$6::consume
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\buffered\bufreader.rs
//     evidence={"module":4610,"record_offset":14008,"parent_record_offset":13460,"ancestor_record_offset":13460,"inlinee":5265,"source_line":453,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_direct] std::io::buffered::bufreader::impl$6::fill_buf
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\buffered\bufreader.rs
//     evidence={"module":4610,"record_offset":13652,"parent_record_offset":13460,"ancestor_record_offset":13460,"inlinee":5252,"source_line":449,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4610,"record_offset":14080,"parent_record_offset":14056,"ancestor_record_offset":13460,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4610,"record_offset":14056,"parent_record_offset":14032,"ancestor_record_offset":13460,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::io::borrowed_buf::BorrowedBuf::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\io\borrowed_buf.rs
//     evidence={"module":4610,"record_offset":13904,"parent_record_offset":13684,"ancestor_record_offset":13460,"inlinee":5262,"source_line":81,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::io::borrowed_buf::BorrowedBuf::set_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\io\borrowed_buf.rs
//     evidence={"module":4610,"record_offset":13848,"parent_record_offset":13684,"ancestor_record_offset":13460,"inlinee":5259,"source_line":162,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::io::borrowed_buf::impl$2::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\io\borrowed_buf.rs
//     evidence={"module":4610,"record_offset":13820,"parent_record_offset":13684,"ancestor_record_offset":13460,"inlinee":5257,"source_line":67,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4610,"record_offset":13932,"parent_record_offset":13684,"ancestor_record_offset":13460,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4610,"record_offset":13760,"parent_record_offset":13732,"ancestor_record_offset":13460,"inlinee":5263,"source_line":656,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4610,"record_offset":13784,"parent_record_offset":13760,"ancestor_record_offset":13460,"inlinee":5264,"source_line":389,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] std::io::buffered::bufreader::buffer::Buffer::buffer
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\buffered\bufreader\buffer.rs
//     evidence={"module":4610,"record_offset":13732,"parent_record_offset":13684,"ancestor_record_offset":13460,"inlinee":5255,"source_line":50,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] std::io::buffered::bufreader::buffer::Buffer::consume
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\buffered\bufreader\buffer.rs
//     evidence={"module":4610,"record_offset":14032,"parent_record_offset":14008,"ancestor_record_offset":13460,"inlinee":5266,"source_line":84,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] std::io::buffered::bufreader::buffer::Buffer::fill_buf
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\buffered\bufreader\buffer.rs
//     evidence={"module":4610,"record_offset":13684,"parent_record_offset":13652,"ancestor_record_offset":13460,"inlinee":5254,"source_line":132,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$0::read_buf
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":4610,"record_offset":13876,"parent_record_offset":13684,"ancestor_record_offset":13460,"inlinee":5261,"source_line":20,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: png::decoder::Reader::next_frame<std::io::buffered::bufreader::BufReader<std::fs::File> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4610,"record_offset":256,"rva":40469488,"va":5409178608}
// 编译实例 RVAs: 0x26983f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c3f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-35701697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409178608,"raw_hex":"e83f3cdffd","size":5,"soft_fail":false,"target_va":5373477872,"va":5409179564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34098252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409178608,"raw_hex":"e8b4b3f7fd","size":5,"soft_fail":false,"target_va":5375081504,"va":5409179751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142698b90
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1839","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409178608,"raw_hex":"e82f070000","size":5,"soft_fail":false,"target_va":5409180560,"va":5409178716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2667858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409178608,"raw_hex":"e852b52800","size":5,"soft_fail":false,"target_va":5411847168,"va":5409179305,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2361643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409178608,"raw_hex":"e82b092400","size":5,"soft_fail":false,"target_va":5411541776,"va":5409180128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t2361982","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409178608,"raw_hex":"e87e0a2400","size":5,"soft_fail":false,"target_va":5411541904,"va":5409179917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_start_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2362967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409178608,"raw_hex":"e8570e2400","size":5,"soft_fail":false,"target_va":5411543088,"va":5409180116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] png::common::ColorType::raw_row_length_from_width
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-28188582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409178608,"raw_hex":"e85ae051fe","size":5,"soft_fail":false,"target_va":5380990192,"va":5409178769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] png::decoder::Reader::next_interlaced_row_impl<std::io::buffered::bufreader::BufReader<std::fs::File> >
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409178608,"raw_hex":"e8e0100000","size":5,"soft_fail":false,"target_va":5409183296,"va":5409178971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] png::decoder::Reader::read_until_image_data<std::io::buffered::bufreader::BufReader<std::fs::File> >
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409178608,"raw_hex":"e855080000","size":5,"soft_fail":false,"target_va":5409180800,"va":5409178662,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] png::decoder::stream::StreamingDecoder::update
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-28173298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409178608,"raw_hex":"e80e1c52fe","size":5,"soft_fail":false,"target_va":5381006384,"va":5409179677,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] png::utils::expand_pass
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-28190483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409178608,"raw_hex":"e8edd851fe","size":5,"soft_fail":false,"target_va":5380988608,"va":5409179086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::panicking::unreachable_display
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panicking.rs
//     evidence={"module":4610,"record_offset":3136,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5268,"source_line":240,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4610,"record_offset":476,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4610,"record_offset":824,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4610,"record_offset":1212,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_direct] png::decoder::ReadDecoder::finished_decoding
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/png-0.17.6/src/decoder/mod.rs
//     evidence={"module":4610,"record_offset":1860,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5250,"source_line":256,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_direct] png::decoder::Reader::finished_frame
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/png-0.17.6/src/decoder/mod.rs
//     evidence={"module":4610,"record_offset":1540,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5247,"source_line":459,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_direct] png::decoder::Reader::info
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/png-0.17.6/src/decoder/mod.rs
//     evidence={"module":4610,"record_offset":912,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5229,"source_line":454,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_direct] png::decoder::Reader::next_interlaced_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/png-0.17.6/src/decoder/mod.rs
//     evidence={"module":4610,"record_offset":996,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5234,"source_line":551,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_direct] png::decoder::Reader::next_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/png-0.17.6/src/decoder/mod.rs
//     evidence={"module":4610,"record_offset":1060,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5235,"source_line":545,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_direct] png::decoder::Reader::output_buffer_size
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/png-0.17.6/src/decoder/mod.rs
//     evidence={"module":4610,"record_offset":504,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5228,"source_line":661,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_direct] png::decoder::Reader::reset_current
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/png-0.17.6/src/decoder/mod.rs
//     evidence={"module":4610,"record_offset":852,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5233,"source_line":446,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_direct] std::io::impls::impl$10::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":4610,"record_offset":1396,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5245,"source_line":430,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4610,"record_offset":2456,"parent_record_offset":2432,"ancestor_record_offset":256,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4610,"record_offset":2432,"parent_record_offset":2376,"ancestor_record_offset":256,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4610,"record_offset":2404,"parent_record_offset":2376,"ancestor_record_offset":256,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4610,"record_offset":2376,"parent_record_offset":2352,"ancestor_record_offset":256,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4610,"record_offset":2352,"parent_record_offset":2328,"ancestor_record_offset":256,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4610,"record_offset":880,"parent_record_offset":852,"ancestor_record_offset":256,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4610,"record_offset":2264,"parent_record_offset":1860,"ancestor_record_offset":256,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4610,"record_offset":1448,"parent_record_offset":1424,"ancestor_record_offset":256,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4610,"record_offset":1424,"parent_record_offset":1396,"ancestor_record_offset":256,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4610,"record_offset":1768,"parent_record_offset":1744,"ancestor_record_offset":256,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":4610,"record_offset":1504,"parent_record_offset":1480,"ancestor_record_offset":256,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::io::borrowed_buf::BorrowedBuf::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\io\borrowed_buf.rs
//     evidence={"module":4610,"record_offset":2200,"parent_record_offset":1976,"ancestor_record_offset":256,"inlinee":5262,"source_line":81,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::io::borrowed_buf::BorrowedBuf::set_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\io\borrowed_buf.rs
//     evidence={"module":4610,"record_offset":2144,"parent_record_offset":1976,"ancestor_record_offset":256,"inlinee":5259,"source_line":162,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::io::borrowed_buf::impl$2::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\io\borrowed_buf.rs
//     evidence={"module":4610,"record_offset":2116,"parent_record_offset":1976,"ancestor_record_offset":256,"inlinee":5257,"source_line":67,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4610,"record_offset":592,"parent_record_offset":568,"ancestor_record_offset":256,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::panicking::unreachable_display
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panicking.rs
//     evidence={"module":4610,"record_offset":1744,"parent_record_offset":1540,"ancestor_record_offset":256,"inlinee":5268,"source_line":240,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4610,"record_offset":2304,"parent_record_offset":1860,"ancestor_record_offset":256,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4610,"record_offset":1140,"parent_record_offset":1060,"ancestor_record_offset":256,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4610,"record_offset":2228,"parent_record_offset":1976,"ancestor_record_offset":256,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4610,"record_offset":1480,"parent_record_offset":1396,"ancestor_record_offset":256,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4610,"record_offset":2056,"parent_record_offset":2028,"ancestor_record_offset":256,"inlinee":5263,"source_line":656,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4610,"record_offset":1348,"parent_record_offset":1324,"ancestor_record_offset":256,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_mut_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4610,"record_offset":1324,"parent_record_offset":1300,"ancestor_record_offset":256,"inlinee":5242,"source_line":112,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4610,"record_offset":2080,"parent_record_offset":2056,"ancestor_record_offset":256,"inlinee":5264,"source_line":389,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4610,"record_offset":1300,"parent_record_offset":1276,"ancestor_record_offset":256,"inlinee":5241,"source_line":413,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4610,"record_offset":1276,"parent_record_offset":1240,"ancestor_record_offset":256,"inlinee":5240,"source_line":553,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4610,"record_offset":1240,"parent_record_offset":1212,"ancestor_record_offset":256,"inlinee":5239,"source_line":568,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] png::common::Info::animation_control
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/png-0.17.6/src/common.rs
//     evidence={"module":4610,"record_offset":1688,"parent_record_offset":1540,"ancestor_record_offset":256,"inlinee":5249,"source_line":570,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] png::decoder::ReadDecoder::info
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/png-0.17.6/src/decoder/mod.rs
//     evidence={"module":4610,"record_offset":568,"parent_record_offset":536,"ancestor_record_offset":256,"inlinee":5231,"source_line":282,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] png::decoder::Reader::info
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/png-0.17.6/src/decoder/mod.rs
//     evidence={"module":4610,"record_offset":536,"parent_record_offset":504,"ancestor_record_offset":256,"inlinee":5229,"source_line":454,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] png::decoder::Reader::next_interlaced_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/png-0.17.6/src/decoder/mod.rs
//     evidence={"module":4610,"record_offset":1096,"parent_record_offset":1060,"ancestor_record_offset":256,"inlinee":5234,"source_line":551,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] png::decoder::Reader::output_line_size
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/png-0.17.6/src/decoder/mod.rs
//     evidence={"module":4610,"record_offset":628,"parent_record_offset":504,"ancestor_record_offset":256,"inlinee":5232,"source_line":690,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] std::io::buffered::bufreader::buffer::Buffer::buffer
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\buffered\bufreader\buffer.rs
//     evidence={"module":4610,"record_offset":2028,"parent_record_offset":1976,"ancestor_record_offset":256,"inlinee":5255,"source_line":50,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] std::io::buffered::bufreader::buffer::Buffer::consume
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\buffered\bufreader\buffer.rs
//     evidence={"module":4610,"record_offset":2528,"parent_record_offset":2504,"ancestor_record_offset":256,"inlinee":5266,"source_line":84,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] std::io::buffered::bufreader::buffer::Buffer::fill_buf
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\buffered\bufreader\buffer.rs
//     evidence={"module":4610,"record_offset":1976,"parent_record_offset":1940,"ancestor_record_offset":256,"inlinee":5254,"source_line":132,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] std::io::buffered::bufreader::impl$6::consume
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\buffered\bufreader.rs
//     evidence={"module":4610,"record_offset":2504,"parent_record_offset":1860,"ancestor_record_offset":256,"inlinee":5265,"source_line":453,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] std::io::buffered::bufreader::impl$6::fill_buf
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\buffered\bufreader.rs
//     evidence={"module":4610,"record_offset":1940,"parent_record_offset":1860,"ancestor_record_offset":256,"inlinee":5252,"source_line":449,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$0::read_buf
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":4610,"record_offset":2172,"parent_record_offset":1976,"ancestor_record_offset":256,"inlinee":5261,"source_line":20,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: png::decoder::Reader::next_interlaced_row_impl<std::io::buffered::bufreader::BufReader<std::fs::File> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4610,"record_offset":9364,"rva":40474176,"va":5409183296}
// 编译实例 RVAs: 0x2699640
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34102121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409183296,"raw_hex":"e897a4f7fd","size":5,"soft_fail":false,"target_va":5375081504,"va":5409183620,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2663480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409183296,"raw_hex":"e838a42800","size":5,"soft_fail":false,"target_va":5411847168,"va":5409183683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2357267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409183296,"raw_hex":"e813f82300","size":5,"soft_fail":false,"target_va":5411541776,"va":5409184504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2357805","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409183296,"raw_hex":"e82dfa2300","size":5,"soft_fail":false,"target_va":5411542296,"va":5409184486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2358634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409183296,"raw_hex":"e86afd2300","size":5,"soft_fail":false,"target_va":5411543104,"va":5409184465,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] png::decoder::Reader::next_raw_interlaced_row<std::io::buffered::bufreader::BufReader<std::fs::File> >
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-1255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409183296,"raw_hex":"e819fbffff","size":5,"soft_fail":false,"target_va":5409182176,"va":5409183426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] png::decoder::expand_gray_u8
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-28183860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409183296,"raw_hex":"e8ccf251fe","size":5,"soft_fail":false,"target_va":5381000560,"va":5409184415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] png::decoder::expand_paletted
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-28186258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409183296,"raw_hex":"e86ee951fe","size":5,"soft_fail":false,"target_va":5380998016,"va":5409184269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] png::utils::expand_trns_line
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-28197938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409183296,"raw_hex":"e8cebb51fe","size":5,"soft_fail":false,"target_va":5380986448,"va":5409184381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] png::utils::expand_trns_line16
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-28197358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409183296,"raw_hex":"e812be51fe","size":5,"soft_fail":false,"target_va":5380987088,"va":5409184441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-34102052","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5409183296,"raw_hex":"e9dca4f7fd","size":5,"soft_fail":false,"target_va":5375081504,"va":5409183551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] png::decoder::Reader::next_raw_interlaced_row<std::io::buffered::bufreader::BufReader<std::fs::File> >
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tjmp\t-1410","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5409183296,"raw_hex":"e97efaffff","size":5,"soft_fail":false,"target_va":5409182176,"va":5409183581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$14::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4610,"record_offset":10740,"parent_record_offset":9364,"ancestor_record_offset":9364,"inlinee":5305,"source_line":3369,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":4610,"record_offset":10872,"parent_record_offset":9364,"ancestor_record_offset":9364,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_direct] core::mem::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":4610,"record_offset":9624,"parent_record_offset":9364,"ancestor_record_offset":9364,"inlinee":5313,"source_line":793,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4610,"record_offset":10696,"parent_record_offset":9364,"ancestor_record_offset":9364,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4610,"record_offset":9800,"parent_record_offset":9364,"ancestor_record_offset":9364,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4610,"record_offset":9740,"parent_record_offset":9364,"ancestor_record_offset":9364,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4610,"record_offset":9772,"parent_record_offset":9364,"ancestor_record_offset":9364,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_direct] png::common::impl$88::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":4610,"record_offset":9592,"parent_record_offset":9364,"ancestor_record_offset":9364,"inlinee":5270,"source_line":363,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_direct] png::decoder::Reader::info
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/png-0.17.6/src/decoder/mod.rs
//     evidence={"module":4610,"record_offset":10424,"parent_record_offset":9364,"ancestor_record_offset":9364,"inlinee":5229,"source_line":454,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_direct] png::decoder::Reader::line_size
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/png-0.17.6/src/decoder/mod.rs
//     evidence={"module":4610,"record_offset":10524,"parent_record_offset":9364,"ancestor_record_offset":9364,"inlinee":5283,"source_line":696,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_direct] std::io::impls::impl$10::write_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":4610,"record_offset":10224,"parent_record_offset":9364,"ancestor_record_offset":9364,"inlinee":5314,"source_line":457,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4610,"record_offset":9952,"parent_record_offset":9928,"ancestor_record_offset":9364,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4610,"record_offset":9928,"parent_record_offset":9872,"ancestor_record_offset":9364,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4610,"record_offset":9900,"parent_record_offset":9872,"ancestor_record_offset":9364,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4610,"record_offset":9872,"parent_record_offset":9848,"ancestor_record_offset":9364,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4610,"record_offset":9848,"parent_record_offset":9824,"ancestor_record_offset":9364,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4610,"record_offset":10300,"parent_record_offset":10276,"ancestor_record_offset":9364,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4610,"record_offset":10932,"parent_record_offset":10900,"ancestor_record_offset":9364,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4610,"record_offset":10276,"parent_record_offset":10248,"ancestor_record_offset":9364,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":4610,"record_offset":10356,"parent_record_offset":10332,"ancestor_record_offset":9364,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":4610,"record_offset":10964,"parent_record_offset":10900,"ancestor_record_offset":9364,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":4610,"record_offset":10900,"parent_record_offset":10872,"ancestor_record_offset":9364,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":4610,"record_offset":9648,"parent_record_offset":9624,"ancestor_record_offset":9364,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4610,"record_offset":10988,"parent_record_offset":10964,"ancestor_record_offset":9364,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4610,"record_offset":10488,"parent_record_offset":10452,"ancestor_record_offset":9364,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4610,"record_offset":9824,"parent_record_offset":9800,"ancestor_record_offset":9364,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4610,"record_offset":9676,"parent_record_offset":9648,"ancestor_record_offset":9364,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4610,"record_offset":9704,"parent_record_offset":9648,"ancestor_record_offset":9364,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4610,"record_offset":10332,"parent_record_offset":10248,"ancestor_record_offset":9364,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4610,"record_offset":10768,"parent_record_offset":10740,"ancestor_record_offset":9364,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4610,"record_offset":10824,"parent_record_offset":10796,"ancestor_record_offset":9364,"inlinee":5308,"source_line":444,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4610,"record_offset":10796,"parent_record_offset":10768,"ancestor_record_offset":9364,"inlinee":5312,"source_line":526,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] png::common::ColorType::checked_raw_row_length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/png-0.17.6/src/common.rs
//     evidence={"module":4610,"record_offset":10620,"parent_record_offset":10524,"ancestor_record_offset":9364,"inlinee":5277,"source_line":51,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] png::common::ColorType::samples_u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/png-0.17.6/src/common.rs
//     evidence={"module":4610,"record_offset":10652,"parent_record_offset":10620,"ancestor_record_offset":9364,"inlinee":5278,"source_line":29,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] png::common::impl$31::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/png-0.17.6/src/common.rs
//     evidence={"module":4610,"record_offset":10592,"parent_record_offset":10524,"ancestor_record_offset":9364,"inlinee":5285,"source_line":84,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] png::decoder::ReadDecoder::info
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/png-0.17.6/src/decoder/mod.rs
//     evidence={"module":4610,"record_offset":10452,"parent_record_offset":10424,"ancestor_record_offset":9364,"inlinee":5231,"source_line":282,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$10::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":4610,"record_offset":10248,"parent_record_offset":10224,"ancestor_record_offset":9364,"inlinee":5245,"source_line":430,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: png::decoder::Reader::read_until_image_data<std::io::buffered::bufreader::BufReader<std::fs::File> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4610,"record_offset":3852,"rva":40471680,"va":5409180800}
// 编译实例 RVAs: 0x2698c80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140030ff0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-40271366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409180800,"raw_hex":"e8fa8199fd","size":5,"soft_fail":false,"target_va":5368909808,"va":5409181169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34099907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409180800,"raw_hex":"e83dadf7fd","size":5,"soft_fail":false,"target_va":5375081504,"va":5409181406,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420db610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6019827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409180800,"raw_hex":"e80d25a4ff","size":5,"soft_fail":false,"target_va":5403162128,"va":5409181950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142698b90
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-334","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409180800,"raw_hex":"e8b2feffff","size":5,"soft_fail":false,"target_va":5409180560,"va":5409180889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::spec_from_elem::impl$3::from_elem<alloc::alloc::Global>
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409180800,"raw_hex":"e84d0e0000","size":5,"soft_fail":false,"target_va":5409185632,"va":5409181966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t2359717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409180800,"raw_hex":"e8a5012400","size":5,"soft_fail":false,"target_va":5411541776,"va":5409182054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] png::common::ColorType::raw_row_length_from_width
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-28190857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409180800,"raw_hex":"e877d751fe","size":5,"soft_fail":false,"target_va":5380990192,"va":5409181044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] png::common::Info::bpp_in_prediction
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-28191288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409180800,"raw_hex":"e8c8d551fe","size":5,"soft_fail":false,"target_va":5380990304,"va":5409181587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] png::common::Info::checked_raw_row_length
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-28190387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409180800,"raw_hex":"e84dd951fe","size":5,"soft_fail":false,"target_va":5380990480,"va":5409180862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] png::decoder::Reader::output_info<std::io::buffered::bufreader::BufReader<std::fs::File> >
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-1571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409180800,"raw_hex":"e8ddf9ffff","size":5,"soft_fail":false,"target_va":5409180464,"va":5409182030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] png::decoder::SubframeInfo::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-28183657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409180800,"raw_hex":"e897f351fe","size":5,"soft_fail":false,"target_va":5380997616,"va":5409181268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4610,"record_offset":4696,"parent_record_offset":3852,"ancestor_record_offset":3852,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4610,"record_offset":5884,"parent_record_offset":3852,"ancestor_record_offset":3852,"inlinee":4759,"source_line":3186,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4610,"record_offset":4844,"parent_record_offset":3852,"ancestor_record_offset":3852,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4610,"record_offset":4416,"parent_record_offset":3852,"ancestor_record_offset":3852,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_direct] png::decoder::ReadDecoder::info
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/png-0.17.6/src/decoder/mod.rs
//     evidence={"module":4610,"record_offset":4444,"parent_record_offset":3852,"ancestor_record_offset":3852,"inlinee":5231,"source_line":282,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_direct] png::decoder::Reader::allocate_out_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/png-0.17.6/src/decoder/mod.rs
//     evidence={"module":4610,"record_offset":5548,"parent_record_offset":3852,"ancestor_record_offset":3852,"inlinee":5282,"source_line":723,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_direct] png::decoder::Reader::info
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/png-0.17.6/src/decoder/mod.rs
//     evidence={"module":4610,"record_offset":4744,"parent_record_offset":3852,"ancestor_record_offset":3852,"inlinee":5229,"source_line":454,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_direct] png::decoder::Reader::output_info
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/png-0.17.6/src/decoder/mod.rs
//     evidence={"module":4610,"record_offset":4600,"parent_record_offset":3852,"ancestor_record_offset":3852,"inlinee":5281,"source_line":431,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_direct] png::decoder::Reader::validate_buffer_sizes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/png-0.17.6/src/decoder/mod.rs
//     evidence={"module":4610,"record_offset":4072,"parent_record_offset":3852,"ancestor_record_offset":3852,"inlinee":5273,"source_line":667,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_direct] png::decoder::impl$28::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/png-0.17.6/src/decoder/mod.rs
//     evidence={"module":4610,"record_offset":4568,"parent_record_offset":3852,"ancestor_record_offset":3852,"inlinee":5280,"source_line":334,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4610,"record_offset":4996,"parent_record_offset":4972,"ancestor_record_offset":3852,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4610,"record_offset":4972,"parent_record_offset":4916,"ancestor_record_offset":3852,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4610,"record_offset":4944,"parent_record_offset":4916,"ancestor_record_offset":3852,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4610,"record_offset":4916,"parent_record_offset":4892,"ancestor_record_offset":3852,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4610,"record_offset":4892,"parent_record_offset":4868,"ancestor_record_offset":3852,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4610,"record_offset":4352,"parent_record_offset":4224,"ancestor_record_offset":3852,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4610,"record_offset":4376,"parent_record_offset":4352,"ancestor_record_offset":3852,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4610,"record_offset":4156,"parent_record_offset":4132,"ancestor_record_offset":3852,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4610,"record_offset":4188,"parent_record_offset":4100,"ancestor_record_offset":3852,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4610,"record_offset":4868,"parent_record_offset":4844,"ancestor_record_offset":3852,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] png::common::ColorType::checked_raw_row_length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/png-0.17.6/src/common.rs
//     evidence={"module":4610,"record_offset":4284,"parent_record_offset":4224,"ancestor_record_offset":3852,"inlinee":5277,"source_line":51,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] png::common::ColorType::samples_u8
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/png-0.17.6/src/common.rs
//     evidence={"module":4610,"record_offset":4312,"parent_record_offset":4284,"ancestor_record_offset":3852,"inlinee":5278,"source_line":29,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] png::common::Info::size
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/png-0.17.6/src/common.rs
//     evidence={"module":4610,"record_offset":4256,"parent_record_offset":4224,"ancestor_record_offset":3852,"inlinee":5275,"source_line":560,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] png::common::impl$31::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/png-0.17.6/src/common.rs
//     evidence={"module":4610,"record_offset":5776,"parent_record_offset":5580,"ancestor_record_offset":3852,"inlinee":5285,"source_line":84,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] png::decoder::ReadDecoder::info
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/png-0.17.6/src/decoder/mod.rs
//     evidence={"module":4610,"record_offset":4132,"parent_record_offset":4100,"ancestor_record_offset":3852,"inlinee":5231,"source_line":282,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] png::decoder::Reader::checked_output_buffer_size
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/png-0.17.6/src/decoder/mod.rs
//     evidence={"module":4610,"record_offset":4224,"parent_record_offset":4072,"ancestor_record_offset":3852,"inlinee":5274,"source_line":681,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] png::decoder::Reader::info
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/png-0.17.6/src/decoder/mod.rs
//     evidence={"module":4610,"record_offset":4100,"parent_record_offset":4072,"ancestor_record_offset":3852,"inlinee":5229,"source_line":454,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] png::decoder::Reader::line_size
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/png-0.17.6/src/decoder/mod.rs
//     evidence={"module":4610,"record_offset":5580,"parent_record_offset":5548,"ancestor_record_offset":3852,"inlinee":5283,"source_line":696,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   [pdb_inline_transitive] png::decoder::Reader::output_line_size
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"png","version":"0.17.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/png-0.17.6/src/decoder/mod.rs
//     evidence={"module":4610,"record_offset":4632,"parent_record_offset":4600,"ancestor_record_offset":3852,"inlinee":5232,"source_line":690,"archive":"_pdb/B/modules/04610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::Deserializer::handle_struct_after_name<serde::de::ignored_any::IgnoredAny>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4139,"record_offset":1120,"rva":38032960,"va":5406742080}
// 编译实例 RVAs: 0x2445640
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] ron::de::impl$6::next_key_seed<core::marker::PhantomData<serde::de::ignored_any::IgnoredAny> >
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t3437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406742080,"raw_hex":"e86d0d0000","size":5,"soft_fail":false,"target_va":5406745632,"va":5406742190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::impl$6::next_value_seed<core::marker::PhantomData<serde::de::ignored_any::IgnoredAny> >
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t3847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406742080,"raw_hex":"e8070f0000","size":5,"soft_fail":false,"target_va":5406746112,"va":5406742260,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::struct_error_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25657000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406742080,"raw_hex":"e8588178fe","size":5,"soft_fail":false,"target_va":5381085680,"va":5406742675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_char
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25695420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406742080,"raw_hex":"e844eb77fe","size":5,"soft_fail":false,"target_va":5381046704,"va":5406742119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_ws
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25671278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406742080,"raw_hex":"e8924978fe","size":5,"soft_fail":false,"target_va":5381071168,"va":5406742441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":1312,"parent_record_offset":1120,"ancestor_record_offset":1120,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":1516,"parent_record_offset":1120,"ancestor_record_offset":1120,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":1580,"parent_record_offset":1120,"ancestor_record_offset":1120,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":1544,"parent_record_offset":1120,"ancestor_record_offset":1120,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":1636,"parent_record_offset":1120,"ancestor_record_offset":1120,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] ron::de::CommaSeparated::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":1344,"parent_record_offset":1120,"ancestor_record_offset":1120,"inlinee":13279,"source_line":791,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] serde::de::ignored_any::impl$0::visit_map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":4139,"record_offset":1372,"parent_record_offset":1120,"ancestor_record_offset":1120,"inlinee":13280,"source_line":203,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$1::handle_struct_after_name::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":1604,"parent_record_offset":1580,"ancestor_record_offset":1120,"inlinee":13282,"source_line":240,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_entry
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":1400,"parent_record_offset":1372,"ancestor_record_offset":1120,"inlinee":6627,"source_line":1876,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_entry_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":1428,"parent_record_offset":1400,"ancestor_record_offset":1120,"inlinee":6628,"source_line":1822,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::impl$2::deserialize_any<serde::de::ignored_any::IgnoredAny>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4139,"record_offset":11836,"rva":38039920,"va":5406749040}
// 编译实例 RVAs: 0x2447170
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142447170
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406749040,"raw_hex":"e817feffff","size":5,"soft_fail":false,"target_va":5406749040,"va":5406749524,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<ron::error::Error> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-7358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406749040,"raw_hex":"e842e3ffff","size":5,"soft_fail":false,"target_va":5406742800,"va":5406750153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::Deserializer::handle_any_struct<serde::de::ignored_any::IgnoredAny>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-8331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406749040,"raw_hex":"e875dfffff","size":5,"soft_fail":false,"target_va":5406741568,"va":5406749894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::Deserializer::handle_struct_after_name<serde::de::ignored_any::IgnoredAny>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-7108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406749040,"raw_hex":"e83ce4ffff","size":5,"soft_fail":false,"target_va":5406742080,"va":5406749183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::impl$2::deserialize_byte_buf<serde::de::ignored_any::IgnoredAny>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t35472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406749040,"raw_hex":"e8908a0000","size":5,"soft_fail":false,"target_va":5406785952,"va":5406750475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::impl$2::deserialize_char<serde::de::ignored_any::IgnoredAny>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-34321584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406749040,"raw_hex":"e8504bf4fd","size":5,"soft_fail":false,"target_va":5372428800,"va":5406750379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::impl$2::deserialize_map<serde::de::ignored_any::IgnoredAny>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t6153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406749040,"raw_hex":"e809180000","size":5,"soft_fail":false,"target_va":5406756592,"va":5406750434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::impl$2::deserialize_seq<serde::de::ignored_any::IgnoredAny>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t8320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406749040,"raw_hex":"e880200000","size":5,"soft_fail":false,"target_va":5406758720,"va":5406750395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::impl$2::deserialize_str<serde::de::ignored_any::IgnoredAny>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-34322224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406749040,"raw_hex":"e8d048f4fd","size":5,"soft_fail":false,"target_va":5372428144,"va":5406750363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::impl$2::deserialize_tuple<serde::de::ignored_any::IgnoredAny>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t13163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406749040,"raw_hex":"e86b330000","size":5,"soft_fail":false,"target_va":5406762416,"va":5406749248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::any_number
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25692193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406749040,"raw_hex":"e8dff777fe","size":5,"soft_fail":false,"target_va":5381058032,"va":5406750220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::check_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25702968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406749040,"raw_hex":"e8c8cd77fe","size":5,"soft_fail":false,"target_va":5381046112,"va":5406749075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::check_ident
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25685140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406749040,"raw_hex":"e86c1378fe","size":5,"soft_fail":false,"target_va":5381064208,"va":5406749343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::check_struct_type
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25684557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406749040,"raw_hex":"e8b31578fe","size":5,"soft_fail":false,"target_va":5381064592,"va":5406749144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::comma
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25685570","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406749040,"raw_hex":"e8be1178fe","size":5,"soft_fail":false,"target_va":5381064016,"va":5406749581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_char
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25702743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406749040,"raw_hex":"e8a9ce77fe","size":5,"soft_fail":false,"target_va":5381046704,"va":5406749442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_ident
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25683556","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406749040,"raw_hex":"e89c1978fe","size":5,"soft_fail":false,"target_va":5381065728,"va":5406749279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_str
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25702535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406749040,"raw_hex":"e879cf77fe","size":5,"soft_fail":false,"target_va":5381046576,"va":5406749106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::identifier
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25680161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406749040,"raw_hex":"e8df2678fe","size":5,"soft_fail":false,"target_va":5381069696,"va":5406749852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::peek_char_or_eof
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25704203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406749040,"raw_hex":"e8f5c877fe","size":5,"soft_fail":false,"target_va":5381045968,"va":5406750166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_ws
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25678255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406749040,"raw_hex":"e8512e78fe","size":5,"soft_fail":false,"target_va":5381071168,"va":5406749418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::src
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25703956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406749040,"raw_hex":"e8ecc977fe","size":5,"soft_fail":false,"target_va":5381046496,"va":5406750447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":12836,"parent_record_offset":11836,"ancestor_record_offset":11836,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":12044,"parent_record_offset":11836,"ancestor_record_offset":11836,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":12076,"parent_record_offset":11836,"ancestor_record_offset":11836,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::starts_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4139,"record_offset":13056,"parent_record_offset":11836,"ancestor_record_offset":11836,"inlinee":8789,"source_line":1366,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] ron::de::impl$2::deserialize_option
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":12104,"parent_record_offset":11836,"ancestor_record_offset":11836,"inlinee":13364,"source_line":484,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] ron::de::impl$2::deserialize_string
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":13028,"parent_record_offset":11836,"ancestor_record_offset":11836,"inlinee":5162,"source_line":455,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] ron::de::impl$2::deserialize_unit
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":12012,"parent_record_offset":11836,"ancestor_record_offset":11836,"inlinee":13363,"source_line":520,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4139,"record_offset":13128,"parent_record_offset":13104,"ancestor_record_offset":11836,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":12416,"parent_record_offset":12104,"ancestor_record_offset":11836,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":12568,"parent_record_offset":12104,"ancestor_record_offset":11836,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4139,"record_offset":12868,"parent_record_offset":12836,"ancestor_record_offset":11836,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":12264,"parent_record_offset":12104,"ancestor_record_offset":11836,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":12776,"parent_record_offset":12104,"ancestor_record_offset":11836,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4139,"record_offset":13152,"parent_record_offset":13128,"ancestor_record_offset":11836,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4139,"record_offset":13176,"parent_record_offset":13152,"ancestor_record_offset":11836,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4139,"record_offset":13104,"parent_record_offset":13080,"ancestor_record_offset":11836,"inlinee":5361,"source_line":2581,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":4139,"record_offset":13080,"parent_record_offset":13056,"ancestor_record_offset":11836,"inlinee":8748,"source_line":981,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_ignored_any
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":12496,"parent_record_offset":12472,"ancestor_record_offset":11836,"inlinee":6889,"source_line":758,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::extensions::Extensions::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.9.0/src/lib.rs
//     evidence={"module":4139,"record_offset":12320,"parent_record_offset":12104,"ancestor_record_offset":11836,"inlinee":12958,"source_line":703,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::extensions::_::InternalBitFlags::bits
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.9.0/src/lib.rs
//     evidence={"module":4139,"record_offset":12376,"parent_record_offset":12348,"ancestor_record_offset":11836,"inlinee":13353,"source_line":641,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::extensions::_::InternalBitFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.9.0/src/lib.rs
//     evidence={"module":4139,"record_offset":12348,"parent_record_offset":12320,"ancestor_record_offset":11836,"inlinee":12959,"source_line":703,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$0::visit_some
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":4139,"record_offset":12448,"parent_record_offset":12104,"ancestor_record_offset":11836,"inlinee":13365,"source_line":171,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":4139,"record_offset":12472,"parent_record_offset":12448,"ancestor_record_offset":11836,"inlinee":6518,"source_line":232,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::impl$2::deserialize_byte_buf<serde::de::ignored_any::IgnoredAny>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4139,"record_offset":48444,"rva":38076832,"va":5406785952}
// 编译实例 RVAs: 0x24501a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31704985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406785952,"raw_hex":"e867381cfe","size":5,"soft_fail":false,"target_va":5375081504,"va":5406786484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14268dab0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2349193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406785952,"raw_hex":"e889d82300","size":5,"soft_fail":false,"target_va":5409135280,"va":5406786082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::byte_string
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25713593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406785952,"raw_hex":"e847a477fe","size":5,"soft_fail":false,"target_va":5381072640,"va":5406786228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::check_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25739894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406785952,"raw_hex":"e88a3d77fe","size":5,"soft_fail":false,"target_va":5381046112,"va":5406786001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_char
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25739327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406785952,"raw_hex":"e8c13f77fe","size":5,"soft_fail":false,"target_va":5381046704,"va":5406786026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_ws
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25714989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406785952,"raw_hex":"e8d39e77fe","size":5,"soft_fail":false,"target_va":5381071168,"va":5406786152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-31704863","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5406785952,"raw_hex":"e9e1381cfe","size":5,"soft_fail":false,"target_va":5375081504,"va":5406786362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":49736,"parent_record_offset":48444,"ancestor_record_offset":48444,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":49828,"parent_record_offset":48444,"ancestor_record_offset":48444,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] serde::de::Visitor::visit_borrowed_bytes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":49772,"parent_record_offset":48444,"ancestor_record_offset":48444,"inlinee":11492,"source_line":1563,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] serde::de::Visitor::visit_byte_buf
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":49476,"parent_record_offset":48444,"ancestor_record_offset":48444,"inlinee":11474,"source_line":1588,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4139,"record_offset":48620,"parent_record_offset":48444,"ancestor_record_offset":48444,"inlinee":6469,"source_line":1150,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4139,"record_offset":49020,"parent_record_offset":48996,"ancestor_record_offset":48444,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4139,"record_offset":48996,"parent_record_offset":48940,"ancestor_record_offset":48444,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4139,"record_offset":48968,"parent_record_offset":48940,"ancestor_record_offset":48444,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4139,"record_offset":48940,"parent_record_offset":48916,"ancestor_record_offset":48444,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4139,"record_offset":48916,"parent_record_offset":48892,"ancestor_record_offset":48444,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":48728,"parent_record_offset":48652,"ancestor_record_offset":48444,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":48804,"parent_record_offset":48652,"ancestor_record_offset":48444,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4139,"record_offset":48868,"parent_record_offset":48652,"ancestor_record_offset":48444,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":48832,"parent_record_offset":48652,"ancestor_record_offset":48444,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::CommaSeparated::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":48760,"parent_record_offset":48652,"ancestor_record_offset":48444,"inlinee":13279,"source_line":791,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_seq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":48652,"parent_record_offset":48620,"ancestor_record_offset":48444,"inlinee":13417,"source_line":595,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$0::visit_bytes
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":4139,"record_offset":49504,"parent_record_offset":49476,"ancestor_record_offset":48444,"inlinee":12487,"source_line":214,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::impl$2::deserialize_option<serde::de::impls::OptionVisitor<array$<u32,2> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4139,"record_offset":30472,"rva":38053984,"va":5406763104}
// 编译实例 RVAs: 0x244a860
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14244a130
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406763104,"raw_hex":"e8fef7ffff","size":5,"soft_fail":false,"target_va":5406761264,"va":5406763309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::comma
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25699360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406763104,"raw_hex":"e8e0db77fe","size":5,"soft_fail":false,"target_va":5381064016,"va":5406763371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_char
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25716528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406763104,"raw_hex":"e8d09877fe","size":5,"soft_fail":false,"target_va":5381046704,"va":5406763227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_ident
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25697419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406763104,"raw_hex":"e875e377fe","size":5,"soft_fail":false,"target_va":5381065728,"va":5406763142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_ws
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25692040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406763104,"raw_hex":"e878f877fe","size":5,"soft_fail":false,"target_va":5381071168,"va":5406763203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":30848,"parent_record_offset":30472,"ancestor_record_offset":30472,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":30976,"parent_record_offset":30472,"ancestor_record_offset":30472,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":30696,"parent_record_offset":30472,"ancestor_record_offset":30472,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":31224,"parent_record_offset":30472,"ancestor_record_offset":30472,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] ron::extensions::Extensions::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.9.0/src/lib.rs
//     evidence={"module":4139,"record_offset":30752,"parent_record_offset":30472,"ancestor_record_offset":30472,"inlinee":12958,"source_line":703,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impls::impl$15::visit_none
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4139,"record_offset":30668,"parent_record_offset":30472,"ancestor_record_offset":30472,"inlinee":9113,"source_line":904,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impls::impl$15::visit_some
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4139,"record_offset":30880,"parent_record_offset":30472,"ancestor_record_offset":30472,"inlinee":11509,"source_line":912,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":30936,"parent_record_offset":30880,"ancestor_record_offset":30472,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::extensions::_::InternalBitFlags::bits
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.9.0/src/lib.rs
//     evidence={"module":4139,"record_offset":30808,"parent_record_offset":30780,"ancestor_record_offset":30472,"inlinee":13353,"source_line":641,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::extensions::_::InternalBitFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.9.0/src/lib.rs
//     evidence={"module":4139,"record_offset":30780,"parent_record_offset":30752,"ancestor_record_offset":30472,"inlinee":12959,"source_line":703,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$96::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4139,"record_offset":30908,"parent_record_offset":30880,"ancestor_record_offset":30472,"inlinee":13361,"source_line":1338,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::impl$2::deserialize_seq<serde::de::ignored_any::IgnoredAny>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4139,"record_offset":23140,"rva":38049600,"va":5406758720}
// 编译实例 RVAs: 0x2449740
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142447170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406758720,"raw_hex":"e871d9ffff","size":5,"soft_fail":false,"target_va":5406749040,"va":5406758906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::CommaSeparated::has_element
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25673303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406758720,"raw_hex":"e8a94178fe","size":5,"soft_fail":false,"target_va":5381085520,"va":5406758818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::comma
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25694945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406758720,"raw_hex":"e81fed77fe","size":5,"soft_fail":false,"target_va":5381064016,"va":5406758956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_char
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25712059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406758720,"raw_hex":"e845aa77fe","size":5,"soft_fail":false,"target_va":5381046704,"va":5406758758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_ws
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25688037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406758720,"raw_hex":"e81b0878fe","size":5,"soft_fail":false,"target_va":5381071168,"va":5406759200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":23312,"parent_record_offset":23140,"ancestor_record_offset":23140,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":23844,"parent_record_offset":23140,"ancestor_record_offset":23140,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":23872,"parent_record_offset":23140,"ancestor_record_offset":23140,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":23816,"parent_record_offset":23140,"ancestor_record_offset":23140,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] ron::de::CommaSeparated::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":23344,"parent_record_offset":23140,"ancestor_record_offset":23140,"inlinee":13279,"source_line":791,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] serde::de::ignored_any::impl$0::visit_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":4139,"record_offset":23372,"parent_record_offset":23140,"ancestor_record_offset":23140,"inlinee":13378,"source_line":192,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":23560,"parent_record_offset":23452,"ancestor_record_offset":23140,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":23720,"parent_record_offset":23452,"ancestor_record_offset":23140,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":23528,"parent_record_offset":23452,"ancestor_record_offset":23140,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_ignored_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":23648,"parent_record_offset":23624,"ancestor_record_offset":23140,"inlinee":6889,"source_line":758,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$5::next_element_seed
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":23452,"parent_record_offset":23420,"ancestor_record_offset":23140,"inlinee":13377,"source_line":819,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":23420,"parent_record_offset":23372,"ancestor_record_offset":23140,"inlinee":6504,"source_line":1724,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":4139,"record_offset":23624,"parent_record_offset":23600,"ancestor_record_offset":23140,"inlinee":6518,"source_line":232,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":23600,"parent_record_offset":23452,"ancestor_record_offset":23140,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::impl$2::deserialize_struct<asset::texture::desc::_::impl$0::deserialize::__Visitor>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4139,"record_offset":44580,"rva":38072736,"va":5406781856}
// 编译实例 RVAs: 0x244f1a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1424499e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406781856,"raw_hex":"e886a1ffff","size":5,"soft_fail":false,"target_va":5406759392,"va":5406783573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142449cb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406781856,"raw_hex":"e864a3ffff","size":5,"soft_fail":false,"target_va":5406760112,"va":5406783815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142449ef0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406781856,"raw_hex":"e8fca4ffff","size":5,"soft_fail":false,"target_va":5406760688,"va":5406783983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14244a860
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406781856,"raw_hex":"e820b1ffff","size":5,"soft_fail":false,"target_va":5406763104,"va":5406783291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14245c070
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t52539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406781856,"raw_hex":"e83bcd0000","size":5,"soft_fail":false,"target_va":5406834800,"va":5406782256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::CommaSeparated::has_element
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25696577","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406781856,"raw_hex":"e8bfe677fe","size":5,"soft_fail":false,"target_va":5381085520,"va":5406782092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::impl$6::next_value_seed<core::marker::PhantomData<serde::de::ignored_any::IgnoredAny> >
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-37304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406781856,"raw_hex":"e8486effff","size":5,"soft_fail":false,"target_va":5406746112,"va":5406783411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::struct_error_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25698974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406781856,"raw_hex":"e862dd77fe","size":5,"soft_fail":false,"target_va":5381085680,"va":5406784649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::comma
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25719359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406781856,"raw_hex":"e8c18d77fe","size":5,"soft_fail":false,"target_va":5381064016,"va":5406783370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_char
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25735306","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406781856,"raw_hex":"e8764f77fe","size":5,"soft_fail":false,"target_va":5381046704,"va":5406782005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_struct_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25716075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406781856,"raw_hex":"e8959a77fe","size":5,"soft_fail":false,"target_va":5381065872,"va":5406781942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::identifier
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25713131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406781856,"raw_hex":"e815a677fe","size":5,"soft_fail":false,"target_va":5381069696,"va":5406782822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_ws
//     owner=package_namespace_candidate; count=10
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25710809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406781856,"raw_hex":"e827af77fe","size":5,"soft_fail":false,"target_va":5381071168,"va":5406781972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":44764,"parent_record_offset":44580,"ancestor_record_offset":44580,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":48356,"parent_record_offset":44580,"ancestor_record_offset":44580,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] ron::de::Deserializer::handle_struct_after_name
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":44828,"parent_record_offset":44580,"ancestor_record_offset":44580,"inlinee":13303,"source_line":225,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":4139,"record_offset":47228,"parent_record_offset":47204,"ancestor_record_offset":44580,"inlinee":108179,"source_line":4,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":4139,"record_offset":45956,"parent_record_offset":45920,"ancestor_record_offset":44580,"inlinee":108176,"source_line":27,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":4139,"record_offset":45504,"parent_record_offset":45480,"ancestor_record_offset":44580,"inlinee":125478,"source_line":27,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::_::impl$0::deserialize::impl$2::visit_map
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":4139,"record_offset":45040,"parent_record_offset":44828,"ancestor_record_offset":44580,"inlinee":125488,"source_line":27,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4139,"record_offset":46020,"parent_record_offset":45992,"ancestor_record_offset":44580,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":44980,"parent_record_offset":44828,"ancestor_record_offset":44580,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":45536,"parent_record_offset":45184,"ancestor_record_offset":44580,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4139,"record_offset":46556,"parent_record_offset":45040,"ancestor_record_offset":44580,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":48180,"parent_record_offset":44828,"ancestor_record_offset":44580,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":45368,"parent_record_offset":45184,"ancestor_record_offset":44580,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":48120,"parent_record_offset":45040,"ancestor_record_offset":44580,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4139,"record_offset":46048,"parent_record_offset":46020,"ancestor_record_offset":44580,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4139,"record_offset":46076,"parent_record_offset":46048,"ancestor_record_offset":44580,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4139,"record_offset":45992,"parent_record_offset":45956,"ancestor_record_offset":44580,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::CommaSeparated::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":45012,"parent_record_offset":44828,"ancestor_record_offset":44580,"inlinee":13279,"source_line":791,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::id::Deserializer::new
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/id.rs
//     evidence={"module":4139,"record_offset":45440,"parent_record_offset":45184,"ancestor_record_offset":44580,"inlinee":13309,"source_line":11,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$1::handle_struct_after_name::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":48228,"parent_record_offset":48180,"ancestor_record_offset":44580,"inlinee":13282,"source_line":240,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":45832,"parent_record_offset":45792,"ancestor_record_offset":44580,"inlinee":13285,"source_line":747,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_key_seed
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":45184,"parent_record_offset":45148,"ancestor_record_offset":44580,"inlinee":13307,"source_line":838,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_value_seed
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":46608,"parent_record_offset":46584,"ancestor_record_offset":44580,"inlinee":13322,"source_line":857,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_enum
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4139,"record_offset":47252,"parent_record_offset":47228,"ancestor_record_offset":44580,"inlinee":5564,"source_line":233,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_option
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4139,"record_offset":46804,"parent_record_offset":46780,"ancestor_record_offset":44580,"inlinee":6910,"source_line":160,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":45148,"parent_record_offset":45040,"ancestor_record_offset":44580,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":46584,"parent_record_offset":45040,"ancestor_record_offset":44580,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":45920,"parent_record_offset":45832,"ancestor_record_offset":44580,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":45480,"parent_record_offset":45184,"ancestor_record_offset":44580,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$16::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4139,"record_offset":46780,"parent_record_offset":46756,"ancestor_record_offset":44580,"inlinee":6990,"source_line":931,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::impl$2::deserialize_struct<asset_pipe::texture::_::impl$0::deserialize::__Visitor>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4139,"record_offset":34960,"rva":38061312,"va":5406770432}
// 编译实例 RVAs: 0x244c500
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140b73a50
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26055052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406770432,"raw_hex":"e8746e72fe","size":5,"soft_fail":false,"target_va":5380717136,"va":5406772183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142447720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406770432,"raw_hex":"e856acffff","size":5,"soft_fail":false,"target_va":5406750496,"va":5406771909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14245bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t62550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406770432,"raw_hex":"e856f40000","size":5,"soft_fail":false,"target_va":5406833440,"va":5406770885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::CommaSeparated::has_element
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25685247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406770432,"raw_hex":"e8011378fe","size":5,"soft_fail":false,"target_va":5381085520,"va":5406770762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::impl$6::next_value_seed<core::marker::PhantomData<serde::de::ignored_any::IgnoredAny> >
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406770432,"raw_hex":"e8de9bffff","size":5,"soft_fail":false,"target_va":5406746112,"va":5406771741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::struct_error_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25687490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406770432,"raw_hex":"e83e0a78fe","size":5,"soft_fail":false,"target_va":5381085680,"va":5406773165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::comma
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25708043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406770432,"raw_hex":"e8f5b977fe","size":5,"soft_fail":false,"target_va":5381064016,"va":5406772054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_char
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25723927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406770432,"raw_hex":"e8e97b77fe","size":5,"soft_fail":false,"target_va":5381046704,"va":5406770626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_struct_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25704684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406770432,"raw_hex":"e814c777fe","size":5,"soft_fail":false,"target_va":5381065872,"va":5406770551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::identifier
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25701716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406770432,"raw_hex":"e8acd277fe","size":5,"soft_fail":false,"target_va":5381069696,"va":5406771407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_ws
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25699422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406770432,"raw_hex":"e8a2db77fe","size":5,"soft_fail":false,"target_va":5381071168,"va":5406770585,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":35144,"parent_record_offset":34960,"ancestor_record_offset":34960,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":38072,"parent_record_offset":34960,"ancestor_record_offset":34960,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] ron::de::Deserializer::handle_struct_after_name
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":35216,"parent_record_offset":34960,"ancestor_record_offset":34960,"inlinee":13303,"source_line":225,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":4139,"record_offset":36192,"parent_record_offset":36168,"ancestor_record_offset":34960,"inlinee":108253,"source_line":38,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":4139,"record_offset":35792,"parent_record_offset":35768,"ancestor_record_offset":34960,"inlinee":125483,"source_line":38,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::_::impl$0::deserialize::impl$2::visit_map
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":4139,"record_offset":35432,"parent_record_offset":35216,"ancestor_record_offset":34960,"inlinee":125481,"source_line":38,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4139,"record_offset":36240,"parent_record_offset":36216,"ancestor_record_offset":34960,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":35372,"parent_record_offset":35216,"ancestor_record_offset":34960,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":35824,"parent_record_offset":35516,"ancestor_record_offset":34960,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4139,"record_offset":36488,"parent_record_offset":35432,"ancestor_record_offset":34960,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4139,"record_offset":36952,"parent_record_offset":36552,"ancestor_record_offset":34960,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":37804,"parent_record_offset":35216,"ancestor_record_offset":34960,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":35660,"parent_record_offset":35516,"ancestor_record_offset":34960,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":37896,"parent_record_offset":35216,"ancestor_record_offset":34960,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4139,"record_offset":36264,"parent_record_offset":36240,"ancestor_record_offset":34960,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4139,"record_offset":36288,"parent_record_offset":36264,"ancestor_record_offset":34960,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4139,"record_offset":36216,"parent_record_offset":36192,"ancestor_record_offset":34960,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":4139,"record_offset":37024,"parent_record_offset":37000,"ancestor_record_offset":34960,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::CommaSeparated::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":35404,"parent_record_offset":35216,"ancestor_record_offset":34960,"inlinee":13279,"source_line":791,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::id::Deserializer::new
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/id.rs
//     evidence={"module":4139,"record_offset":35728,"parent_record_offset":35516,"ancestor_record_offset":34960,"inlinee":13309,"source_line":11,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$1::handle_struct_after_name::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":37828,"parent_record_offset":37804,"ancestor_record_offset":34960,"inlinee":13282,"source_line":240,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":36096,"parent_record_offset":36068,"ancestor_record_offset":34960,"inlinee":13285,"source_line":747,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_key_seed
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":35516,"parent_record_offset":35484,"ancestor_record_offset":34960,"inlinee":13307,"source_line":838,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_value_seed
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":36552,"parent_record_offset":36516,"ancestor_record_offset":34960,"inlinee":13322,"source_line":857,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_map
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4139,"record_offset":36784,"parent_record_offset":36760,"ancestor_record_offset":34960,"inlinee":100077,"source_line":214,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":35484,"parent_record_offset":35432,"ancestor_record_offset":34960,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":36424,"parent_record_offset":35432,"ancestor_record_offset":34960,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":36168,"parent_record_offset":36096,"ancestor_record_offset":34960,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":35768,"parent_record_offset":35516,"ancestor_record_offset":34960,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$204::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4139,"record_offset":36760,"parent_record_offset":36736,"ancestor_record_offset":34960,"inlinee":73646,"source_line":1505,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::impl$2::deserialize_struct<serde::de::impls::impl$36::deserialize::DurationVisitor>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4139,"record_offset":34360,"rva":38060656,"va":5406769776}
// 编译实例 RVAs: 0x244c270
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14268d400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2363604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406769776,"raw_hex":"e8d4102400","size":5,"soft_fail":false,"target_va":5409133568,"va":5406769959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::struct_error_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25684618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406769776,"raw_hex":"e8761578fe","size":5,"soft_fail":false,"target_va":5381085680,"va":5406770293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_char
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25723189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406769776,"raw_hex":"e8cb7e77fe","size":5,"soft_fail":false,"target_va":5381046704,"va":5406769888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_struct_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25703957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406769776,"raw_hex":"e8ebc977fe","size":5,"soft_fail":false,"target_va":5381065872,"va":5406769824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_ws
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25698691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406769776,"raw_hex":"e87dde77fe","size":5,"soft_fail":false,"target_va":5381071168,"va":5406769854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":34544,"parent_record_offset":34360,"ancestor_record_offset":34360,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":34928,"parent_record_offset":34360,"ancestor_record_offset":34360,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] ron::de::Deserializer::handle_struct_after_name
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":34608,"parent_record_offset":34360,"ancestor_record_offset":34360,"inlinee":13303,"source_line":225,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":34708,"parent_record_offset":34608,"ancestor_record_offset":34360,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":34840,"parent_record_offset":34608,"ancestor_record_offset":34360,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":34768,"parent_record_offset":34608,"ancestor_record_offset":34360,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":34868,"parent_record_offset":34608,"ancestor_record_offset":34360,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":34896,"parent_record_offset":34608,"ancestor_record_offset":34360,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::CommaSeparated::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":34740,"parent_record_offset":34608,"ancestor_record_offset":34360,"inlinee":13279,"source_line":791,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$1::handle_struct_after_name::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":34800,"parent_record_offset":34768,"ancestor_record_offset":34360,"inlinee":13282,"source_line":240,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::impl$2::deserialize_tuple<serde::de::ignored_any::IgnoredAny>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4139,"record_offset":29696,"rva":38053296,"va":5406762416}
// 编译实例 RVAs: 0x244a5b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142447170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13583","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406762416,"raw_hex":"e8f1caffff","size":5,"soft_fail":false,"target_va":5406749040,"va":5406762618,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::CommaSeparated::has_element
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25677008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406762416,"raw_hex":"e8303378fe","size":5,"soft_fail":false,"target_va":5381085520,"va":5406762523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::comma
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25698657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406762416,"raw_hex":"e89fde77fe","size":5,"soft_fail":false,"target_va":5381064016,"va":5406762668,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_char
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25715756","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406762416,"raw_hex":"e8d49b77fe","size":5,"soft_fail":false,"target_va":5381046704,"va":5406762455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_ws
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25691747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406762416,"raw_hex":"e89df977fe","size":5,"soft_fail":false,"target_va":5381071168,"va":5406762910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":29872,"parent_record_offset":29696,"ancestor_record_offset":29696,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":30404,"parent_record_offset":29696,"ancestor_record_offset":29696,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":30432,"parent_record_offset":29696,"ancestor_record_offset":29696,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":30376,"parent_record_offset":29696,"ancestor_record_offset":29696,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] ron::de::CommaSeparated::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":29904,"parent_record_offset":29696,"ancestor_record_offset":29696,"inlinee":13279,"source_line":791,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] serde::de::ignored_any::impl$0::visit_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":4139,"record_offset":29932,"parent_record_offset":29696,"ancestor_record_offset":29696,"inlinee":13378,"source_line":192,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":30120,"parent_record_offset":30012,"ancestor_record_offset":29696,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":30280,"parent_record_offset":30012,"ancestor_record_offset":29696,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":30088,"parent_record_offset":30012,"ancestor_record_offset":29696,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_ignored_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":30208,"parent_record_offset":30184,"ancestor_record_offset":29696,"inlinee":6889,"source_line":758,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$5::next_element_seed
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":30012,"parent_record_offset":29980,"ancestor_record_offset":29696,"inlinee":13377,"source_line":819,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":29980,"parent_record_offset":29932,"ancestor_record_offset":29696,"inlinee":6504,"source_line":1724,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":4139,"record_offset":30184,"parent_record_offset":30160,"ancestor_record_offset":29696,"inlinee":6518,"source_line":232,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":30160,"parent_record_offset":30012,"ancestor_record_offset":29696,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::impl$2::deserialize_tuple<serde::de::impls::ArrayVisitor<array$<u32,2> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4139,"record_offset":28544,"rva":38052144,"va":5406761264}
// 编译实例 RVAs: 0x244a130
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::invalid_length
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4718800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406761264,"raw_hex":"e8d0004800","size":5,"soft_fail":false,"target_va":5411480896,"va":5406762091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142690050
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2383465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406761264,"raw_hex":"e8695e2400","size":5,"soft_fail":false,"target_va":5409144912,"va":5406761442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::CommaSeparated::has_element
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25675856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406761264,"raw_hex":"e8b03778fe","size":5,"soft_fail":false,"target_va":5381085520,"va":5406761371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::comma
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25697494","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406761264,"raw_hex":"e82ae377fe","size":5,"soft_fail":false,"target_va":5381064016,"va":5406761505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_char
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25714601","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406761264,"raw_hex":"e857a077fe","size":5,"soft_fail":false,"target_va":5381046704,"va":5406761300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_ws
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25690796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406761264,"raw_hex":"e854fd77fe","size":5,"soft_fail":false,"target_va":5381071168,"va":5406761959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":28732,"parent_record_offset":28544,"ancestor_record_offset":28544,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":29600,"parent_record_offset":28544,"ancestor_record_offset":28544,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":29628,"parent_record_offset":28544,"ancestor_record_offset":28544,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":29664,"parent_record_offset":28544,"ancestor_record_offset":28544,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] ron::de::CommaSeparated::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":28764,"parent_record_offset":28544,"ancestor_record_offset":28544,"inlinee":13279,"source_line":791,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impls::impl$94::visit_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4139,"record_offset":28812,"parent_record_offset":28544,"ancestor_record_offset":28544,"inlinee":13382,"source_line":1292,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":29024,"parent_record_offset":28928,"ancestor_record_offset":28544,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":29196,"parent_record_offset":28928,"ancestor_record_offset":28544,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":28996,"parent_record_offset":28928,"ancestor_record_offset":28544,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_u32
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":29120,"parent_record_offset":29092,"ancestor_record_offset":28544,"inlinee":5581,"source_line":402,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$5::next_element_seed
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":28928,"parent_record_offset":28892,"ancestor_record_offset":28544,"inlinee":13377,"source_line":819,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":28892,"parent_record_offset":28812,"ancestor_record_offset":28544,"inlinee":6504,"source_line":1724,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":29064,"parent_record_offset":28928,"ancestor_record_offset":28544,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$69::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4139,"record_offset":29092,"parent_record_offset":29064,"ancestor_record_offset":28544,"inlinee":5579,"source_line":134,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::impl$5::next_element_seed<core::marker::PhantomData<relative_path::RelativePathBuf> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4139,"record_offset":10028,"rva":38039232,"va":5406748352}
// 编译实例 RVAs: 0x2446ec0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31667058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406748352,"raw_hex":"e88ecc1cfe","size":5,"soft_fail":false,"target_va":5375081488,"va":5406748541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31667354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406748352,"raw_hex":"e866cb1cfe","size":5,"soft_fail":false,"target_va":5375081504,"va":5406748853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5098264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406748352,"raw_hex":"e818cb4d00","size":5,"soft_fail":false,"target_va":5411847168,"va":5406748899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4792010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406748352,"raw_hex":"e8ca1e4900","size":5,"soft_fail":false,"target_va":5411540963,"va":5406748948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::CommaSeparated::has_element
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25662880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406748352,"raw_hex":"e8606a78fe","size":5,"soft_fail":false,"target_va":5381085520,"va":5406748395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::comma
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25684732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406748352,"raw_hex":"e8041578fe","size":5,"soft_fail":false,"target_va":5381064016,"va":5406748743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::string
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25674469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406748352,"raw_hex":"e81b3d78fe","size":5,"soft_fail":false,"target_va":5381074000,"va":5406748464,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":10264,"parent_record_offset":10028,"ancestor_record_offset":10028,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":11228,"parent_record_offset":10028,"ancestor_record_offset":10028,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4139,"record_offset":11320,"parent_record_offset":10028,"ancestor_record_offset":10028,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":10228,"parent_record_offset":10028,"ancestor_record_offset":10028,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":11172,"parent_record_offset":10028,"ancestor_record_offset":10028,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":10296,"parent_record_offset":10028,"ancestor_record_offset":10028,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4139,"record_offset":10996,"parent_record_offset":10972,"ancestor_record_offset":10028,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4139,"record_offset":11020,"parent_record_offset":10996,"ancestor_record_offset":10028,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4139,"record_offset":11520,"parent_record_offset":11496,"ancestor_record_offset":10028,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4139,"record_offset":10972,"parent_record_offset":10796,"ancestor_record_offset":10028,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4139,"record_offset":11496,"parent_record_offset":11440,"ancestor_record_offset":10028,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4139,"record_offset":10736,"parent_record_offset":10708,"ancestor_record_offset":10028,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4139,"record_offset":11468,"parent_record_offset":11440,"ancestor_record_offset":10028,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4139,"record_offset":11440,"parent_record_offset":11416,"ancestor_record_offset":10028,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4139,"record_offset":10796,"parent_record_offset":10764,"ancestor_record_offset":10028,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4139,"record_offset":10764,"parent_record_offset":10736,"ancestor_record_offset":10028,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4139,"record_offset":11416,"parent_record_offset":11392,"ancestor_record_offset":10028,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4139,"record_offset":10832,"parent_record_offset":10796,"ancestor_record_offset":10028,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4139,"record_offset":10672,"parent_record_offset":10640,"ancestor_record_offset":10028,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4139,"record_offset":10640,"parent_record_offset":10608,"ancestor_record_offset":10028,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4139,"record_offset":10576,"parent_record_offset":10544,"ancestor_record_offset":10028,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4139,"record_offset":10608,"parent_record_offset":10576,"ancestor_record_offset":10028,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4139,"record_offset":10544,"parent_record_offset":10512,"ancestor_record_offset":10028,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":4139,"record_offset":10512,"parent_record_offset":10480,"ancestor_record_offset":10028,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4139,"record_offset":10708,"parent_record_offset":10672,"ancestor_record_offset":10028,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":4139,"record_offset":10856,"parent_record_offset":10832,"ancestor_record_offset":10028,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":4139,"record_offset":10880,"parent_record_offset":10856,"ancestor_record_offset":10028,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":4139,"record_offset":11096,"parent_record_offset":11072,"ancestor_record_offset":10028,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":4139,"record_offset":10928,"parent_record_offset":10904,"ancestor_record_offset":10028,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":10904,"parent_record_offset":10880,"ancestor_record_offset":10028,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":4139,"record_offset":11072,"parent_record_offset":10672,"ancestor_record_offset":10028,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4139,"record_offset":11344,"parent_record_offset":11320,"ancestor_record_offset":10028,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":10412,"parent_record_offset":10368,"ancestor_record_offset":10028,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] relative_path::impl$56::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"relative-path","version":"1.9.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-1.9.0/src/lib.rs
//     evidence={"module":4139,"record_offset":10332,"parent_record_offset":10296,"ancestor_record_offset":10028,"inlinee":31418,"source_line":2081,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] relative_path::impl$56::deserialize::impl$0::visit_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"relative-path","version":"1.9.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-1.9.0/src/lib.rs
//     evidence={"module":4139,"record_offset":10480,"parent_record_offset":10448,"ancestor_record_offset":10028,"inlinee":72617,"source_line":2104,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_str
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":10368,"parent_record_offset":10332,"ancestor_record_offset":10028,"inlinee":5163,"source_line":445,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":10448,"parent_record_offset":10368,"ancestor_record_offset":10028,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::impl$5::next_element_seed<core::marker::PhantomData<u8> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4139,"record_offset":9548,"rva":38038912,"va":5406748032}
// 编译实例 RVAs: 0x2446d80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142690810
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2398769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406748032,"raw_hex":"e8319a2400","size":5,"soft_fail":false,"target_va":5409146896,"va":5406748122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::CommaSeparated::has_element
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25662535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406748032,"raw_hex":"e8b96b78fe","size":5,"soft_fail":false,"target_va":5381085520,"va":5406748050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::comma
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25684176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406748032,"raw_hex":"e8301778fe","size":5,"soft_fail":false,"target_va":5381064016,"va":5406748187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":9744,"parent_record_offset":9548,"ancestor_record_offset":9548,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":9908,"parent_record_offset":9548,"ancestor_record_offset":9548,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":9716,"parent_record_offset":9548,"ancestor_record_offset":9548,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":9996,"parent_record_offset":9548,"ancestor_record_offset":9548,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":9776,"parent_record_offset":9548,"ancestor_record_offset":9548,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":9860,"parent_record_offset":9832,"ancestor_record_offset":9548,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_u8
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":9832,"parent_record_offset":9804,"ancestor_record_offset":9548,"inlinee":13362,"source_line":388,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$63::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4139,"record_offset":9804,"parent_record_offset":9776,"ancestor_record_offset":9548,"inlinee":6547,"source_line":134,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::impl$6::next_key_seed<core::marker::PhantomData<serde::de::ignored_any::IgnoredAny> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4139,"record_offset":6748,"rva":38036512,"va":5406745632}
// 编译实例 RVAs: 0x2446420
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142447170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406745632,"raw_hex":"e88c0b0000","size":5,"soft_fail":false,"target_va":5406749040,"va":5406746079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14245c680
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t90481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406745632,"raw_hex":"e871610100","size":5,"soft_fail":false,"target_va":5406836352,"va":5406745866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::CommaSeparated::has_element
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25660137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406745632,"raw_hex":"e8177578fe","size":5,"soft_fail":false,"target_va":5381085520,"va":5406745652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":6980,"parent_record_offset":6748,"ancestor_record_offset":6748,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":7200,"parent_record_offset":6748,"ancestor_record_offset":6748,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":6948,"parent_record_offset":6748,"ancestor_record_offset":6748,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":7040,"parent_record_offset":6748,"ancestor_record_offset":6748,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] ron::de::id::Deserializer::new
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/id.rs
//     evidence={"module":4139,"record_offset":7012,"parent_record_offset":6748,"ancestor_record_offset":6748,"inlinee":13309,"source_line":11,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":7128,"parent_record_offset":6748,"ancestor_record_offset":6748,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_ignored_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":7308,"parent_record_offset":7284,"ancestor_record_offset":6748,"inlinee":6889,"source_line":758,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":4139,"record_offset":7168,"parent_record_offset":7128,"ancestor_record_offset":6748,"inlinee":6518,"source_line":232,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::impl$6::next_key_seed<core::marker::PhantomData<serde::de::impls::impl$36::deserialize::Field> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4139,"record_offset":6164,"rva":38036032,"va":5406745152}
// 编译实例 RVAs: 0x2446240
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142450470
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406745152,"raw_hex":"e86ba00000","size":5,"soft_fail":false,"target_va":5406786672,"va":5406745600,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14245c530
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t90625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406745152,"raw_hex":"e801620100","size":5,"soft_fail":false,"target_va":5406836016,"va":5406745386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::CommaSeparated::has_element
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25659657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406745152,"raw_hex":"e8f77678fe","size":5,"soft_fail":false,"target_va":5381085520,"va":5406745172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":6408,"parent_record_offset":6164,"ancestor_record_offset":6164,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":6628,"parent_record_offset":6164,"ancestor_record_offset":6164,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":6376,"parent_record_offset":6164,"ancestor_record_offset":6164,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":6468,"parent_record_offset":6164,"ancestor_record_offset":6164,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] ron::de::id::Deserializer::new
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/id.rs
//     evidence={"module":4139,"record_offset":6440,"parent_record_offset":6164,"ancestor_record_offset":6164,"inlinee":13309,"source_line":11,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":6556,"parent_record_offset":6164,"ancestor_record_offset":6164,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$36::deserialize::impl$0::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4139,"record_offset":6596,"parent_record_offset":6556,"ancestor_record_offset":6164,"inlinee":125480,"source_line":2287,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::impl$6::next_value_seed<core::marker::PhantomData<serde::de::ignored_any::IgnoredAny> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4139,"record_offset":7348,"rva":38036992,"va":5406746112}
// 编译实例 RVAs: 0x2446600
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142447170
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406746112,"raw_hex":"e8f20a0000","size":5,"soft_fail":false,"target_va":5406749040,"va":5406746233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::comma
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25682272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406746112,"raw_hex":"e8a01e78fe","size":5,"soft_fail":false,"target_va":5381064016,"va":5406746283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25699467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406746112,"raw_hex":"e875db77fe","size":5,"soft_fail":false,"target_va":5381046704,"va":5406746166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_ws
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25674979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406746112,"raw_hex":"e81d3b78fe","size":5,"soft_fail":false,"target_va":5381071168,"va":5406746142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":7600,"parent_record_offset":7348,"ancestor_record_offset":7348,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":7776,"parent_record_offset":7348,"ancestor_record_offset":7348,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":7544,"parent_record_offset":7348,"ancestor_record_offset":7348,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":7868,"parent_record_offset":7348,"ancestor_record_offset":7348,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":7632,"parent_record_offset":7348,"ancestor_record_offset":7348,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_ignored_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":7704,"parent_record_offset":7680,"ancestor_record_offset":7348,"inlinee":6889,"source_line":758,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_ignored_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4139,"record_offset":7680,"parent_record_offset":7656,"ancestor_record_offset":7348,"inlinee":6911,"source_line":245,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":4139,"record_offset":7656,"parent_record_offset":7632,"ancestor_record_offset":7348,"inlinee":6518,"source_line":232,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::impl$6::next_value_seed<core::marker::PhantomData<u32> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4139,"record_offset":8500,"rva":38037776,"va":5406746896}
// 编译实例 RVAs: 0x2446910
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142690050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2397889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406746896,"raw_hex":"e8c1962400","size":5,"soft_fail":false,"target_va":5409144912,"va":5406747018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::comma
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25683065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406746896,"raw_hex":"e8871b78fe","size":5,"soft_fail":false,"target_va":5381064016,"va":5406747076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25700252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406746896,"raw_hex":"e864d877fe","size":5,"soft_fail":false,"target_va":5381046704,"va":5406746951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_ws
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25675764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406746896,"raw_hex":"e80c3878fe","size":5,"soft_fail":false,"target_va":5381071168,"va":5406746927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":8724,"parent_record_offset":8500,"ancestor_record_offset":8500,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":8920,"parent_record_offset":8500,"ancestor_record_offset":8500,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":8668,"parent_record_offset":8500,"ancestor_record_offset":8500,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":9012,"parent_record_offset":8500,"ancestor_record_offset":8500,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":8756,"parent_record_offset":8500,"ancestor_record_offset":8500,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":8868,"parent_record_offset":8840,"ancestor_record_offset":8500,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_u32
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":8840,"parent_record_offset":8812,"ancestor_record_offset":8500,"inlinee":5581,"source_line":402,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_u32
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4139,"record_offset":8812,"parent_record_offset":8784,"ancestor_record_offset":8500,"inlinee":5580,"source_line":96,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$69::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4139,"record_offset":8784,"parent_record_offset":8756,"ancestor_record_offset":8500,"inlinee":5579,"source_line":134,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::impl$6::next_value_seed<core::marker::PhantomData<u64> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4139,"record_offset":7928,"rva":38037360,"va":5406746480}
// 编译实例 RVAs: 0x2446770
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14268f890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2396318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406746480,"raw_hex":"e89e902400","size":5,"soft_fail":false,"target_va":5409142928,"va":5406746605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::comma
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25682653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406746480,"raw_hex":"e8231d78fe","size":5,"soft_fail":false,"target_va":5381064016,"va":5406746664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25699835","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406746480,"raw_hex":"e805da77fe","size":5,"soft_fail":false,"target_va":5381046704,"va":5406746534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_ws
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25675347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406746480,"raw_hex":"e8ad3978fe","size":5,"soft_fail":false,"target_va":5381071168,"va":5406746510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":8152,"parent_record_offset":7928,"ancestor_record_offset":7928,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":8348,"parent_record_offset":7928,"ancestor_record_offset":7928,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":8096,"parent_record_offset":7928,"ancestor_record_offset":7928,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":8440,"parent_record_offset":7928,"ancestor_record_offset":7928,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":8184,"parent_record_offset":7928,"ancestor_record_offset":7928,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":8296,"parent_record_offset":8268,"ancestor_record_offset":7928,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_u64
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":8268,"parent_record_offset":8240,"ancestor_record_offset":7928,"inlinee":6887,"source_line":409,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_u64
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/tag.rs
//     evidence={"module":4139,"record_offset":8240,"parent_record_offset":8212,"ancestor_record_offset":7928,"inlinee":6907,"source_line":103,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$72::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4139,"record_offset":8212,"parent_record_offset":8184,"ancestor_record_offset":7928,"inlinee":11548,"source_line":134,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::impl$8::variant_seed<core::marker::PhantomData<asset::texture::desc::_::impl$0::deserialize::__Field> >
// 记录形式: compiled_procedure; instances=3
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4139,"record_offset":2540,"rva":38033888,"va":5406743008}
// 编译实例 RVAs: 0x24459e0, 0x2445bc0, 0x2445da0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ron..error..Error$u20$as$u20$serde..de..Error$GT$::unknown_variant
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t4737750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406743008,"raw_hex":"e8d64a4800","size":5,"soft_fail":false,"target_va":5411481152,"va":5406743397,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::identifier
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25673390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406743008,"raw_hex":"e8524178fe","size":5,"soft_fail":false,"target_va":5381069696,"va":5406743081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_ws
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25671873","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406743008,"raw_hex":"e83f4778fe","size":5,"soft_fail":false,"target_va":5381071168,"va":5406743036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":2784,"parent_record_offset":2540,"ancestor_record_offset":2540,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":3484,"parent_record_offset":2540,"ancestor_record_offset":2540,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":2752,"parent_record_offset":2540,"ancestor_record_offset":2540,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":3456,"parent_record_offset":2540,"ancestor_record_offset":2540,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":2816,"parent_record_offset":2540,"ancestor_record_offset":2540,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":4139,"record_offset":2980,"parent_record_offset":2952,"ancestor_record_offset":2540,"inlinee":108176,"source_line":27,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":4139,"record_offset":2848,"parent_record_offset":2816,"ancestor_record_offset":2540,"inlinee":125478,"source_line":27,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4139,"record_offset":3036,"parent_record_offset":3008,"ancestor_record_offset":2540,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":2916,"parent_record_offset":2876,"ancestor_record_offset":2540,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":3376,"parent_record_offset":2876,"ancestor_record_offset":2540,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4139,"record_offset":3064,"parent_record_offset":3036,"ancestor_record_offset":2540,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4139,"record_offset":3092,"parent_record_offset":3064,"ancestor_record_offset":2540,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4139,"record_offset":3008,"parent_record_offset":2980,"ancestor_record_offset":2540,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":2876,"parent_record_offset":2848,"ancestor_record_offset":2540,"inlinee":13285,"source_line":747,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":2952,"parent_record_offset":2876,"ancestor_record_offset":2540,"inlinee":5165,"source_line":1504,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::de::impl$9::newtype_variant_seed<core::marker::PhantomData<u32> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4139,"record_offset":5420,"rva":38035360,"va":5406744480}
// 编译实例 RVAs: 0x2445fa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142690050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2400288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406744480,"raw_hex":"e820a02400","size":5,"soft_fail":false,"target_va":5409144912,"va":5406744619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::struct_error_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25659149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406744480,"raw_hex":"e8f37878fe","size":5,"soft_fail":false,"target_va":5381085680,"va":5406744824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::comma
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25680942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406744480,"raw_hex":"e8d22378fe","size":5,"soft_fail":false,"target_va":5381064016,"va":5406744953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_char
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25697842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406744480,"raw_hex":"e8cee177fe","size":5,"soft_fail":false,"target_va":5381046704,"va":5406744541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_ws
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25673358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406744480,"raw_hex":"e8724178fe","size":5,"soft_fail":false,"target_va":5381071168,"va":5406744521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":5752,"parent_record_offset":5420,"ancestor_record_offset":5420,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4139,"record_offset":6040,"parent_record_offset":5420,"ancestor_record_offset":5420,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":5984,"parent_record_offset":5420,"ancestor_record_offset":5420,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":5600,"parent_record_offset":5420,"ancestor_record_offset":5420,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":6104,"parent_record_offset":5420,"ancestor_record_offset":5420,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] ron::extensions::Extensions::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.9.0/src/lib.rs
//     evidence={"module":4139,"record_offset":5656,"parent_record_offset":5420,"ancestor_record_offset":5420,"inlinee":12958,"source_line":703,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4139,"record_offset":5784,"parent_record_offset":5420,"ancestor_record_offset":5420,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4139,"record_offset":5868,"parent_record_offset":5840,"ancestor_record_offset":5420,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_u32
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":5840,"parent_record_offset":5812,"ancestor_record_offset":5420,"inlinee":5581,"source_line":402,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$9::newtype_variant_seed::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/de/mod.rs
//     evidence={"module":4139,"record_offset":6008,"parent_record_offset":5984,"ancestor_record_offset":5420,"inlinee":13355,"source_line":932,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::extensions::_::InternalBitFlags::bits
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.9.0/src/lib.rs
//     evidence={"module":4139,"record_offset":5712,"parent_record_offset":5684,"ancestor_record_offset":5420,"inlinee":13353,"source_line":641,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] ron::extensions::_::InternalBitFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.9.0/src/lib.rs
//     evidence={"module":4139,"record_offset":5684,"parent_record_offset":5656,"ancestor_record_offset":5420,"inlinee":12959,"source_line":703,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$69::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4139,"record_offset":5812,"parent_record_offset":5784,"ancestor_record_offset":5420,"inlinee":5579,"source_line":134,"archive":"_pdb/B/modules/04139.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::options::Options::from_str_seed<core::marker::PhantomData<alloc::vec::Vec<relative_path::RelativePathBuf,alloc::alloc::Global> >,alloc::vec::Vec<relative_path::RelativePathBuf,alloc::alloc::Global> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3166,"record_offset":1100,"rva":34439008,"va":5403148128}
// 编译实例 RVAs: 0x20d7f60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f8f70
//     owner=unresolved_namespace; count=1
//     target_aliases=18 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28177298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403148128,"raw_hex":"e86e0c52fe","size":5,"soft_fail":false,"target_va":5374971760,"va":5403149053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28067456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403148128,"raw_hex":"e880b953fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5403148955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1424494f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3609690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403148128,"raw_hex":"e85a143700","size":5,"soft_fail":false,"target_va":5406758128,"va":5403148433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::Deserializer::end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-22063131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403148128,"raw_hex":"e8e557affe","size":5,"soft_fail":false,"target_va":5381085360,"va":5403148486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::Deserializer::from_str_with_options
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-22063867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403148128,"raw_hex":"e80555affe","size":5,"soft_fail":false,"target_va":5381084320,"va":5403148182,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::de::Deserializer::span_error
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-22063489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403148128,"raw_hex":"e87f56affe","size":5,"soft_fail":false,"target_va":5381085120,"va":5403148604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3166,"record_offset":1700,"parent_record_offset":1100,"ancestor_record_offset":1100,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03166.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3166,"record_offset":1488,"parent_record_offset":1100,"ancestor_record_offset":1100,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/03166.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3166,"record_offset":1404,"parent_record_offset":1100,"ancestor_record_offset":1100,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/03166.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3166,"record_offset":1644,"parent_record_offset":1100,"ancestor_record_offset":1100,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/03166.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":3166,"record_offset":1432,"parent_record_offset":1100,"ancestor_record_offset":1100,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/03166.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3166,"record_offset":2000,"parent_record_offset":1976,"ancestor_record_offset":1100,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03166.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3166,"record_offset":1976,"parent_record_offset":1912,"ancestor_record_offset":1100,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03166.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3166,"record_offset":1944,"parent_record_offset":1912,"ancestor_record_offset":1100,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03166.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3166,"record_offset":1912,"parent_record_offset":1884,"ancestor_record_offset":1100,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03166.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3166,"record_offset":1884,"parent_record_offset":1856,"ancestor_record_offset":1100,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03166.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3166,"record_offset":1724,"parent_record_offset":1700,"ancestor_record_offset":1100,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/03166.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3166,"record_offset":2168,"parent_record_offset":2140,"ancestor_record_offset":1100,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/03166.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3166,"record_offset":1748,"parent_record_offset":1724,"ancestor_record_offset":1100,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03166.jsonl.gz"}
//   [pdb_inline_transitive] ron::options::impl$2::from_str_seed::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/options.rs
//     evidence={"module":3166,"record_offset":1520,"parent_record_offset":1488,"ancestor_record_offset":1100,"inlinee":15033,"source_line":163,"archive":"_pdb/B/modules/03166.jsonl.gz"}
//   [pdb_inline_transitive] ron::options::impl$2::from_str_seed::closure$1
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/options.rs
//     evidence={"module":3166,"record_offset":1584,"parent_record_offset":1552,"ancestor_record_offset":1100,"inlinee":15034,"source_line":165,"archive":"_pdb/B/modules/03166.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":3166,"record_offset":1456,"parent_record_offset":1432,"ancestor_record_offset":1100,"inlinee":6469,"source_line":1150,"archive":"_pdb/B/modules/03166.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::parse::Parser::integer<u32>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4601,"record_offset":16524,"rva":40435792,"va":5409144912}
// 编译实例 RVAs: 0x2690050
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$u32$u20$as$u20$ron..parse..Integer$GT$::parse
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-28064245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409144912,"raw_hex":"e80bc653fe","size":5,"soft_fail":false,"target_va":5381082560,"va":5409146800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$u32$u20$as$u20$ron..parse..Integer$GT$::try_from_parsed_integer
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-28063802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409144912,"raw_hex":"e8c6c753fe","size":5,"soft_fail":false,"target_va":5381082592,"va":5409146389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<enum2$<ron::parse::ParsedInteger>,enum2$<ron::error::Error> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409144912,"raw_hex":"e8c4070000","size":5,"soft_fail":false,"target_va":5409148880,"va":5409146887,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t2398066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409144912,"raw_hex":"e872972400","size":5,"soft_fail":false,"target_va":5411543360,"va":5409145289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28099490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409144912,"raw_hex":"e85e3c53fe","size":5,"soft_fail":false,"target_va":5381046704,"va":5409146189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_ident
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28079654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409144912,"raw_hex":"e8da8953fe","size":5,"soft_fail":false,"target_va":5381065728,"va":5409145377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_str
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28098469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409144912,"raw_hex":"e85b4053fe","size":5,"soft_fail":false,"target_va":5381046576,"va":5409145040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::next_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28099497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409144912,"raw_hex":"e8573c53fe","size":5,"soft_fail":false,"target_va":5381045568,"va":5409145060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::next_chars_while_len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28097352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409144912,"raw_hex":"e8b84453fe","size":5,"soft_fail":false,"target_va":5381047888,"va":5409145235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_escape
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28066321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409144912,"raw_hex":"e8efbd53fe","size":5,"soft_fail":false,"target_va":5381078784,"va":5409145100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<i16>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-9951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409144912,"raw_hex":"e821d9ffff","size":5,"soft_fail":false,"target_va":5409135664,"va":5409145610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<i32>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-6451","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409144912,"raw_hex":"e8cde6ffff","size":5,"soft_fail":false,"target_va":5409139280,"va":5409145726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<i64>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-9284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409144912,"raw_hex":"e8bcdbffff","size":5,"soft_fail":false,"target_va":5409136560,"va":5409145839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<i8>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-7049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409144912,"raw_hex":"e877e4ffff","size":5,"soft_fail":false,"target_va":5409138384,"va":5409145428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<u16>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-8694","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409144912,"raw_hex":"e80adeffff","size":5,"soft_fail":false,"target_va":5409137456,"va":5409146145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<u32>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-4324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409144912,"raw_hex":"e81cefffff","size":5,"soft_fail":false,"target_va":5409142016,"va":5409146335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<u64>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-6295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409144912,"raw_hex":"e869e7ffff","size":5,"soft_fail":false,"target_va":5409140176,"va":5409146466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<u8>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-4856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409144912,"raw_hex":"e808edffff","size":5,"soft_fail":false,"target_va":5409141104,"va":5409145955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::peek_char_or_eof
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28099011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409144912,"raw_hex":"e83d3e53fe","size":5,"soft_fail":false,"target_va":5381045968,"va":5409144974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_identifier
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28077447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409144912,"raw_hex":"e8799253fe","size":5,"soft_fail":false,"target_va":5381069184,"va":5409146626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_next_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28099545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409144912,"raw_hex":"e8273c53fe","size":5,"soft_fail":false,"target_va":5381045680,"va":5409145220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::src
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28098465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409144912,"raw_hex":"e85f4053fe","size":5,"soft_fail":false,"target_va":5381046496,"va":5409144956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::char::methods::impl$0::is_ascii
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":4601,"record_offset":17616,"parent_record_offset":16524,"ancestor_record_offset":16524,"inlinee":9129,"source_line":1172,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4601,"record_offset":18248,"parent_record_offset":16524,"ancestor_record_offset":16524,"inlinee":4667,"source_line":1351,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4601,"record_offset":17412,"parent_record_offset":16524,"ancestor_record_offset":16524,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4601,"record_offset":16668,"parent_record_offset":16524,"ancestor_record_offset":16524,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4601,"record_offset":17704,"parent_record_offset":16524,"ancestor_record_offset":16524,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":16796,"parent_record_offset":16524,"ancestor_record_offset":16524,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::advance_bytes
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":17356,"parent_record_offset":16524,"ancestor_record_offset":16524,"inlinee":9130,"source_line":124,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::set_cursor
//     owner=git_dependency_definition; count=9
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":17384,"parent_record_offset":16524,"ancestor_record_offset":16524,"inlinee":9150,"source_line":113,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":4601,"record_offset":16924,"parent_record_offset":16892,"ancestor_record_offset":16524,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4601,"record_offset":17184,"parent_record_offset":17156,"ancestor_record_offset":16524,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4601,"record_offset":17156,"parent_record_offset":17104,"ancestor_record_offset":16524,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4601,"record_offset":16892,"parent_record_offset":16864,"ancestor_record_offset":16524,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4601,"record_offset":17076,"parent_record_offset":17048,"ancestor_record_offset":16524,"inlinee":8764,"source_line":39,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4601,"record_offset":17048,"parent_record_offset":17020,"ancestor_record_offset":16524,"inlinee":8762,"source_line":185,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::Pattern::is_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":4601,"record_offset":16992,"parent_record_offset":16964,"ancestor_record_offset":16524,"inlinee":9155,"source_line":115,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$10::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":4601,"record_offset":17020,"parent_record_offset":16992,"ancestor_record_offset":16524,"inlinee":8760,"source_line":689,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$17::is_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":4601,"record_offset":16964,"parent_record_offset":16796,"ancestor_record_offset":16524,"inlinee":9153,"source_line":746,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":17856,"parent_record_offset":17820,"ancestor_record_offset":16524,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":17820,"parent_record_offset":17792,"ancestor_record_offset":16524,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":16864,"parent_record_offset":16832,"ancestor_record_offset":16524,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":16832,"parent_record_offset":16796,"ancestor_record_offset":16524,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4601,"record_offset":17104,"parent_record_offset":17076,"ancestor_record_offset":16524,"inlinee":8765,"source_line":36,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4601,"record_offset":17244,"parent_record_offset":17104,"ancestor_record_offset":16524,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4601,"record_offset":17216,"parent_record_offset":17104,"ancestor_record_offset":16524,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::impl$3::integer::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":18280,"parent_record_offset":18248,"ancestor_record_offset":16524,"inlinee":9157,"source_line":427,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::parse::Parser::integer<u64>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4601,"record_offset":14704,"rva":40433808,"va":5409142928}
// 编译实例 RVAs: 0x268f890
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$u64$u20$as$u20$ron..parse..Integer$GT$::parse
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-28061797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409142928,"raw_hex":"e89bcf53fe","size":5,"soft_fail":false,"target_va":5381083024,"va":5409144816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$u64$u20$as$u20$ron..parse..Integer$GT$::try_from_parsed_integer
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-28061354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409142928,"raw_hex":"e856d153fe","size":5,"soft_fail":false,"target_va":5381083056,"va":5409144405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<enum2$<ron::parse::ParsedInteger>,enum2$<ron::error::Error> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409142928,"raw_hex":"e8840f0000","size":5,"soft_fail":false,"target_va":5409148880,"va":5409144903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t2400050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409142928,"raw_hex":"e8329f2400","size":5,"soft_fail":false,"target_va":5411543360,"va":5409143305,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28097506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409142928,"raw_hex":"e81e4453fe","size":5,"soft_fail":false,"target_va":5381046704,"va":5409144205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_ident
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28077670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409142928,"raw_hex":"e89a9153fe","size":5,"soft_fail":false,"target_va":5381065728,"va":5409143393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_str
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28096485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409142928,"raw_hex":"e81b4853fe","size":5,"soft_fail":false,"target_va":5381046576,"va":5409143056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::next_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28097513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409142928,"raw_hex":"e8174453fe","size":5,"soft_fail":false,"target_va":5381045568,"va":5409143076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::next_chars_while_len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28095368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409142928,"raw_hex":"e8784c53fe","size":5,"soft_fail":false,"target_va":5381047888,"va":5409143251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_escape
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28064337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409142928,"raw_hex":"e8afc553fe","size":5,"soft_fail":false,"target_va":5381078784,"va":5409143116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<i16>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-7967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409142928,"raw_hex":"e8e1e0ffff","size":5,"soft_fail":false,"target_va":5409135664,"va":5409143626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<i32>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-4467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409142928,"raw_hex":"e88deeffff","size":5,"soft_fail":false,"target_va":5409139280,"va":5409143742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<i64>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-7300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409142928,"raw_hex":"e87ce3ffff","size":5,"soft_fail":false,"target_va":5409136560,"va":5409143855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<i8>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-5065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409142928,"raw_hex":"e837ecffff","size":5,"soft_fail":false,"target_va":5409138384,"va":5409143444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<u16>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-6710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409142928,"raw_hex":"e8cae5ffff","size":5,"soft_fail":false,"target_va":5409137456,"va":5409144161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<u32>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-2340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409142928,"raw_hex":"e8dcf6ffff","size":5,"soft_fail":false,"target_va":5409142016,"va":5409144351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<u64>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-4311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409142928,"raw_hex":"e829efffff","size":5,"soft_fail":false,"target_va":5409140176,"va":5409144482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<u8>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-2872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409142928,"raw_hex":"e8c8f4ffff","size":5,"soft_fail":false,"target_va":5409141104,"va":5409143971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::peek_char_or_eof
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28097027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409142928,"raw_hex":"e8fd4553fe","size":5,"soft_fail":false,"target_va":5381045968,"va":5409142990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_identifier
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28075463","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409142928,"raw_hex":"e8399a53fe","size":5,"soft_fail":false,"target_va":5381069184,"va":5409144642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_next_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28097561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409142928,"raw_hex":"e8e74353fe","size":5,"soft_fail":false,"target_va":5381045680,"va":5409143236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::src
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28096481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409142928,"raw_hex":"e81f4853fe","size":5,"soft_fail":false,"target_va":5381046496,"va":5409142972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::char::methods::impl$0::is_ascii
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":4601,"record_offset":15796,"parent_record_offset":14704,"ancestor_record_offset":14704,"inlinee":9129,"source_line":1172,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4601,"record_offset":16428,"parent_record_offset":14704,"ancestor_record_offset":14704,"inlinee":4667,"source_line":1351,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4601,"record_offset":15592,"parent_record_offset":14704,"ancestor_record_offset":14704,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4601,"record_offset":14848,"parent_record_offset":14704,"ancestor_record_offset":14704,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4601,"record_offset":15884,"parent_record_offset":14704,"ancestor_record_offset":14704,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":14976,"parent_record_offset":14704,"ancestor_record_offset":14704,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::advance_bytes
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":15536,"parent_record_offset":14704,"ancestor_record_offset":14704,"inlinee":9130,"source_line":124,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::set_cursor
//     owner=git_dependency_definition; count=9
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":15564,"parent_record_offset":14704,"ancestor_record_offset":14704,"inlinee":9150,"source_line":113,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":4601,"record_offset":15104,"parent_record_offset":15072,"ancestor_record_offset":14704,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4601,"record_offset":15364,"parent_record_offset":15336,"ancestor_record_offset":14704,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4601,"record_offset":15336,"parent_record_offset":15284,"ancestor_record_offset":14704,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4601,"record_offset":15072,"parent_record_offset":15044,"ancestor_record_offset":14704,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4601,"record_offset":15256,"parent_record_offset":15228,"ancestor_record_offset":14704,"inlinee":8764,"source_line":39,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4601,"record_offset":15228,"parent_record_offset":15200,"ancestor_record_offset":14704,"inlinee":8762,"source_line":185,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::Pattern::is_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":4601,"record_offset":15172,"parent_record_offset":15144,"ancestor_record_offset":14704,"inlinee":9155,"source_line":115,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$10::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":4601,"record_offset":15200,"parent_record_offset":15172,"ancestor_record_offset":14704,"inlinee":8760,"source_line":689,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$17::is_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":4601,"record_offset":15144,"parent_record_offset":14976,"ancestor_record_offset":14704,"inlinee":9153,"source_line":746,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":16036,"parent_record_offset":16000,"ancestor_record_offset":14704,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":16000,"parent_record_offset":15972,"ancestor_record_offset":14704,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":15044,"parent_record_offset":15012,"ancestor_record_offset":14704,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":15012,"parent_record_offset":14976,"ancestor_record_offset":14704,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4601,"record_offset":15284,"parent_record_offset":15256,"ancestor_record_offset":14704,"inlinee":8765,"source_line":36,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4601,"record_offset":15424,"parent_record_offset":15284,"ancestor_record_offset":14704,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4601,"record_offset":15396,"parent_record_offset":15284,"ancestor_record_offset":14704,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::impl$3::integer::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":16460,"parent_record_offset":16428,"ancestor_record_offset":14704,"inlinee":9157,"source_line":427,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::parse::Parser::integer<u8>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4601,"record_offset":18344,"rva":40437776,"va":5409146896}
// 编译实例 RVAs: 0x2690810
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$u8$u20$as$u20$ron..parse..Integer$GT$::parse
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-28067157","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409146896,"raw_hex":"e8abba53fe","size":5,"soft_fail":false,"target_va":5381081632,"va":5409148784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$u8$u20$as$u20$ron..parse..Integer$GT$::try_from_parsed_integer
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-28066714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409146896,"raw_hex":"e866bc53fe","size":5,"soft_fail":false,"target_va":5381081664,"va":5409148373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<enum2$<ron::parse::ParsedInteger>,enum2$<ron::error::Error> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409146896,"raw_hex":"e804000000","size":5,"soft_fail":false,"target_va":5409148880,"va":5409148871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t2396082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409146896,"raw_hex":"e8b28f2400","size":5,"soft_fail":false,"target_va":5411543360,"va":5409147273,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28101474","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409146896,"raw_hex":"e89e3453fe","size":5,"soft_fail":false,"target_va":5381046704,"va":5409148173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_ident
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28081638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409146896,"raw_hex":"e81a8253fe","size":5,"soft_fail":false,"target_va":5381065728,"va":5409147361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_str
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28100453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409146896,"raw_hex":"e89b3853fe","size":5,"soft_fail":false,"target_va":5381046576,"va":5409147024,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::next_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28101481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409146896,"raw_hex":"e8973453fe","size":5,"soft_fail":false,"target_va":5381045568,"va":5409147044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::next_chars_while_len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28099336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409146896,"raw_hex":"e8f83c53fe","size":5,"soft_fail":false,"target_va":5381047888,"va":5409147219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_escape
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28068305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409146896,"raw_hex":"e82fb653fe","size":5,"soft_fail":false,"target_va":5381078784,"va":5409147084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<i16>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-11935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409146896,"raw_hex":"e861d1ffff","size":5,"soft_fail":false,"target_va":5409135664,"va":5409147594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<i32>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-8435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409146896,"raw_hex":"e80ddfffff","size":5,"soft_fail":false,"target_va":5409139280,"va":5409147710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<i64>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-11268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409146896,"raw_hex":"e8fcd3ffff","size":5,"soft_fail":false,"target_va":5409136560,"va":5409147823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<i8>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-9033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409146896,"raw_hex":"e8b7dcffff","size":5,"soft_fail":false,"target_va":5409138384,"va":5409147412,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<u16>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-10678","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409146896,"raw_hex":"e84ad6ffff","size":5,"soft_fail":false,"target_va":5409137456,"va":5409148129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<u32>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-6308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409146896,"raw_hex":"e85ce7ffff","size":5,"soft_fail":false,"target_va":5409142016,"va":5409148319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<u64>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-8279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409146896,"raw_hex":"e8a9dfffff","size":5,"soft_fail":false,"target_va":5409140176,"va":5409148450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::parse_integer<u8>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-6840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409146896,"raw_hex":"e848e5ffff","size":5,"soft_fail":false,"target_va":5409141104,"va":5409147939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::peek_char_or_eof
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28100995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409146896,"raw_hex":"e87d3653fe","size":5,"soft_fail":false,"target_va":5381045968,"va":5409146958,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_identifier
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28079431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409146896,"raw_hex":"e8b98a53fe","size":5,"soft_fail":false,"target_va":5381069184,"va":5409148610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::skip_next_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28101529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409146896,"raw_hex":"e8673453fe","size":5,"soft_fail":false,"target_va":5381045680,"va":5409147204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::src
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28100449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409146896,"raw_hex":"e89f3853fe","size":5,"soft_fail":false,"target_va":5381046496,"va":5409146940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::char::methods::impl$0::is_ascii
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":4601,"record_offset":19436,"parent_record_offset":18344,"ancestor_record_offset":18344,"inlinee":9129,"source_line":1172,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4601,"record_offset":20068,"parent_record_offset":18344,"ancestor_record_offset":18344,"inlinee":4667,"source_line":1351,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4601,"record_offset":19232,"parent_record_offset":18344,"ancestor_record_offset":18344,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4601,"record_offset":18488,"parent_record_offset":18344,"ancestor_record_offset":18344,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4601,"record_offset":19524,"parent_record_offset":18344,"ancestor_record_offset":18344,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":18616,"parent_record_offset":18344,"ancestor_record_offset":18344,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::advance_bytes
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":19176,"parent_record_offset":18344,"ancestor_record_offset":18344,"inlinee":9130,"source_line":124,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::set_cursor
//     owner=git_dependency_definition; count=9
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":19204,"parent_record_offset":18344,"ancestor_record_offset":18344,"inlinee":9150,"source_line":113,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":4601,"record_offset":18744,"parent_record_offset":18712,"ancestor_record_offset":18344,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4601,"record_offset":19004,"parent_record_offset":18976,"ancestor_record_offset":18344,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4601,"record_offset":18976,"parent_record_offset":18924,"ancestor_record_offset":18344,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4601,"record_offset":18712,"parent_record_offset":18684,"ancestor_record_offset":18344,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4601,"record_offset":18896,"parent_record_offset":18868,"ancestor_record_offset":18344,"inlinee":8764,"source_line":39,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4601,"record_offset":18868,"parent_record_offset":18840,"ancestor_record_offset":18344,"inlinee":8762,"source_line":185,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::Pattern::is_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":4601,"record_offset":18812,"parent_record_offset":18784,"ancestor_record_offset":18344,"inlinee":9155,"source_line":115,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$10::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":4601,"record_offset":18840,"parent_record_offset":18812,"ancestor_record_offset":18344,"inlinee":8760,"source_line":689,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$17::is_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":4601,"record_offset":18784,"parent_record_offset":18616,"ancestor_record_offset":18344,"inlinee":9153,"source_line":746,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":19676,"parent_record_offset":19640,"ancestor_record_offset":18344,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":19640,"parent_record_offset":19612,"ancestor_record_offset":18344,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":18684,"parent_record_offset":18652,"ancestor_record_offset":18344,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":18652,"parent_record_offset":18616,"ancestor_record_offset":18344,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4601,"record_offset":18924,"parent_record_offset":18896,"ancestor_record_offset":18344,"inlinee":8765,"source_line":36,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4601,"record_offset":19064,"parent_record_offset":18924,"ancestor_record_offset":18344,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4601,"record_offset":19036,"parent_record_offset":18924,"ancestor_record_offset":18344,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::impl$3::integer::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":20100,"parent_record_offset":20068,"ancestor_record_offset":18344,"inlinee":9157,"source_line":427,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::parse::Parser::parse_integer<i16>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4601,"record_offset":392,"rva":40426544,"va":5409135664}
// 编译实例 RVAs: 0x268dc30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$i16$u20$as$u20$ron..parse..Num$GT$::checked_add_ext
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-28055445","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409135664,"raw_hex":"e86be853fe","size":5,"soft_fail":false,"target_va":5381080704,"va":5409136144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$i16$u20$as$u20$ron..parse..Num$GT$::checked_mul_ext
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-28055385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409135664,"raw_hex":"e8a7e853fe","size":5,"soft_fail":false,"target_va":5381080688,"va":5409136068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$i16$u20$as$u20$ron..parse..Num$GT$::checked_sub_ext
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-28055685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409135664,"raw_hex":"e87be753fe","size":5,"soft_fail":false,"target_va":5381080720,"va":5409136400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2406807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409135664,"raw_hex":"e897b92400","size":5,"soft_fail":false,"target_va":5411543360,"va":5409136548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::check_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28089703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409135664,"raw_hex":"e8996253fe","size":5,"soft_fail":false,"target_va":5381046112,"va":5409135810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_str
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28089134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409135664,"raw_hex":"e8d26453fe","size":5,"soft_fail":false,"target_va":5381046576,"va":5409135705,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::next_chars_while_len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28087906","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409135664,"raw_hex":"e89e6953fe","size":5,"soft_fail":false,"target_va":5381047888,"va":5409135789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::src
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28089347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409135664,"raw_hex":"e8fd6353fe","size":5,"soft_fail":false,"target_va":5381046496,"va":5409135838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":520,"parent_record_offset":392,"ancestor_record_offset":392,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::parse_integer_digits
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":676,"parent_record_offset":392,"ancestor_record_offset":392,"inlinee":9127,"source_line":224,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::is_ascii
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":4601,"record_offset":1444,"parent_record_offset":1412,"ancestor_record_offset":392,"inlinee":9129,"source_line":1172,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":4601,"record_offset":632,"parent_record_offset":604,"ancestor_record_offset":392,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4601,"record_offset":824,"parent_record_offset":800,"ancestor_record_offset":392,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4601,"record_offset":1332,"parent_record_offset":1308,"ancestor_record_offset":392,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4601,"record_offset":1144,"parent_record_offset":1116,"ancestor_record_offset":392,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4601,"record_offset":896,"parent_record_offset":676,"ancestor_record_offset":392,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4601,"record_offset":776,"parent_record_offset":752,"ancestor_record_offset":392,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":4601,"record_offset":800,"parent_record_offset":776,"ancestor_record_offset":392,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4601,"record_offset":1308,"parent_record_offset":1284,"ancestor_record_offset":392,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4601,"record_offset":1116,"parent_record_offset":1052,"ancestor_record_offset":392,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4601,"record_offset":1284,"parent_record_offset":1260,"ancestor_record_offset":392,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::char_indices
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4601,"record_offset":728,"parent_record_offset":676,"ancestor_record_offset":392,"inlinee":8753,"source_line":1097,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::chars
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4601,"record_offset":752,"parent_record_offset":728,"ancestor_record_offset":392,"inlinee":8754,"source_line":1040,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4601,"record_offset":604,"parent_record_offset":580,"ancestor_record_offset":392,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4601,"record_offset":1024,"parent_record_offset":988,"ancestor_record_offset":392,"inlinee":8764,"source_line":39,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4601,"record_offset":988,"parent_record_offset":928,"ancestor_record_offset":392,"inlinee":8762,"source_line":185,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":580,"parent_record_offset":548,"ancestor_record_offset":392,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":548,"parent_record_offset":520,"ancestor_record_offset":392,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4601,"record_offset":1052,"parent_record_offset":1024,"ancestor_record_offset":392,"inlinee":8765,"source_line":36,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4601,"record_offset":1204,"parent_record_offset":1052,"ancestor_record_offset":392,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4601,"record_offset":1176,"parent_record_offset":1052,"ancestor_record_offset":392,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::Parser::advance_bytes
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":868,"parent_record_offset":676,"ancestor_record_offset":392,"inlinee":9130,"source_line":124,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::Parser::decode_hex
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":1412,"parent_record_offset":928,"ancestor_record_offset":392,"inlinee":9128,"source_line":1165,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::parse::Parser::parse_integer<i32>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4601,"record_offset":7476,"rva":40430160,"va":5409139280}
// 编译实例 RVAs: 0x268ea50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$i32$u20$as$u20$ron..parse..Num$GT$::checked_add_ext
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-28058997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409139280,"raw_hex":"e88bda53fe","size":5,"soft_fail":false,"target_va":5381080768,"va":5409139760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$i32$u20$as$u20$ron..parse..Num$GT$::checked_mul_ext
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-28058937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409139280,"raw_hex":"e8c7da53fe","size":5,"soft_fail":false,"target_va":5381080752,"va":5409139684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$i32$u20$as$u20$ron..parse..Num$GT$::checked_sub_ext
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-28059237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409139280,"raw_hex":"e89bd953fe","size":5,"soft_fail":false,"target_va":5381080784,"va":5409140016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2403193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409139280,"raw_hex":"e879ab2400","size":5,"soft_fail":false,"target_va":5411543360,"va":5409140162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::check_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28093319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409139280,"raw_hex":"e8795453fe","size":5,"soft_fail":false,"target_va":5381046112,"va":5409139426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_str
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28092750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409139280,"raw_hex":"e8b25653fe","size":5,"soft_fail":false,"target_va":5381046576,"va":5409139321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::next_chars_while_len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28091522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409139280,"raw_hex":"e87e5b53fe","size":5,"soft_fail":false,"target_va":5381047888,"va":5409139405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::src
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28092963","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409139280,"raw_hex":"e8dd5553fe","size":5,"soft_fail":false,"target_va":5381046496,"va":5409139454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":7604,"parent_record_offset":7476,"ancestor_record_offset":7476,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::parse_integer_digits
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":7760,"parent_record_offset":7476,"ancestor_record_offset":7476,"inlinee":9127,"source_line":224,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::is_ascii
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":4601,"record_offset":8528,"parent_record_offset":8496,"ancestor_record_offset":7476,"inlinee":9129,"source_line":1172,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":4601,"record_offset":7716,"parent_record_offset":7688,"ancestor_record_offset":7476,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4601,"record_offset":7908,"parent_record_offset":7884,"ancestor_record_offset":7476,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4601,"record_offset":8416,"parent_record_offset":8392,"ancestor_record_offset":7476,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4601,"record_offset":8228,"parent_record_offset":8200,"ancestor_record_offset":7476,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4601,"record_offset":7980,"parent_record_offset":7760,"ancestor_record_offset":7476,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4601,"record_offset":7860,"parent_record_offset":7836,"ancestor_record_offset":7476,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":4601,"record_offset":7884,"parent_record_offset":7860,"ancestor_record_offset":7476,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4601,"record_offset":8392,"parent_record_offset":8368,"ancestor_record_offset":7476,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4601,"record_offset":8200,"parent_record_offset":8136,"ancestor_record_offset":7476,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4601,"record_offset":8368,"parent_record_offset":8344,"ancestor_record_offset":7476,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::char_indices
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4601,"record_offset":7812,"parent_record_offset":7760,"ancestor_record_offset":7476,"inlinee":8753,"source_line":1097,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::chars
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4601,"record_offset":7836,"parent_record_offset":7812,"ancestor_record_offset":7476,"inlinee":8754,"source_line":1040,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4601,"record_offset":7688,"parent_record_offset":7664,"ancestor_record_offset":7476,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4601,"record_offset":8108,"parent_record_offset":8072,"ancestor_record_offset":7476,"inlinee":8764,"source_line":39,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4601,"record_offset":8072,"parent_record_offset":8012,"ancestor_record_offset":7476,"inlinee":8762,"source_line":185,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":7664,"parent_record_offset":7632,"ancestor_record_offset":7476,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":7632,"parent_record_offset":7604,"ancestor_record_offset":7476,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4601,"record_offset":8136,"parent_record_offset":8108,"ancestor_record_offset":7476,"inlinee":8765,"source_line":36,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4601,"record_offset":8288,"parent_record_offset":8136,"ancestor_record_offset":7476,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4601,"record_offset":8260,"parent_record_offset":8136,"ancestor_record_offset":7476,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::Parser::advance_bytes
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":7952,"parent_record_offset":7760,"ancestor_record_offset":7476,"inlinee":9130,"source_line":124,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::Parser::decode_hex
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":8496,"parent_record_offset":8012,"ancestor_record_offset":7476,"inlinee":9128,"source_line":1165,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::parse::Parser::parse_integer<i64>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4601,"record_offset":2160,"rva":40427440,"va":5409136560}
// 编译实例 RVAs: 0x268dfb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$i64$u20$as$u20$ron..parse..Num$GT$::checked_add_ext
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-28056213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409136560,"raw_hex":"e86be553fe","size":5,"soft_fail":false,"target_va":5381080832,"va":5409137040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$i64$u20$as$u20$ron..parse..Num$GT$::checked_mul_ext
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-28056153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409136560,"raw_hex":"e8a7e553fe","size":5,"soft_fail":false,"target_va":5381080816,"va":5409136964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$i64$u20$as$u20$ron..parse..Num$GT$::checked_sub_ext
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-28056453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409136560,"raw_hex":"e87be453fe","size":5,"soft_fail":false,"target_va":5381080848,"va":5409137296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2405911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409136560,"raw_hex":"e817b62400","size":5,"soft_fail":false,"target_va":5411543360,"va":5409137444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::check_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28090599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409136560,"raw_hex":"e8195f53fe","size":5,"soft_fail":false,"target_va":5381046112,"va":5409136706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_str
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28090030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409136560,"raw_hex":"e8526153fe","size":5,"soft_fail":false,"target_va":5381046576,"va":5409136601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::next_chars_while_len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28088802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409136560,"raw_hex":"e81e6653fe","size":5,"soft_fail":false,"target_va":5381047888,"va":5409136685,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::src
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28090243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409136560,"raw_hex":"e87d6053fe","size":5,"soft_fail":false,"target_va":5381046496,"va":5409136734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":2288,"parent_record_offset":2160,"ancestor_record_offset":2160,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::parse_integer_digits
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":2444,"parent_record_offset":2160,"ancestor_record_offset":2160,"inlinee":9127,"source_line":224,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::is_ascii
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":4601,"record_offset":3232,"parent_record_offset":3200,"ancestor_record_offset":2160,"inlinee":9129,"source_line":1172,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":4601,"record_offset":2400,"parent_record_offset":2372,"ancestor_record_offset":2160,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4601,"record_offset":2592,"parent_record_offset":2568,"ancestor_record_offset":2160,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4601,"record_offset":3120,"parent_record_offset":3096,"ancestor_record_offset":2160,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4601,"record_offset":2932,"parent_record_offset":2904,"ancestor_record_offset":2160,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4601,"record_offset":2692,"parent_record_offset":2444,"ancestor_record_offset":2160,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4601,"record_offset":2544,"parent_record_offset":2520,"ancestor_record_offset":2160,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":4601,"record_offset":2568,"parent_record_offset":2544,"ancestor_record_offset":2160,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4601,"record_offset":3096,"parent_record_offset":3072,"ancestor_record_offset":2160,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4601,"record_offset":2904,"parent_record_offset":2840,"ancestor_record_offset":2160,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4601,"record_offset":3072,"parent_record_offset":3048,"ancestor_record_offset":2160,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::char_indices
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4601,"record_offset":2496,"parent_record_offset":2444,"ancestor_record_offset":2160,"inlinee":8753,"source_line":1097,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::chars
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4601,"record_offset":2520,"parent_record_offset":2496,"ancestor_record_offset":2160,"inlinee":8754,"source_line":1040,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4601,"record_offset":2372,"parent_record_offset":2348,"ancestor_record_offset":2160,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4601,"record_offset":2812,"parent_record_offset":2776,"ancestor_record_offset":2160,"inlinee":8764,"source_line":39,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4601,"record_offset":2776,"parent_record_offset":2724,"ancestor_record_offset":2160,"inlinee":8762,"source_line":185,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":2348,"parent_record_offset":2316,"ancestor_record_offset":2160,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":2316,"parent_record_offset":2288,"ancestor_record_offset":2160,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4601,"record_offset":2840,"parent_record_offset":2812,"ancestor_record_offset":2160,"inlinee":8765,"source_line":36,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4601,"record_offset":2992,"parent_record_offset":2840,"ancestor_record_offset":2160,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4601,"record_offset":2964,"parent_record_offset":2840,"ancestor_record_offset":2160,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::Parser::advance_bytes
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":2636,"parent_record_offset":2444,"ancestor_record_offset":2160,"inlinee":9130,"source_line":124,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::Parser::decode_hex
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":3200,"parent_record_offset":2724,"ancestor_record_offset":2160,"inlinee":9128,"source_line":1165,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::parse::Parser::parse_integer<i8>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4601,"record_offset":5708,"rva":40429264,"va":5409138384}
// 编译实例 RVAs: 0x268e6d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$i8$u20$as$u20$ron..parse..Num$GT$::checked_add_ext
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-28058213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409138384,"raw_hex":"e89bdd53fe","size":5,"soft_fail":false,"target_va":5381080656,"va":5409138864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$i8$u20$as$u20$ron..parse..Num$GT$::checked_mul_ext
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-28058153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409138384,"raw_hex":"e8d7dd53fe","size":5,"soft_fail":false,"target_va":5381080640,"va":5409138788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$i8$u20$as$u20$ron..parse..Num$GT$::checked_sub_ext
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-28058453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409138384,"raw_hex":"e8abdc53fe","size":5,"soft_fail":false,"target_va":5381080672,"va":5409139120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2404088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409138384,"raw_hex":"e8f8ae2400","size":5,"soft_fail":false,"target_va":5411543360,"va":5409139267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::check_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28092423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409138384,"raw_hex":"e8f95753fe","size":5,"soft_fail":false,"target_va":5381046112,"va":5409138530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_str
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28091854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409138384,"raw_hex":"e8325a53fe","size":5,"soft_fail":false,"target_va":5381046576,"va":5409138425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::next_chars_while_len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28090626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409138384,"raw_hex":"e8fe5e53fe","size":5,"soft_fail":false,"target_va":5381047888,"va":5409138509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::src
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28092067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409138384,"raw_hex":"e85d5953fe","size":5,"soft_fail":false,"target_va":5381046496,"va":5409138558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":5836,"parent_record_offset":5708,"ancestor_record_offset":5708,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::parse_integer_digits
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":5992,"parent_record_offset":5708,"ancestor_record_offset":5708,"inlinee":9127,"source_line":224,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::is_ascii
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":4601,"record_offset":6760,"parent_record_offset":6728,"ancestor_record_offset":5708,"inlinee":9129,"source_line":1172,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":4601,"record_offset":5948,"parent_record_offset":5920,"ancestor_record_offset":5708,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4601,"record_offset":6140,"parent_record_offset":6116,"ancestor_record_offset":5708,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4601,"record_offset":6648,"parent_record_offset":6624,"ancestor_record_offset":5708,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4601,"record_offset":6460,"parent_record_offset":6432,"ancestor_record_offset":5708,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4601,"record_offset":6212,"parent_record_offset":5992,"ancestor_record_offset":5708,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4601,"record_offset":6092,"parent_record_offset":6068,"ancestor_record_offset":5708,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":4601,"record_offset":6116,"parent_record_offset":6092,"ancestor_record_offset":5708,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4601,"record_offset":6624,"parent_record_offset":6600,"ancestor_record_offset":5708,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4601,"record_offset":6432,"parent_record_offset":6368,"ancestor_record_offset":5708,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4601,"record_offset":6600,"parent_record_offset":6576,"ancestor_record_offset":5708,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::char_indices
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4601,"record_offset":6044,"parent_record_offset":5992,"ancestor_record_offset":5708,"inlinee":8753,"source_line":1097,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::chars
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4601,"record_offset":6068,"parent_record_offset":6044,"ancestor_record_offset":5708,"inlinee":8754,"source_line":1040,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4601,"record_offset":5920,"parent_record_offset":5896,"ancestor_record_offset":5708,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4601,"record_offset":6340,"parent_record_offset":6304,"ancestor_record_offset":5708,"inlinee":8764,"source_line":39,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4601,"record_offset":6304,"parent_record_offset":6244,"ancestor_record_offset":5708,"inlinee":8762,"source_line":185,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":5896,"parent_record_offset":5864,"ancestor_record_offset":5708,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":5864,"parent_record_offset":5836,"ancestor_record_offset":5708,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4601,"record_offset":6368,"parent_record_offset":6340,"ancestor_record_offset":5708,"inlinee":8765,"source_line":36,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4601,"record_offset":6520,"parent_record_offset":6368,"ancestor_record_offset":5708,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4601,"record_offset":6492,"parent_record_offset":6368,"ancestor_record_offset":5708,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::Parser::advance_bytes
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":6184,"parent_record_offset":5992,"ancestor_record_offset":5708,"inlinee":9130,"source_line":124,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::Parser::decode_hex
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":6728,"parent_record_offset":6244,"ancestor_record_offset":5708,"inlinee":9128,"source_line":1165,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::parse::Parser::parse_integer<u16>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4601,"record_offset":3884,"rva":40428336,"va":5409137456}
// 编译实例 RVAs: 0x268e330
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$u16$u20$as$u20$ron..parse..Num$GT$::checked_add_ext
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-28056965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409137456,"raw_hex":"e87be253fe","size":5,"soft_fail":false,"target_va":5381080976,"va":5409137936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$u16$u20$as$u20$ron..parse..Num$GT$::checked_mul_ext
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-28056921","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409137456,"raw_hex":"e8a7e253fe","size":5,"soft_fail":false,"target_va":5381080944,"va":5409137860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2404990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409137456,"raw_hex":"e87eb22400","size":5,"soft_fail":false,"target_va":5411543360,"va":5409138365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::check_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28091495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409137456,"raw_hex":"e8995b53fe","size":5,"soft_fail":false,"target_va":5381046112,"va":5409137602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_str
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28090926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409137456,"raw_hex":"e8d25d53fe","size":5,"soft_fail":false,"target_va":5381046576,"va":5409137497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::next_chars_while_len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28089698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409137456,"raw_hex":"e89e6253fe","size":5,"soft_fail":false,"target_va":5381047888,"va":5409137581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::src
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28091139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409137456,"raw_hex":"e8fd5c53fe","size":5,"soft_fail":false,"target_va":5381046496,"va":5409137630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":4012,"parent_record_offset":3884,"ancestor_record_offset":3884,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::parse_integer_digits
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":4168,"parent_record_offset":3884,"ancestor_record_offset":3884,"inlinee":9127,"source_line":224,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::is_ascii
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":4601,"record_offset":4936,"parent_record_offset":4904,"ancestor_record_offset":3884,"inlinee":9129,"source_line":1172,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":4601,"record_offset":4124,"parent_record_offset":4096,"ancestor_record_offset":3884,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4601,"record_offset":4316,"parent_record_offset":4292,"ancestor_record_offset":3884,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4601,"record_offset":4824,"parent_record_offset":4800,"ancestor_record_offset":3884,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4601,"record_offset":4636,"parent_record_offset":4608,"ancestor_record_offset":3884,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4601,"record_offset":4388,"parent_record_offset":4168,"ancestor_record_offset":3884,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4601,"record_offset":4268,"parent_record_offset":4244,"ancestor_record_offset":3884,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":4601,"record_offset":4292,"parent_record_offset":4268,"ancestor_record_offset":3884,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4601,"record_offset":4800,"parent_record_offset":4776,"ancestor_record_offset":3884,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4601,"record_offset":4608,"parent_record_offset":4544,"ancestor_record_offset":3884,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4601,"record_offset":4776,"parent_record_offset":4752,"ancestor_record_offset":3884,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::char_indices
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4601,"record_offset":4220,"parent_record_offset":4168,"ancestor_record_offset":3884,"inlinee":8753,"source_line":1097,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::chars
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4601,"record_offset":4244,"parent_record_offset":4220,"ancestor_record_offset":3884,"inlinee":8754,"source_line":1040,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4601,"record_offset":4096,"parent_record_offset":4072,"ancestor_record_offset":3884,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4601,"record_offset":4516,"parent_record_offset":4480,"ancestor_record_offset":3884,"inlinee":8764,"source_line":39,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4601,"record_offset":4480,"parent_record_offset":4420,"ancestor_record_offset":3884,"inlinee":8762,"source_line":185,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":4072,"parent_record_offset":4040,"ancestor_record_offset":3884,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":4040,"parent_record_offset":4012,"ancestor_record_offset":3884,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4601,"record_offset":4544,"parent_record_offset":4516,"ancestor_record_offset":3884,"inlinee":8765,"source_line":36,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4601,"record_offset":4696,"parent_record_offset":4544,"ancestor_record_offset":3884,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4601,"record_offset":4668,"parent_record_offset":4544,"ancestor_record_offset":3884,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::Parser::advance_bytes
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":4360,"parent_record_offset":4168,"ancestor_record_offset":3884,"inlinee":9130,"source_line":124,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::Parser::decode_hex
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":4904,"parent_record_offset":4420,"ancestor_record_offset":3884,"inlinee":9128,"source_line":1165,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::impl$15::checked_sub_ext
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":5580,"parent_record_offset":5028,"ancestor_record_offset":3884,"inlinee":9135,"source_line":1365,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::impl$15::from_u8
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":5604,"parent_record_offset":5580,"ancestor_record_offset":3884,"inlinee":9136,"source_line":1341,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::parse::Parser::parse_integer<u32>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4601,"record_offset":12880,"rva":40432896,"va":5409142016}
// 编译实例 RVAs: 0x268f500
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$u32$u20$as$u20$ron..parse..Num$GT$::checked_add_ext
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-28061445","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409142016,"raw_hex":"e8fbd053fe","size":5,"soft_fail":false,"target_va":5381081056,"va":5409142496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$u32$u20$as$u20$ron..parse..Num$GT$::checked_mul_ext
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-28061385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409142016,"raw_hex":"e837d153fe","size":5,"soft_fail":false,"target_va":5381081040,"va":5409142420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2400433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409142016,"raw_hex":"e8b1a02400","size":5,"soft_fail":false,"target_va":5411543360,"va":5409142922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::check_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28096055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409142016,"raw_hex":"e8c94953fe","size":5,"soft_fail":false,"target_va":5381046112,"va":5409142162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_str
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28095486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409142016,"raw_hex":"e8024c53fe","size":5,"soft_fail":false,"target_va":5381046576,"va":5409142057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::next_chars_while_len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28094258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409142016,"raw_hex":"e8ce5053fe","size":5,"soft_fail":false,"target_va":5381047888,"va":5409142141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::src
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28095699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409142016,"raw_hex":"e82d4b53fe","size":5,"soft_fail":false,"target_va":5381046496,"va":5409142190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":13008,"parent_record_offset":12880,"ancestor_record_offset":12880,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::parse_integer_digits
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":13164,"parent_record_offset":12880,"ancestor_record_offset":12880,"inlinee":9127,"source_line":224,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::is_ascii
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":4601,"record_offset":13932,"parent_record_offset":13900,"ancestor_record_offset":12880,"inlinee":9129,"source_line":1172,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":4601,"record_offset":13120,"parent_record_offset":13092,"ancestor_record_offset":12880,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4601,"record_offset":13312,"parent_record_offset":13288,"ancestor_record_offset":12880,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4601,"record_offset":13820,"parent_record_offset":13796,"ancestor_record_offset":12880,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4601,"record_offset":13632,"parent_record_offset":13604,"ancestor_record_offset":12880,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4601,"record_offset":13384,"parent_record_offset":13164,"ancestor_record_offset":12880,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4601,"record_offset":13264,"parent_record_offset":13240,"ancestor_record_offset":12880,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":4601,"record_offset":13288,"parent_record_offset":13264,"ancestor_record_offset":12880,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4601,"record_offset":13796,"parent_record_offset":13772,"ancestor_record_offset":12880,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4601,"record_offset":13604,"parent_record_offset":13540,"ancestor_record_offset":12880,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4601,"record_offset":13772,"parent_record_offset":13748,"ancestor_record_offset":12880,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::char_indices
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4601,"record_offset":13216,"parent_record_offset":13164,"ancestor_record_offset":12880,"inlinee":8753,"source_line":1097,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::chars
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4601,"record_offset":13240,"parent_record_offset":13216,"ancestor_record_offset":12880,"inlinee":8754,"source_line":1040,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4601,"record_offset":13092,"parent_record_offset":13068,"ancestor_record_offset":12880,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4601,"record_offset":13512,"parent_record_offset":13476,"ancestor_record_offset":12880,"inlinee":8764,"source_line":39,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4601,"record_offset":13476,"parent_record_offset":13416,"ancestor_record_offset":12880,"inlinee":8762,"source_line":185,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":13068,"parent_record_offset":13036,"ancestor_record_offset":12880,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":13036,"parent_record_offset":13008,"ancestor_record_offset":12880,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4601,"record_offset":13540,"parent_record_offset":13512,"ancestor_record_offset":12880,"inlinee":8765,"source_line":36,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4601,"record_offset":13692,"parent_record_offset":13540,"ancestor_record_offset":12880,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4601,"record_offset":13664,"parent_record_offset":13540,"ancestor_record_offset":12880,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::Parser::advance_bytes
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":13356,"parent_record_offset":13164,"ancestor_record_offset":12880,"inlinee":9130,"source_line":124,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::Parser::decode_hex
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":13900,"parent_record_offset":13416,"ancestor_record_offset":12880,"inlinee":9128,"source_line":1165,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::impl$16::checked_sub_ext
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":14576,"parent_record_offset":14024,"ancestor_record_offset":12880,"inlinee":9132,"source_line":1365,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::impl$16::from_u8
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":14600,"parent_record_offset":14576,"ancestor_record_offset":12880,"inlinee":9133,"source_line":1341,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::parse::Parser::parse_integer<u64>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4601,"record_offset":9244,"rva":40431056,"va":5409140176}
// 编译实例 RVAs: 0x268edd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$u64$u20$as$u20$ron..parse..Num$GT$::checked_add_ext
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-28059525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409140176,"raw_hex":"e87bd853fe","size":5,"soft_fail":false,"target_va":5381081136,"va":5409140656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$u64$u20$as$u20$ron..parse..Num$GT$::checked_mul_ext
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-28059481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409140176,"raw_hex":"e8a7d853fe","size":5,"soft_fail":false,"target_va":5381081104,"va":5409140580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2402270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409140176,"raw_hex":"e8dea72400","size":5,"soft_fail":false,"target_va":5411543360,"va":5409141085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::check_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28094215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409140176,"raw_hex":"e8f95053fe","size":5,"soft_fail":false,"target_va":5381046112,"va":5409140322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_str
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28093646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409140176,"raw_hex":"e8325353fe","size":5,"soft_fail":false,"target_va":5381046576,"va":5409140217,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::next_chars_while_len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28092418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409140176,"raw_hex":"e8fe5753fe","size":5,"soft_fail":false,"target_va":5381047888,"va":5409140301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::src
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28093859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409140176,"raw_hex":"e85d5253fe","size":5,"soft_fail":false,"target_va":5381046496,"va":5409140350,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":9372,"parent_record_offset":9244,"ancestor_record_offset":9244,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::parse_integer_digits
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":9528,"parent_record_offset":9244,"ancestor_record_offset":9244,"inlinee":9127,"source_line":224,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::is_ascii
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":4601,"record_offset":10296,"parent_record_offset":10264,"ancestor_record_offset":9244,"inlinee":9129,"source_line":1172,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":4601,"record_offset":9484,"parent_record_offset":9456,"ancestor_record_offset":9244,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4601,"record_offset":9676,"parent_record_offset":9652,"ancestor_record_offset":9244,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4601,"record_offset":10184,"parent_record_offset":10160,"ancestor_record_offset":9244,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4601,"record_offset":9996,"parent_record_offset":9968,"ancestor_record_offset":9244,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4601,"record_offset":9748,"parent_record_offset":9528,"ancestor_record_offset":9244,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4601,"record_offset":9628,"parent_record_offset":9604,"ancestor_record_offset":9244,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":4601,"record_offset":9652,"parent_record_offset":9628,"ancestor_record_offset":9244,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4601,"record_offset":10160,"parent_record_offset":10136,"ancestor_record_offset":9244,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4601,"record_offset":9968,"parent_record_offset":9904,"ancestor_record_offset":9244,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4601,"record_offset":10136,"parent_record_offset":10112,"ancestor_record_offset":9244,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::char_indices
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4601,"record_offset":9580,"parent_record_offset":9528,"ancestor_record_offset":9244,"inlinee":8753,"source_line":1097,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::chars
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4601,"record_offset":9604,"parent_record_offset":9580,"ancestor_record_offset":9244,"inlinee":8754,"source_line":1040,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4601,"record_offset":9456,"parent_record_offset":9432,"ancestor_record_offset":9244,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4601,"record_offset":9876,"parent_record_offset":9840,"ancestor_record_offset":9244,"inlinee":8764,"source_line":39,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4601,"record_offset":9840,"parent_record_offset":9780,"ancestor_record_offset":9244,"inlinee":8762,"source_line":185,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":9432,"parent_record_offset":9400,"ancestor_record_offset":9244,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":9400,"parent_record_offset":9372,"ancestor_record_offset":9244,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4601,"record_offset":9904,"parent_record_offset":9876,"ancestor_record_offset":9244,"inlinee":8765,"source_line":36,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4601,"record_offset":10056,"parent_record_offset":9904,"ancestor_record_offset":9244,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4601,"record_offset":10028,"parent_record_offset":9904,"ancestor_record_offset":9244,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::Parser::advance_bytes
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":9720,"parent_record_offset":9528,"ancestor_record_offset":9244,"inlinee":9130,"source_line":124,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::Parser::decode_hex
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":10264,"parent_record_offset":9780,"ancestor_record_offset":9244,"inlinee":9128,"source_line":1165,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::impl$17::checked_sub_ext
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":10936,"parent_record_offset":10388,"ancestor_record_offset":9244,"inlinee":9138,"source_line":1365,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::impl$17::from_u8
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":10960,"parent_record_offset":10936,"ancestor_record_offset":9244,"inlinee":9139,"source_line":1341,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::parse::Parser::parse_integer<u8>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4601,"record_offset":11064,"rva":40431984,"va":5409141104}
// 编译实例 RVAs: 0x268f170
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$u8$u20$as$u20$ron..parse..Num$GT$::checked_add_ext
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-28060693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409141104,"raw_hex":"e8ebd353fe","size":5,"soft_fail":false,"target_va":5381080896,"va":5409141584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$u8$u20$as$u20$ron..parse..Num$GT$::checked_mul_ext
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-28060633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409141104,"raw_hex":"e827d453fe","size":5,"soft_fail":false,"target_va":5381080880,"va":5409141508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2401347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409141104,"raw_hex":"e843a42400","size":5,"soft_fail":false,"target_va":5411543360,"va":5409142008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::check_char
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28095143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409141104,"raw_hex":"e8594d53fe","size":5,"soft_fail":false,"target_va":5381046112,"va":5409141250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::consume_str
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28094574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409141104,"raw_hex":"e8924f53fe","size":5,"soft_fail":false,"target_va":5381046576,"va":5409141145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::next_chars_while_len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28093346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409141104,"raw_hex":"e85e5453fe","size":5,"soft_fail":false,"target_va":5381047888,"va":5409141229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::Parser::src
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-28094787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409141104,"raw_hex":"e8bd4e53fe","size":5,"soft_fail":false,"target_va":5381046496,"va":5409141278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":11192,"parent_record_offset":11064,"ancestor_record_offset":11064,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_direct] ron::parse::Parser::parse_integer_digits
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":11348,"parent_record_offset":11064,"ancestor_record_offset":11064,"inlinee":9127,"source_line":224,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::char::methods::impl$0::is_ascii
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\methods.rs
//     evidence={"module":4601,"record_offset":12116,"parent_record_offset":12084,"ancestor_record_offset":11064,"inlinee":9129,"source_line":1172,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4601,"record_offset":12784,"parent_record_offset":12760,"ancestor_record_offset":11064,"inlinee":9142,"source_line":653,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":4601,"record_offset":11304,"parent_record_offset":11276,"ancestor_record_offset":11064,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4601,"record_offset":11496,"parent_record_offset":11472,"ancestor_record_offset":11064,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4601,"record_offset":12004,"parent_record_offset":11980,"ancestor_record_offset":11064,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4601,"record_offset":11816,"parent_record_offset":11788,"ancestor_record_offset":11064,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4601,"record_offset":11568,"parent_record_offset":11348,"ancestor_record_offset":11064,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4601,"record_offset":11448,"parent_record_offset":11424,"ancestor_record_offset":11064,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":4601,"record_offset":11472,"parent_record_offset":11448,"ancestor_record_offset":11064,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4601,"record_offset":11980,"parent_record_offset":11956,"ancestor_record_offset":11064,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4601,"record_offset":11788,"parent_record_offset":11724,"ancestor_record_offset":11064,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4601,"record_offset":11956,"parent_record_offset":11932,"ancestor_record_offset":11064,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::char_indices
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4601,"record_offset":11400,"parent_record_offset":11348,"ancestor_record_offset":11064,"inlinee":8753,"source_line":1097,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::chars
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4601,"record_offset":11424,"parent_record_offset":11400,"ancestor_record_offset":11064,"inlinee":8754,"source_line":1040,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":4601,"record_offset":11276,"parent_record_offset":11252,"ancestor_record_offset":11064,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4601,"record_offset":11696,"parent_record_offset":11660,"ancestor_record_offset":11064,"inlinee":8764,"source_line":39,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4601,"record_offset":11660,"parent_record_offset":11600,"ancestor_record_offset":11064,"inlinee":8762,"source_line":185,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":11252,"parent_record_offset":11220,"ancestor_record_offset":11064,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4601,"record_offset":11220,"parent_record_offset":11192,"ancestor_record_offset":11064,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4601,"record_offset":11724,"parent_record_offset":11696,"ancestor_record_offset":11064,"inlinee":8765,"source_line":36,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4601,"record_offset":11876,"parent_record_offset":11724,"ancestor_record_offset":11064,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4601,"record_offset":11848,"parent_record_offset":11724,"ancestor_record_offset":11064,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::Parser::advance_bytes
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":11540,"parent_record_offset":11348,"ancestor_record_offset":11064,"inlinee":9130,"source_line":124,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::Parser::decode_hex
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":12084,"parent_record_offset":11600,"ancestor_record_offset":11064,"inlinee":9128,"source_line":1165,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   [pdb_inline_transitive] ron::parse::impl$14::checked_sub_ext
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/parse.rs
//     evidence={"module":4601,"record_offset":12760,"parent_record_offset":12208,"ancestor_record_offset":11064,"inlinee":9141,"source_line":1365,"archive":"_pdb/B/modules/04601.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::ser::Serializer::write_identifier<ref_mut$<alloc::string::String> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4140,"record_offset":8112,"rva":38080208,"va":5406789328}
// 编译实例 RVAs: 0x2450ed0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142451200
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406789328,"raw_hex":"e807030000","size":5,"soft_fail":false,"target_va":5406790144,"va":5406789364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5057126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406789328,"raw_hex":"e8662a4d00","size":5,"soft_fail":false,"target_va":5411847168,"va":5406790037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5001415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406789328,"raw_hex":"e8c7504c00","size":5,"soft_fail":false,"target_va":5411791504,"va":5406790084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ron::parse::is_ident_first_char
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     evidence={"assembly":"\tcallq\t-25745502","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406789328,"raw_hex":"e8a22777fe","size":5,"soft_fail":false,"target_va":5381043904,"va":5406789401,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] unicode_ident::is_xid_continue
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"unicode-ident","version":"1.0.18","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-25696024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406789328,"raw_hex":"e8e8e877fe","size":5,"soft_fail":false,"target_va":5381093600,"va":5406789619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::impl$1::write_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4140,"record_offset":9516,"parent_record_offset":8112,"ancestor_record_offset":8112,"inlinee":6451,"source_line":253,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":4140,"record_offset":8660,"parent_record_offset":8112,"ancestor_record_offset":8112,"inlinee":12922,"source_line":2705,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4140,"record_offset":8576,"parent_record_offset":8112,"ancestor_record_offset":8112,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4140,"record_offset":8288,"parent_record_offset":8112,"ancestor_record_offset":8112,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4140,"record_offset":9044,"parent_record_offset":8112,"ancestor_record_offset":8112,"inlinee":12923,"source_line":2551,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4140,"record_offset":8320,"parent_record_offset":8112,"ancestor_record_offset":8112,"inlinee":8764,"source_line":39,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4140,"record_offset":10368,"parent_record_offset":10336,"ancestor_record_offset":8112,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4140,"record_offset":9752,"parent_record_offset":9720,"ancestor_record_offset":8112,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4140,"record_offset":9836,"parent_record_offset":9812,"ancestor_record_offset":8112,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4140,"record_offset":9812,"parent_record_offset":9780,"ancestor_record_offset":8112,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4140,"record_offset":10416,"parent_record_offset":10392,"ancestor_record_offset":8112,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4140,"record_offset":10392,"parent_record_offset":10368,"ancestor_record_offset":8112,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4140,"record_offset":9780,"parent_record_offset":9752,"ancestor_record_offset":8112,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::push_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4140,"record_offset":9580,"parent_record_offset":9548,"ancestor_record_offset":8112,"inlinee":6418,"source_line":1126,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$60::write_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4140,"record_offset":9548,"parent_record_offset":9516,"ancestor_record_offset":8112,"inlinee":6417,"source_line":3162,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":9676,"parent_record_offset":9644,"ancestor_record_offset":8112,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":10336,"parent_record_offset":10116,"ancestor_record_offset":8112,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":9612,"parent_record_offset":9580,"ancestor_record_offset":8112,"inlinee":5425,"source_line":3033,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":9924,"parent_record_offset":9676,"ancestor_record_offset":8112,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":9720,"parent_record_offset":9676,"ancestor_record_offset":8112,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":4140,"record_offset":9644,"parent_record_offset":9612,"ancestor_record_offset":8112,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4140,"record_offset":9176,"parent_record_offset":9116,"ancestor_record_offset":8112,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::all::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":4140,"record_offset":8712,"parent_record_offset":8684,"ancestor_record_offset":8112,"inlinee":11064,"source_line":2712,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":4140,"record_offset":8684,"parent_record_offset":8660,"ancestor_record_offset":8112,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":4140,"record_offset":8736,"parent_record_offset":8712,"ancestor_record_offset":8112,"inlinee":4196,"source_line":166,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":4140,"record_offset":8616,"parent_record_offset":8576,"ancestor_record_offset":8112,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4140,"record_offset":9888,"parent_record_offset":9676,"ancestor_record_offset":8112,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4140,"record_offset":8424,"parent_record_offset":8396,"ancestor_record_offset":8112,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4140,"record_offset":9356,"parent_record_offset":9296,"ancestor_record_offset":8112,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$15::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4140,"record_offset":9068,"parent_record_offset":9044,"ancestor_record_offset":8112,"inlinee":12925,"source_line":236,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$15::slice_contains::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4140,"record_offset":9116,"parent_record_offset":9092,"ancestor_record_offset":8112,"inlinee":12927,"source_line":237,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4140,"record_offset":9416,"parent_record_offset":9356,"ancestor_record_offset":8112,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4140,"record_offset":9092,"parent_record_offset":9068,"ancestor_record_offset":8112,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4140,"record_offset":8396,"parent_record_offset":8348,"ancestor_record_offset":8112,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":4140,"record_offset":8768,"parent_record_offset":8684,"ancestor_record_offset":8112,"inlinee":8764,"source_line":39,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":4140,"record_offset":9236,"parent_record_offset":9176,"ancestor_record_offset":8112,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4140,"record_offset":8348,"parent_record_offset":8320,"ancestor_record_offset":8112,"inlinee":8765,"source_line":36,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4140,"record_offset":8484,"parent_record_offset":8348,"ancestor_record_offset":8112,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":4140,"record_offset":8456,"parent_record_offset":8348,"ancestor_record_offset":8112,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,enum2$<core::option::Option<array$<u32,2> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4140,"record_offset":63408,"rva":38094784,"va":5406803904}
// 编译实例 RVAs: 0x24547c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$::write_char
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-3609255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406803904,"raw_hex":"e859edc8ff","size":5,"soft_fail":false,"target_va":5403195712,"va":5406804962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$::write_str
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-3608937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406803904,"raw_hex":"e897eec8ff","size":5,"soft_fail":false,"target_va":5403196096,"va":5406805028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142450ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406803904,"raw_hex":"e887c5ffff","size":5,"soft_fail":false,"target_va":5406789328,"va":5406804292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5043036","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406803904,"raw_hex":"e85cf34c00","size":5,"soft_fail":false,"target_va":5411847168,"va":5406804127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-23966374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406803904,"raw_hex":"e85a4d92fe","size":5,"soft_fail":false,"target_va":5382837616,"va":5406803985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t4987231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406803904,"raw_hex":"e85f194c00","size":5,"soft_fail":false,"target_va":5411791504,"va":5406804268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::write
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-23917288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406803904,"raw_hex":"e8180d93fe","size":5,"soft_fail":false,"target_va":5382888144,"va":5406805427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$30::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4140,"record_offset":64024,"parent_record_offset":63408,"ancestor_record_offset":63408,"inlinee":5600,"source_line":2689,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::impl$1::write_char
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4140,"record_offset":63644,"parent_record_offset":63408,"ancestor_record_offset":63408,"inlinee":6450,"source_line":257,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::impl$1::write_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4140,"record_offset":64416,"parent_record_offset":63408,"ancestor_record_offset":63408,"inlinee":6451,"source_line":253,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4140,"record_offset":66600,"parent_record_offset":63408,"ancestor_record_offset":63408,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4140,"record_offset":73876,"parent_record_offset":63408,"ancestor_record_offset":63408,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4140,"record_offset":65656,"parent_record_offset":63408,"ancestor_record_offset":63408,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4140,"record_offset":73820,"parent_record_offset":63408,"ancestor_record_offset":63408,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] ron::ser::Serializer::compact_structs
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":63908,"parent_record_offset":63408,"ancestor_record_offset":63408,"inlinee":12931,"source_line":454,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] ron::ser::Serializer::indent
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":64864,"parent_record_offset":63408,"ancestor_record_offset":63408,"inlinee":12932,"source_line":500,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] serde::ser::impls::impl$5::serialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/ser/impls.rs
//     evidence={"module":4140,"record_offset":66632,"parent_record_offset":63408,"ancestor_record_offset":63408,"inlinee":8898,"source_line":104,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4140,"record_offset":63776,"parent_record_offset":63752,"ancestor_record_offset":63408,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4140,"record_offset":64624,"parent_record_offset":64596,"ancestor_record_offset":63408,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4140,"record_offset":64720,"parent_record_offset":64692,"ancestor_record_offset":63408,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4140,"record_offset":64692,"parent_record_offset":64656,"ancestor_record_offset":63408,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4140,"record_offset":63824,"parent_record_offset":63800,"ancestor_record_offset":63408,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4140,"record_offset":63800,"parent_record_offset":63776,"ancestor_record_offset":63408,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4140,"record_offset":64656,"parent_record_offset":64624,"ancestor_record_offset":63408,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4140,"record_offset":64048,"parent_record_offset":64024,"ancestor_record_offset":63408,"inlinee":5602,"source_line":1081,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4140,"record_offset":63692,"parent_record_offset":63668,"ancestor_record_offset":63408,"inlinee":6414,"source_line":1419,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::push_str
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4140,"record_offset":64472,"parent_record_offset":64444,"ancestor_record_offset":63408,"inlinee":6418,"source_line":1126,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$30::deref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4140,"record_offset":65456,"parent_record_offset":64864,"ancestor_record_offset":63408,"inlinee":5600,"source_line":2689,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$60::write_char
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4140,"record_offset":63668,"parent_record_offset":63644,"ancestor_record_offset":63408,"inlinee":6413,"source_line":3168,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$60::write_str
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4140,"record_offset":64444,"parent_record_offset":64416,"ancestor_record_offset":63408,"inlinee":6417,"source_line":3162,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":64556,"parent_record_offset":64528,"ancestor_record_offset":63408,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":63752,"parent_record_offset":63716,"ancestor_record_offset":63408,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":64096,"parent_record_offset":64072,"ancestor_record_offset":63408,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":64072,"parent_record_offset":64048,"ancestor_record_offset":63408,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":64500,"parent_record_offset":64472,"ancestor_record_offset":63408,"inlinee":5425,"source_line":3033,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":64812,"parent_record_offset":64556,"ancestor_record_offset":63408,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":63716,"parent_record_offset":63692,"ancestor_record_offset":63408,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":64596,"parent_record_offset":64556,"ancestor_record_offset":63408,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":4140,"record_offset":64528,"parent_record_offset":64500,"ancestor_record_offset":63408,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4140,"record_offset":64956,"parent_record_offset":64928,"ancestor_record_offset":63408,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Write::write_fmt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4140,"record_offset":70512,"parent_record_offset":70488,"ancestor_record_offset":63408,"inlinee":5573,"source_line":218,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Write::write_fmt::impl$1::spec_write_fmt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4140,"record_offset":70536,"parent_record_offset":70512,"ancestor_record_offset":63408,"inlinee":5575,"source_line":238,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::impl$1::write_char
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4140,"record_offset":68320,"parent_record_offset":68224,"ancestor_record_offset":63408,"inlinee":6450,"source_line":257,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::impl$1::write_fmt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4140,"record_offset":70488,"parent_record_offset":70456,"ancestor_record_offset":63408,"inlinee":5571,"source_line":261,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::impl$1::write_str
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4140,"record_offset":64996,"parent_record_offset":64864,"ancestor_record_offset":63408,"inlinee":6451,"source_line":253,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":4140,"record_offset":69664,"parent_record_offset":69600,"ancestor_record_offset":63408,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":4140,"record_offset":64928,"parent_record_offset":64900,"ancestor_record_offset":63408,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":4140,"record_offset":64900,"parent_record_offset":64864,"ancestor_record_offset":63408,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4140,"record_offset":67612,"parent_record_offset":66696,"ancestor_record_offset":63408,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4140,"record_offset":70812,"parent_record_offset":68224,"ancestor_record_offset":63408,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4140,"record_offset":69688,"parent_record_offset":69664,"ancestor_record_offset":63408,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4140,"record_offset":63948,"parent_record_offset":63908,"ancestor_record_offset":63408,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4140,"record_offset":63984,"parent_record_offset":63908,"ancestor_record_offset":63408,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4140,"record_offset":71416,"parent_record_offset":71368,"ancestor_record_offset":63408,"inlinee":4666,"source_line":998,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4140,"record_offset":70844,"parent_record_offset":67656,"ancestor_record_offset":63408,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4140,"record_offset":64776,"parent_record_offset":64556,"ancestor_record_offset":63408,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":4140,"record_offset":68188,"parent_record_offset":68152,"ancestor_record_offset":63408,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4140,"record_offset":63864,"parent_record_offset":63716,"ancestor_record_offset":63408,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4140,"record_offset":67776,"parent_record_offset":67732,"ancestor_record_offset":63408,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":4140,"record_offset":68152,"parent_record_offset":67656,"ancestor_record_offset":63408,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::extensions::Extensions::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.9.0/src/lib.rs
//     evidence={"module":4140,"record_offset":67004,"parent_record_offset":66696,"ancestor_record_offset":63408,"inlinee":12958,"source_line":703,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::extensions::Extensions::union
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.9.0/src/lib.rs
//     evidence={"module":4140,"record_offset":66932,"parent_record_offset":66904,"ancestor_record_offset":63408,"inlinee":12956,"source_line":757,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::extensions::_::InternalBitFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.9.0/src/lib.rs
//     evidence={"module":4140,"record_offset":67036,"parent_record_offset":67004,"ancestor_record_offset":63408,"inlinee":12959,"source_line":703,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::extensions::_::InternalBitFlags::union
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.9.0/src/lib.rs
//     evidence={"module":4140,"record_offset":66960,"parent_record_offset":66932,"ancestor_record_offset":63408,"inlinee":12957,"source_line":757,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::extensions::_::impl$41::bitor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.9.0/src/public.rs
//     evidence={"module":4140,"record_offset":66904,"parent_record_offset":66820,"ancestor_record_offset":63408,"inlinee":12955,"source_line":382,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::Serializer::end_indent
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":71368,"parent_record_offset":70940,"ancestor_record_offset":63408,"inlinee":12929,"source_line":511,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::Serializer::extensions
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":66820,"parent_record_offset":66696,"ancestor_record_offset":63408,"inlinee":12953,"source_line":472,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::Serializer::indent
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":69532,"parent_record_offset":68224,"ancestor_record_offset":63408,"inlinee":12932,"source_line":500,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::Serializer::number_suffixes
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":70600,"parent_record_offset":70456,"ancestor_record_offset":63408,"inlinee":5576,"source_line":466,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::Serializer::separate_tuple_members
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":68060,"parent_record_offset":67732,"ancestor_record_offset":63408,"inlinee":12930,"source_line":442,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::Serializer::serialize_uint
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":70456,"parent_record_offset":70432,"ancestor_record_offset":63408,"inlinee":5569,"source_line":603,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::Serializer::start_indent
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":67804,"parent_record_offset":67732,"ancestor_record_offset":63408,"inlinee":12937,"source_line":486,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::impl$3::serialize_none
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":72220,"parent_record_offset":66632,"ancestor_record_offset":63408,"inlinee":12951,"source_line":795,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::impl$3::serialize_some
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":66696,"parent_record_offset":66632,"ancestor_record_offset":63408,"inlinee":12952,"source_line":811,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::impl$3::serialize_tuple
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":67732,"parent_record_offset":67656,"ancestor_record_offset":63408,"inlinee":12940,"source_line":954,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::impl$3::serialize_u32
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":70432,"parent_record_offset":70408,"ancestor_record_offset":63408,"inlinee":5567,"source_line":711,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::impl$5::drop
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":70872,"parent_record_offset":70844,"ancestor_record_offset":63408,"inlinee":5557,"source_line":1098,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::impl$7::end
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":70940,"parent_record_offset":67656,"ancestor_record_offset":63408,"inlinee":12985,"source_line":1192,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::impl$7::serialize_element
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":68224,"parent_record_offset":67656,"ancestor_record_offset":63408,"inlinee":12942,"source_line":1166,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] serde::ser::impls::impl$47::serialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/ser/impls.rs
//     evidence={"module":4140,"record_offset":70408,"parent_record_offset":68224,"ancestor_record_offset":63408,"inlinee":5566,"source_line":11,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] serde::ser::impls::impl$54::serialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/ser/impls.rs
//     evidence={"module":4140,"record_offset":67656,"parent_record_offset":66696,"ancestor_record_offset":63408,"inlinee":12939,"source_line":151,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,std::collections::hash::map::HashMap<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta,std::hash::random::RandomState> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4140,"record_offset":25900,"rva":38084832,"va":5406793952}
// 编译实例 RVAs: 0x24520e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$::write_char
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-3600209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406793952,"raw_hex":"e8af10c9ff","size":5,"soft_fail":false,"target_va":5403195712,"va":5406795916,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$::write_str
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-3598950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406793952,"raw_hex":"e89a15c9ff","size":5,"soft_fail":false,"target_va":5403196096,"va":5406795041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420e3560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3600745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406793952,"raw_hex":"e8970ec9ff","size":5,"soft_fail":false,"target_va":5403194720,"va":5406795460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142450ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406793952,"raw_hex":"e84becffff","size":5,"soft_fail":false,"target_va":5406789328,"va":5406794368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1424557e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406793952,"raw_hex":"e850320000","size":5,"soft_fail":false,"target_va":5406808032,"va":5406795147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5052939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406793952,"raw_hex":"e80b1a4d00","size":5,"soft_fail":false,"target_va":5411847168,"va":5406794224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t-23956436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406793952,"raw_hex":"e82c7492fe","size":5,"soft_fail":false,"target_va":5382837616,"va":5406794047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t4997224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406793952,"raw_hex":"e868404c00","size":5,"soft_fail":false,"target_va":5411791504,"va":5406794275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$30::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4140,"record_offset":26488,"parent_record_offset":25900,"ancestor_record_offset":25900,"inlinee":5600,"source_line":2689,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::impl$1::write_char
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4140,"record_offset":26228,"parent_record_offset":25900,"ancestor_record_offset":25900,"inlinee":6450,"source_line":257,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::impl$1::write_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4140,"record_offset":27776,"parent_record_offset":25900,"ancestor_record_offset":25900,"inlinee":6451,"source_line":253,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4140,"record_offset":29168,"parent_record_offset":25900,"ancestor_record_offset":25900,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4140,"record_offset":35856,"parent_record_offset":25900,"ancestor_record_offset":25900,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4140,"record_offset":28224,"parent_record_offset":25900,"ancestor_record_offset":25900,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4140,"record_offset":35764,"parent_record_offset":25900,"ancestor_record_offset":25900,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] ron::ser::Serializer::compact_structs
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":26684,"parent_record_offset":25900,"ancestor_record_offset":25900,"inlinee":12931,"source_line":454,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] ron::ser::Serializer::indent
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":26788,"parent_record_offset":25900,"ancestor_record_offset":25900,"inlinee":12932,"source_line":500,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] serde::ser::impls::impl$108::serialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/ser/impls.rs
//     evidence={"module":4140,"record_offset":29200,"parent_record_offset":25900,"ancestor_record_offset":25900,"inlinee":14436,"source_line":443,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4140,"record_offset":26356,"parent_record_offset":26332,"ancestor_record_offset":25900,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4140,"record_offset":27156,"parent_record_offset":27124,"ancestor_record_offset":25900,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4140,"record_offset":27240,"parent_record_offset":27216,"ancestor_record_offset":25900,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4140,"record_offset":27216,"parent_record_offset":27184,"ancestor_record_offset":25900,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4140,"record_offset":26404,"parent_record_offset":26380,"ancestor_record_offset":25900,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4140,"record_offset":26380,"parent_record_offset":26356,"ancestor_record_offset":25900,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4140,"record_offset":27184,"parent_record_offset":27156,"ancestor_record_offset":25900,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4140,"record_offset":26512,"parent_record_offset":26488,"ancestor_record_offset":25900,"inlinee":5602,"source_line":1081,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::push
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4140,"record_offset":26276,"parent_record_offset":26252,"ancestor_record_offset":25900,"inlinee":6414,"source_line":1419,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::push_str
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4140,"record_offset":26984,"parent_record_offset":26952,"ancestor_record_offset":25900,"inlinee":6418,"source_line":1126,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$30::deref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4140,"record_offset":27380,"parent_record_offset":26788,"ancestor_record_offset":25900,"inlinee":5600,"source_line":2689,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$60::write_char
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4140,"record_offset":26252,"parent_record_offset":26228,"ancestor_record_offset":25900,"inlinee":6413,"source_line":3168,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$60::write_str
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4140,"record_offset":26952,"parent_record_offset":26920,"ancestor_record_offset":25900,"inlinee":6417,"source_line":3162,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":27080,"parent_record_offset":27048,"ancestor_record_offset":25900,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":26332,"parent_record_offset":26300,"ancestor_record_offset":25900,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":26560,"parent_record_offset":26536,"ancestor_record_offset":25900,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":26536,"parent_record_offset":26512,"ancestor_record_offset":25900,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":27016,"parent_record_offset":26984,"ancestor_record_offset":25900,"inlinee":5425,"source_line":3033,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":27328,"parent_record_offset":27080,"ancestor_record_offset":25900,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":26300,"parent_record_offset":26276,"ancestor_record_offset":25900,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":27124,"parent_record_offset":27080,"ancestor_record_offset":25900,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":4140,"record_offset":27048,"parent_record_offset":27016,"ancestor_record_offset":25900,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4140,"record_offset":26880,"parent_record_offset":26852,"ancestor_record_offset":25900,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":4140,"record_offset":29532,"parent_record_offset":29508,"ancestor_record_offset":25900,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":4140,"record_offset":30816,"parent_record_offset":30780,"ancestor_record_offset":25900,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::impl$1::write_char
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4140,"record_offset":29640,"parent_record_offset":29588,"ancestor_record_offset":25900,"inlinee":6450,"source_line":257,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::impl$1::write_str
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4140,"record_offset":26920,"parent_record_offset":26788,"ancestor_record_offset":25900,"inlinee":6451,"source_line":253,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":4140,"record_offset":32580,"parent_record_offset":32516,"ancestor_record_offset":25900,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":4140,"record_offset":26852,"parent_record_offset":26824,"ancestor_record_offset":25900,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":4140,"record_offset":26824,"parent_record_offset":26788,"ancestor_record_offset":25900,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":4140,"record_offset":30284,"parent_record_offset":30244,"ancestor_record_offset":25900,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":4140,"record_offset":30244,"parent_record_offset":29248,"ancestor_record_offset":25900,"inlinee":10981,"source_line":2420,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":4140,"record_offset":31008,"parent_record_offset":30284,"ancestor_record_offset":25900,"inlinee":11044,"source_line":2428,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4140,"record_offset":33292,"parent_record_offset":31144,"ancestor_record_offset":25900,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4140,"record_offset":33380,"parent_record_offset":31144,"ancestor_record_offset":25900,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4140,"record_offset":32604,"parent_record_offset":32580,"ancestor_record_offset":25900,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":4140,"record_offset":30616,"parent_record_offset":30584,"ancestor_record_offset":25900,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":4140,"record_offset":30584,"parent_record_offset":30552,"ancestor_record_offset":25900,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4140,"record_offset":26720,"parent_record_offset":26684,"ancestor_record_offset":25900,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4140,"record_offset":26752,"parent_record_offset":26684,"ancestor_record_offset":25900,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4140,"record_offset":34992,"parent_record_offset":34944,"ancestor_record_offset":25900,"inlinee":4666,"source_line":998,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":4140,"record_offset":30908,"parent_record_offset":30420,"ancestor_record_offset":25900,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4140,"record_offset":35384,"parent_record_offset":34548,"ancestor_record_offset":25900,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4140,"record_offset":27292,"parent_record_offset":27080,"ancestor_record_offset":25900,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4140,"record_offset":30876,"parent_record_offset":30852,"ancestor_record_offset":25900,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4140,"record_offset":26444,"parent_record_offset":26300,"ancestor_record_offset":25900,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4140,"record_offset":34460,"parent_record_offset":33412,"ancestor_record_offset":25900,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":4140,"record_offset":30512,"parent_record_offset":30472,"ancestor_record_offset":25900,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":4140,"record_offset":30552,"parent_record_offset":30512,"ancestor_record_offset":25900,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":4140,"record_offset":30660,"parent_record_offset":30472,"ancestor_record_offset":25900,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":4140,"record_offset":30472,"parent_record_offset":30420,"ancestor_record_offset":25900,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":4140,"record_offset":29508,"parent_record_offset":29484,"ancestor_record_offset":25900,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":4140,"record_offset":30780,"parent_record_offset":30752,"ancestor_record_offset":25900,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":4140,"record_offset":30752,"parent_record_offset":30420,"ancestor_record_offset":25900,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":4140,"record_offset":29376,"parent_record_offset":29352,"ancestor_record_offset":25900,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":4140,"record_offset":30356,"parent_record_offset":30328,"ancestor_record_offset":25900,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":4140,"record_offset":30852,"parent_record_offset":30420,"ancestor_record_offset":25900,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":4140,"record_offset":29484,"parent_record_offset":29424,"ancestor_record_offset":25900,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":4140,"record_offset":30420,"parent_record_offset":30384,"ancestor_record_offset":25900,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":4140,"record_offset":29400,"parent_record_offset":29376,"ancestor_record_offset":25900,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":4140,"record_offset":29456,"parent_record_offset":29424,"ancestor_record_offset":25900,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":4140,"record_offset":29424,"parent_record_offset":29400,"ancestor_record_offset":25900,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":4140,"record_offset":30384,"parent_record_offset":30356,"ancestor_record_offset":25900,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] relative_path::impl$55::serialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"relative-path","version":"1.9.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-1.9.0/src/lib.rs
//     evidence={"module":4140,"record_offset":33348,"parent_record_offset":33324,"ancestor_record_offset":25900,"inlinee":125493,"source_line":2060,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::Serializer::compact_maps
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":29904,"parent_record_offset":29588,"ancestor_record_offset":25900,"inlinee":125495,"source_line":460,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::Serializer::end_indent
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":34944,"parent_record_offset":34548,"ancestor_record_offset":25900,"inlinee":12929,"source_line":511,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::Serializer::indent
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":32444,"parent_record_offset":31144,"ancestor_record_offset":25900,"inlinee":12932,"source_line":500,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::Serializer::start_indent
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":29992,"parent_record_offset":29588,"ancestor_record_offset":25900,"inlinee":12937,"source_line":486,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::impl$10::end
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":34548,"parent_record_offset":29248,"ancestor_record_offset":25900,"inlinee":125499,"source_line":1290,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::impl$10::serialize_key
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":31144,"parent_record_offset":31072,"ancestor_record_offset":25900,"inlinee":125497,"source_line":1250,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::impl$10::serialize_value
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":33412,"parent_record_offset":31072,"ancestor_record_offset":25900,"inlinee":125498,"source_line":1275,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::impl$3::serialize_map
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":29588,"parent_record_offset":29248,"ancestor_record_offset":25900,"inlinee":125494,"source_line":1009,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::impl$5::drop
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":35408,"parent_record_offset":35384,"ancestor_record_offset":25900,"inlinee":5557,"source_line":1098,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] serde::ser::SerializeMap::serialize_entry
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":31072,"parent_record_offset":31040,"ancestor_record_offset":25900,"inlinee":6925,"source_line":1804,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] serde::ser::Serializer::collect_map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":29248,"parent_record_offset":29200,"ancestor_record_offset":25900,"inlinee":11056,"source_line":1311,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] serde::ser::Serializer::collect_map::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":31040,"parent_record_offset":31008,"ancestor_record_offset":25900,"inlinee":11046,"source_line":1319,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] serde::ser::impls::impl$109::serialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/ser/impls.rs
//     evidence={"module":4140,"record_offset":33324,"parent_record_offset":31144,"ancestor_record_offset":25900,"inlinee":8878,"source_line":499,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":4140,"record_offset":29352,"parent_record_offset":29328,"ancestor_record_offset":25900,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$44::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":4140,"record_offset":29328,"parent_record_offset":29248,"ancestor_record_offset":25900,"inlinee":5192,"source_line":2340,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":4140,"record_offset":30328,"parent_record_offset":30284,"ancestor_record_offset":25900,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,u32>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4140,"record_offset":56312,"rva":38092864,"va":5406801984}
// 编译实例 RVAs: 0x2454040
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$::write_fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-3606760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406801984,"raw_hex":"e818f7c8ff","size":5,"soft_fail":false,"target_va":5403196032,"va":5406802787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142450ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406801984,"raw_hex":"e827cdffff","size":5,"soft_fail":false,"target_va":5406789328,"va":5406802340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5044955","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406801984,"raw_hex":"e8dbfa4c00","size":5,"soft_fail":false,"target_va":5411847168,"va":5406802208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-23964434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406801984,"raw_hex":"e8ee5492fe","size":5,"soft_fail":false,"target_va":5382837616,"va":5406802045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4989247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406801984,"raw_hex":"e83f214c00","size":5,"soft_fail":false,"target_va":5411791504,"va":5406802252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::write
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-23914481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406801984,"raw_hex":"e80f1893fe","size":5,"soft_fail":false,"target_va":5382888144,"va":5406802620,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$30::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4140,"record_offset":56760,"parent_record_offset":56312,"ancestor_record_offset":56312,"inlinee":5600,"source_line":2689,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::impl$1::write_char
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4140,"record_offset":56500,"parent_record_offset":56312,"ancestor_record_offset":56312,"inlinee":6450,"source_line":257,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::impl$1::write_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4140,"record_offset":58048,"parent_record_offset":56312,"ancestor_record_offset":56312,"inlinee":6451,"source_line":253,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4140,"record_offset":59440,"parent_record_offset":56312,"ancestor_record_offset":56312,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4140,"record_offset":59828,"parent_record_offset":56312,"ancestor_record_offset":56312,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4140,"record_offset":58496,"parent_record_offset":56312,"ancestor_record_offset":56312,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] ron::ser::Serializer::compact_structs
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":56956,"parent_record_offset":56312,"ancestor_record_offset":56312,"inlinee":12931,"source_line":454,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] ron::ser::Serializer::indent
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":57060,"parent_record_offset":56312,"ancestor_record_offset":56312,"inlinee":12932,"source_line":500,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] serde::ser::impls::impl$47::serialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/ser/impls.rs
//     evidence={"module":4140,"record_offset":59472,"parent_record_offset":56312,"ancestor_record_offset":56312,"inlinee":5566,"source_line":11,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4140,"record_offset":56628,"parent_record_offset":56604,"ancestor_record_offset":56312,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4140,"record_offset":57428,"parent_record_offset":57396,"ancestor_record_offset":56312,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4140,"record_offset":57512,"parent_record_offset":57488,"ancestor_record_offset":56312,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4140,"record_offset":57488,"parent_record_offset":57456,"ancestor_record_offset":56312,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4140,"record_offset":56676,"parent_record_offset":56652,"ancestor_record_offset":56312,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4140,"record_offset":56652,"parent_record_offset":56628,"ancestor_record_offset":56312,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4140,"record_offset":57456,"parent_record_offset":57428,"ancestor_record_offset":56312,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4140,"record_offset":56784,"parent_record_offset":56760,"ancestor_record_offset":56312,"inlinee":5602,"source_line":1081,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4140,"record_offset":56548,"parent_record_offset":56524,"ancestor_record_offset":56312,"inlinee":6414,"source_line":1419,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::push_str
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4140,"record_offset":57256,"parent_record_offset":57224,"ancestor_record_offset":56312,"inlinee":6418,"source_line":1126,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$30::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4140,"record_offset":57652,"parent_record_offset":57060,"ancestor_record_offset":56312,"inlinee":5600,"source_line":2689,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$60::write_char
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4140,"record_offset":56524,"parent_record_offset":56500,"ancestor_record_offset":56312,"inlinee":6413,"source_line":3168,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$60::write_str
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4140,"record_offset":57224,"parent_record_offset":57192,"ancestor_record_offset":56312,"inlinee":6417,"source_line":3162,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":57352,"parent_record_offset":57320,"ancestor_record_offset":56312,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":56604,"parent_record_offset":56572,"ancestor_record_offset":56312,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":56832,"parent_record_offset":56808,"ancestor_record_offset":56312,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":56808,"parent_record_offset":56784,"ancestor_record_offset":56312,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":57288,"parent_record_offset":57256,"ancestor_record_offset":56312,"inlinee":5425,"source_line":3033,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":57600,"parent_record_offset":57352,"ancestor_record_offset":56312,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":56572,"parent_record_offset":56548,"ancestor_record_offset":56312,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":57396,"parent_record_offset":57352,"ancestor_record_offset":56312,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":4140,"record_offset":57320,"parent_record_offset":57288,"ancestor_record_offset":56312,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4140,"record_offset":57152,"parent_record_offset":57124,"ancestor_record_offset":56312,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4140,"record_offset":59760,"parent_record_offset":59528,"ancestor_record_offset":56312,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Write::write_fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4140,"record_offset":59584,"parent_record_offset":59560,"ancestor_record_offset":56312,"inlinee":5573,"source_line":218,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Write::write_fmt::impl$1::spec_write_fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4140,"record_offset":59608,"parent_record_offset":59584,"ancestor_record_offset":56312,"inlinee":5575,"source_line":238,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::impl$1::write_fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4140,"record_offset":59560,"parent_record_offset":59528,"ancestor_record_offset":56312,"inlinee":5571,"source_line":261,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::impl$1::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4140,"record_offset":57192,"parent_record_offset":57060,"ancestor_record_offset":56312,"inlinee":6451,"source_line":253,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":4140,"record_offset":57124,"parent_record_offset":57096,"ancestor_record_offset":56312,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":4140,"record_offset":57096,"parent_record_offset":57060,"ancestor_record_offset":56312,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4140,"record_offset":56992,"parent_record_offset":56956,"ancestor_record_offset":56312,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4140,"record_offset":57024,"parent_record_offset":56956,"ancestor_record_offset":56312,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4140,"record_offset":57564,"parent_record_offset":57352,"ancestor_record_offset":56312,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4140,"record_offset":56716,"parent_record_offset":56572,"ancestor_record_offset":56312,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4140,"record_offset":59644,"parent_record_offset":59528,"ancestor_record_offset":56312,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::Serializer::number_suffixes
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":59672,"parent_record_offset":59528,"ancestor_record_offset":56312,"inlinee":5576,"source_line":466,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::Serializer::serialize_uint
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":59528,"parent_record_offset":59500,"ancestor_record_offset":56312,"inlinee":5569,"source_line":603,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::impl$3::serialize_u32
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":59500,"parent_record_offset":59472,"ancestor_record_offset":56312,"inlinee":5567,"source_line":711,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,u64>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4140,"record_offset":59860,"rva":38093824,"va":5406802944}
// 编译实例 RVAs: 0x2454400
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$::write_fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-3607721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406802944,"raw_hex":"e857f3c8ff","size":5,"soft_fail":false,"target_va":5403196032,"va":5406803748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142450ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406802944,"raw_hex":"e867c9ffff","size":5,"soft_fail":false,"target_va":5406789328,"va":5406803300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5043995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406802944,"raw_hex":"e81bf74c00","size":5,"soft_fail":false,"target_va":5411847168,"va":5406803168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-23965394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406802944,"raw_hex":"e82e5192fe","size":5,"soft_fail":false,"target_va":5382837616,"va":5406803005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4988287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406802944,"raw_hex":"e87f1d4c00","size":5,"soft_fail":false,"target_va":5411791504,"va":5406803212,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::write
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-23915442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406802944,"raw_hex":"e84e1493fe","size":5,"soft_fail":false,"target_va":5382888144,"va":5406803581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$30::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4140,"record_offset":60308,"parent_record_offset":59860,"ancestor_record_offset":59860,"inlinee":5600,"source_line":2689,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::impl$1::write_char
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4140,"record_offset":60048,"parent_record_offset":59860,"ancestor_record_offset":59860,"inlinee":6450,"source_line":257,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::impl$1::write_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4140,"record_offset":61596,"parent_record_offset":59860,"ancestor_record_offset":59860,"inlinee":6451,"source_line":253,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4140,"record_offset":62988,"parent_record_offset":59860,"ancestor_record_offset":59860,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4140,"record_offset":63376,"parent_record_offset":59860,"ancestor_record_offset":59860,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4140,"record_offset":62044,"parent_record_offset":59860,"ancestor_record_offset":59860,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] ron::ser::Serializer::compact_structs
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":60504,"parent_record_offset":59860,"ancestor_record_offset":59860,"inlinee":12931,"source_line":454,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] ron::ser::Serializer::indent
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":60608,"parent_record_offset":59860,"ancestor_record_offset":59860,"inlinee":12932,"source_line":500,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] serde::ser::impls::impl$48::serialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/ser/impls.rs
//     evidence={"module":4140,"record_offset":63020,"parent_record_offset":59860,"ancestor_record_offset":59860,"inlinee":8908,"source_line":11,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4140,"record_offset":60176,"parent_record_offset":60152,"ancestor_record_offset":59860,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4140,"record_offset":60976,"parent_record_offset":60944,"ancestor_record_offset":59860,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4140,"record_offset":61060,"parent_record_offset":61036,"ancestor_record_offset":59860,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4140,"record_offset":61036,"parent_record_offset":61004,"ancestor_record_offset":59860,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4140,"record_offset":60224,"parent_record_offset":60200,"ancestor_record_offset":59860,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4140,"record_offset":60200,"parent_record_offset":60176,"ancestor_record_offset":59860,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4140,"record_offset":61004,"parent_record_offset":60976,"ancestor_record_offset":59860,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::as_str
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4140,"record_offset":60332,"parent_record_offset":60308,"ancestor_record_offset":59860,"inlinee":5602,"source_line":1081,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4140,"record_offset":60096,"parent_record_offset":60072,"ancestor_record_offset":59860,"inlinee":6414,"source_line":1419,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::push_str
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4140,"record_offset":60804,"parent_record_offset":60772,"ancestor_record_offset":59860,"inlinee":6418,"source_line":1126,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$30::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4140,"record_offset":61200,"parent_record_offset":60608,"ancestor_record_offset":59860,"inlinee":5600,"source_line":2689,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$60::write_char
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4140,"record_offset":60072,"parent_record_offset":60048,"ancestor_record_offset":59860,"inlinee":6413,"source_line":3168,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$60::write_str
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4140,"record_offset":60772,"parent_record_offset":60740,"ancestor_record_offset":59860,"inlinee":6417,"source_line":3162,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":60900,"parent_record_offset":60868,"ancestor_record_offset":59860,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":60152,"parent_record_offset":60120,"ancestor_record_offset":59860,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":60380,"parent_record_offset":60356,"ancestor_record_offset":59860,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":60356,"parent_record_offset":60332,"ancestor_record_offset":59860,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":60836,"parent_record_offset":60804,"ancestor_record_offset":59860,"inlinee":5425,"source_line":3033,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":61148,"parent_record_offset":60900,"ancestor_record_offset":59860,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":60120,"parent_record_offset":60096,"ancestor_record_offset":59860,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":60944,"parent_record_offset":60900,"ancestor_record_offset":59860,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":4140,"record_offset":60868,"parent_record_offset":60836,"ancestor_record_offset":59860,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4140,"record_offset":60700,"parent_record_offset":60672,"ancestor_record_offset":59860,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4140,"record_offset":63308,"parent_record_offset":63076,"ancestor_record_offset":59860,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Write::write_fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4140,"record_offset":63132,"parent_record_offset":63108,"ancestor_record_offset":59860,"inlinee":5573,"source_line":218,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Write::write_fmt::impl$1::spec_write_fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4140,"record_offset":63156,"parent_record_offset":63132,"ancestor_record_offset":59860,"inlinee":5575,"source_line":238,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::impl$1::write_fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4140,"record_offset":63108,"parent_record_offset":63076,"ancestor_record_offset":59860,"inlinee":5571,"source_line":261,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::impl$1::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4140,"record_offset":60740,"parent_record_offset":60608,"ancestor_record_offset":59860,"inlinee":6451,"source_line":253,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":4140,"record_offset":60672,"parent_record_offset":60644,"ancestor_record_offset":59860,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":4140,"record_offset":60644,"parent_record_offset":60608,"ancestor_record_offset":59860,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4140,"record_offset":60540,"parent_record_offset":60504,"ancestor_record_offset":59860,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4140,"record_offset":60572,"parent_record_offset":60504,"ancestor_record_offset":59860,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4140,"record_offset":61112,"parent_record_offset":60900,"ancestor_record_offset":59860,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4140,"record_offset":60264,"parent_record_offset":60120,"ancestor_record_offset":59860,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4140,"record_offset":63192,"parent_record_offset":63076,"ancestor_record_offset":59860,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::Serializer::number_suffixes
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":63220,"parent_record_offset":63076,"ancestor_record_offset":59860,"inlinee":5576,"source_line":466,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::Serializer::serialize_uint
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":63076,"parent_record_offset":63048,"ancestor_record_offset":59860,"inlinee":5569,"source_line":603,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::impl$3::serialize_u64
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":63048,"parent_record_offset":63020,"ancestor_record_offset":59860,"inlinee":72435,"source_line":715,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: ron::ser::impl$3::serialize_newtype_variant<ref_mut$<alloc::string::String>,u32>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4140,"record_offset":87352,"rva":38100976,"va":5406810096}
// 编译实例 RVAs: 0x2455ff0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$::write_fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-3614568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406810096,"raw_hex":"e898d8c8ff","size":5,"soft_fail":false,"target_va":5403196032,"va":5406810595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142450ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406810096,"raw_hex":"e890aeffff","size":5,"soft_fail":false,"target_va":5406789328,"va":5406810171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142451200
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406810096,"raw_hex":"e8e8b1ffff","size":5,"soft_fail":false,"target_va":5406790144,"va":5406810131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-23972599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406810096,"raw_hex":"e8093592fe","size":5,"soft_fail":false,"target_va":5382837616,"va":5406810210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::write
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-23922294","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5406810096,"raw_hex":"e88af992fe","size":5,"soft_fail":false,"target_va":5382888144,"va":5406810433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::impl$1::write_char
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4140,"record_offset":87596,"parent_record_offset":87352,"ancestor_record_offset":87352,"inlinee":6450,"source_line":257,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4140,"record_offset":88020,"parent_record_offset":87352,"ancestor_record_offset":87352,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4140,"record_offset":88432,"parent_record_offset":87352,"ancestor_record_offset":87352,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4140,"record_offset":87540,"parent_record_offset":87352,"ancestor_record_offset":87352,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4140,"record_offset":88376,"parent_record_offset":87352,"ancestor_record_offset":87352,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] ron::ser::Serializer::extensions
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":87856,"parent_record_offset":87352,"ancestor_record_offset":87352,"inlinee":12953,"source_line":472,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_direct] serde::ser::impls::impl$47::serialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/ser/impls.rs
//     evidence={"module":4140,"record_offset":88052,"parent_record_offset":87352,"ancestor_record_offset":87352,"inlinee":5566,"source_line":11,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4140,"record_offset":87724,"parent_record_offset":87700,"ancestor_record_offset":87352,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4140,"record_offset":87772,"parent_record_offset":87748,"ancestor_record_offset":87352,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4140,"record_offset":87748,"parent_record_offset":87724,"ancestor_record_offset":87352,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4140,"record_offset":87644,"parent_record_offset":87620,"ancestor_record_offset":87352,"inlinee":6414,"source_line":1419,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$60::write_char
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4140,"record_offset":87620,"parent_record_offset":87596,"ancestor_record_offset":87352,"inlinee":6413,"source_line":3168,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":87700,"parent_record_offset":87668,"ancestor_record_offset":87352,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4140,"record_offset":87668,"parent_record_offset":87644,"ancestor_record_offset":87352,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4140,"record_offset":88308,"parent_record_offset":88108,"ancestor_record_offset":87352,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Write::write_fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4140,"record_offset":88164,"parent_record_offset":88140,"ancestor_record_offset":87352,"inlinee":5573,"source_line":218,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Write::write_fmt::impl$1::spec_write_fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4140,"record_offset":88188,"parent_record_offset":88164,"ancestor_record_offset":87352,"inlinee":5575,"source_line":238,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::impl$1::write_fmt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":4140,"record_offset":88140,"parent_record_offset":88108,"ancestor_record_offset":87352,"inlinee":5571,"source_line":261,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4140,"record_offset":87888,"parent_record_offset":87856,"ancestor_record_offset":87352,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4140,"record_offset":88276,"parent_record_offset":88252,"ancestor_record_offset":87352,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4140,"record_offset":87812,"parent_record_offset":87668,"ancestor_record_offset":87352,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4140,"record_offset":88224,"parent_record_offset":88108,"ancestor_record_offset":87352,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::extensions::Extensions::union
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.9.0/src/lib.rs
//     evidence={"module":4140,"record_offset":87948,"parent_record_offset":87920,"ancestor_record_offset":87352,"inlinee":12956,"source_line":757,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::extensions::_::InternalBitFlags::union
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.9.0/src/lib.rs
//     evidence={"module":4140,"record_offset":87976,"parent_record_offset":87948,"ancestor_record_offset":87352,"inlinee":12957,"source_line":757,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::extensions::_::impl$41::bitor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"2.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.9.0/src/public.rs
//     evidence={"module":4140,"record_offset":87920,"parent_record_offset":87856,"ancestor_record_offset":87352,"inlinee":12955,"source_line":382,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::Serializer::number_suffixes
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":88252,"parent_record_offset":88108,"ancestor_record_offset":87352,"inlinee":5576,"source_line":466,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::Serializer::serialize_uint
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":88108,"parent_record_offset":88080,"ancestor_record_offset":87352,"inlinee":5569,"source_line":603,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::impl$3::serialize_u32
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"ron","version":"0.8.1","source":"git+https://github.com/PounceLight/ron?rev=2787d8c#2787d8c05250c4dd706dfb358a2e9f16969d2968"}]
//     definition_source=/home/h3/.cargo/git/checkouts/ron-f84f20cc0a1e2c53/2787d8c/src/ser/mod.rs
//     evidence={"module":4140,"record_offset":88080,"parent_record_offset":88052,"ancestor_record_offset":87352,"inlinee":5567,"source_line":711,"archive":"_pdb/B/modules/04140.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: serde::de::impls::impl$36::deserialize::impl$1::visit_map<ron::de::CommaSeparated>
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4600,"record_offset":256,"rva":40424448,"va":5409133568}
// 编译实例 RVAs: 0x268d400
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142446240
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2388451","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409133568,"raw_hex":"e81d8edbff","size":5,"soft_fail":false,"target_va":5406745152,"va":5409133598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142446770
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2387253","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409133568,"raw_hex":"e8cb92dbff","size":5,"soft_fail":false,"target_va":5406746480,"va":5409133728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142446910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2386791","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409133568,"raw_hex":"e89994dbff","size":5,"soft_fail":false,"target_va":5406746896,"va":5409133682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14291f020
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2693749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409133568,"raw_hex":"e8751a2900","size":5,"soft_fail":false,"target_va":5411827744,"va":5409133990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2407711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409133568,"raw_hex":"e81fbd2400","size":5,"soft_fail":false,"target_va":5411541808,"va":5409134092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_div_by_zero
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2409411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409133568,"raw_hex":"e8c3c32400","size":5,"soft_fail":false,"target_va":5411543520,"va":5409134104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::time::Duration::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":4600,"record_offset":620,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":19137,"source_line":194,"archive":"_pdb/B/modules/04600.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4600,"record_offset":436,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/04600.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4600,"record_offset":468,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/04600.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impls::impl$36::deserialize::check_overflow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4600,"record_offset":524,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":134169,"source_line":2330,"archive":"_pdb/B/modules/04600.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":4600,"record_offset":584,"parent_record_offset":560,"ancestor_record_offset":256,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/04600.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4600,"record_offset":560,"parent_record_offset":524,"ancestor_record_offset":256,"inlinee":7229,"source_line":486,"archive":"_pdb/B/modules/04600.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4600,"record_offset":716,"parent_record_offset":620,"ancestor_record_offset":256,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/04600.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: zune_jpeg::decoder::JpegDecoder::decode_headers_internal<ref$<alloc::vec::Vec<u8,alloc::alloc::Global> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4609,"record_offset":7544,"rva":40467408,"va":5409176528}
// 编译实例 RVAs: 0x2697bd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34096798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409176528,"raw_hex":"e862b9f7fd","size":5,"soft_fail":false,"target_va":5375081488,"va":5409178281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14287eb70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1993485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409176528,"raw_hex":"e80d6b1e00","size":5,"soft_fail":false,"target_va":5411171184,"va":5409177694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14287ef40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1994476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409176528,"raw_hex":"e8ec6e1e00","size":5,"soft_fail":false,"target_va":5411172160,"va":5409177679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14287f660
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1996270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409176528,"raw_hex":"e8ee751e00","size":5,"soft_fail":false,"target_va":5411173984,"va":5409177709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14287ffa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1998749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409176528,"raw_hex":"e89d7f1e00","size":5,"soft_fail":false,"target_va":5411176352,"va":5409177598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142880ec0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2002589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409176528,"raw_hex":"e89d8e1e00","size":5,"soft_fail":false,"target_va":5411180224,"va":5409177630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142882000
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2007022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409176528,"raw_hex":"e8ee9f1e00","size":5,"soft_fail":false,"target_va":5411184640,"va":5409177613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142883320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2011355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409176528,"raw_hex":"e8dbb01e00","size":5,"soft_fail":false,"target_va":5411189536,"va":5409178176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-26337637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409176528,"raw_hex":"e89b1e6efe","size":5,"soft_fail":false,"target_va":5382840208,"va":5409177840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2362365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409176528,"raw_hex":"e8fd0b2400","size":5,"soft_fail":false,"target_va":5411540963,"va":5409178593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::slice::hack::impl$1::to_vec<u8,alloc::alloc::Global>
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409176528,"raw_hex":"e8c4ebffff","size":5,"soft_fail":false,"target_va":5409173168,"va":5409178343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2363251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409176528,"raw_hex":"e8730f2400","size":5,"soft_fail":false,"target_va":5411541776,"va":5409178520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2364304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409176528,"raw_hex":"e890132400","size":5,"soft_fail":false,"target_va":5411542880,"va":5409178571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zune_core::bytestream::reader::ZByteReader$LT$T$GT$::read_exact
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1992234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409176528,"raw_hex":"e82a661e00","size":5,"soft_fail":false,"target_va":5411169536,"va":5409177297,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zune_jpeg::color_convert::choose_ycbcr_to_rgb_convert_func
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zune-jpeg","version":"0.4.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-28144059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409176528,"raw_hex":"e8458e52fe","size":5,"soft_fail":false,"target_va":5381032560,"va":5409176614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zune_jpeg::errors::UnsupportedSchemes::from_int
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zune-jpeg","version":"0.4.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-28136603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409176528,"raw_hex":"e865ab52fe","size":5,"soft_fail":false,"target_va":5381041648,"va":5409178246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zune_jpeg::marker::Marker::from_u8
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zune-jpeg","version":"0.4.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-3415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409176528,"raw_hex":"e8a9f2ffff","size":5,"soft_fail":false,"target_va":5409173536,"va":5409176946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":4609,"record_offset":12288,"parent_record_offset":7544,"ancestor_record_offset":7544,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4609,"record_offset":7804,"parent_record_offset":7544,"ancestor_record_offset":7544,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4609,"record_offset":8184,"parent_record_offset":7544,"ancestor_record_offset":7544,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_direct] zune_core::bytestream::reader::ZByteReader::get_u16_be_err
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4609,"record_offset":7840,"parent_record_offset":7544,"ancestor_record_offset":7544,"inlinee":10823,"source_line":378,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_direct] zune_core::bytestream::reader::ZByteReader::get_u8_err
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4609,"record_offset":8212,"parent_record_offset":7544,"ancestor_record_offset":7544,"inlinee":10825,"source_line":439,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_direct] zune_core::bytestream::reader::ZByteReader::skip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4609,"record_offset":9120,"parent_record_offset":7544,"ancestor_record_offset":7544,"inlinee":10814,"source_line":96,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_direct] zune_core::options::decoder::DecoderOptions::get_strict_mode
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/options/decoder.rs
//     evidence={"module":4609,"record_offset":9176,"parent_record_offset":7544,"ancestor_record_offset":7544,"inlinee":10822,"source_line":228,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_direct] zune_core::options::decoder::DecoderOptions::jpeg_get_out_colorspace
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/options/decoder.rs
//     evidence={"module":4609,"record_offset":7772,"parent_record_offset":7544,"ancestor_record_offset":7544,"inlinee":15200,"source_line":431,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_direct] zune_jpeg::decoder::JpegDecoder::parse_marker_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-jpeg","version":"0.4.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-jpeg-0.4.11/src/decoder.rs
//     evidence={"module":4609,"record_offset":9208,"parent_record_offset":7544,"ancestor_record_offset":7544,"inlinee":15201,"source_line":453,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_direct] zune_jpeg::marker::impl$5::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-jpeg","version":"0.4.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-jpeg-0.4.11/src/marker.rs
//     evidence={"module":4609,"record_offset":12228,"parent_record_offset":7544,"ancestor_record_offset":7544,"inlinee":15203,"source_line":11,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4609,"record_offset":11748,"parent_record_offset":11724,"ancestor_record_offset":7544,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4609,"record_offset":11772,"parent_record_offset":11748,"ancestor_record_offset":7544,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4609,"record_offset":11724,"parent_record_offset":11696,"ancestor_record_offset":7544,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":4609,"record_offset":11152,"parent_record_offset":9208,"ancestor_record_offset":7544,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":4609,"record_offset":11200,"parent_record_offset":11176,"ancestor_record_offset":7544,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4609,"record_offset":8416,"parent_record_offset":8392,"ancestor_record_offset":7544,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4609,"record_offset":11636,"parent_record_offset":11608,"ancestor_record_offset":7544,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4609,"record_offset":8464,"parent_record_offset":8440,"ancestor_record_offset":7544,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4609,"record_offset":8440,"parent_record_offset":8416,"ancestor_record_offset":7544,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4609,"record_offset":11696,"parent_record_offset":11664,"ancestor_record_offset":7544,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4609,"record_offset":11664,"parent_record_offset":11636,"ancestor_record_offset":7544,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4609,"record_offset":11572,"parent_record_offset":11544,"ancestor_record_offset":7544,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4609,"record_offset":11544,"parent_record_offset":11516,"ancestor_record_offset":7544,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4609,"record_offset":11488,"parent_record_offset":11460,"ancestor_record_offset":7544,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4609,"record_offset":11516,"parent_record_offset":11488,"ancestor_record_offset":7544,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4609,"record_offset":11460,"parent_record_offset":11432,"ancestor_record_offset":7544,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":4609,"record_offset":11432,"parent_record_offset":11404,"ancestor_record_offset":7544,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$110::spec_to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4609,"record_offset":11376,"parent_record_offset":11348,"ancestor_record_offset":7544,"inlinee":10837,"source_line":2877,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4609,"record_offset":11348,"parent_record_offset":9208,"ancestor_record_offset":7544,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4609,"record_offset":11404,"parent_record_offset":11376,"ancestor_record_offset":7544,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4609,"record_offset":8392,"parent_record_offset":8364,"ancestor_record_offset":7544,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4609,"record_offset":8364,"parent_record_offset":8284,"ancestor_record_offset":7544,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4609,"record_offset":11608,"parent_record_offset":11572,"ancestor_record_offset":7544,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":4609,"record_offset":9920,"parent_record_offset":9896,"ancestor_record_offset":7544,"inlinee":10090,"source_line":9,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":4609,"record_offset":9944,"parent_record_offset":9920,"ancestor_record_offset":7544,"inlinee":9478,"source_line":146,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4609,"record_offset":9896,"parent_record_offset":9208,"ancestor_record_offset":7544,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":4609,"record_offset":8060,"parent_record_offset":8036,"ancestor_record_offset":7544,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4609,"record_offset":7996,"parent_record_offset":7968,"ancestor_record_offset":7544,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4609,"record_offset":9828,"parent_record_offset":9804,"ancestor_record_offset":7544,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4609,"record_offset":9144,"parent_record_offset":9120,"ancestor_record_offset":7544,"inlinee":4914,"source_line":1980,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::from_be
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4609,"record_offset":8116,"parent_record_offset":8092,"ancestor_record_offset":7544,"inlinee":10808,"source_line":355,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::from_be_bytes
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4609,"record_offset":8092,"parent_record_offset":7872,"ancestor_record_offset":7544,"inlinee":10807,"source_line":3510,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::swap_bytes
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4609,"record_offset":8140,"parent_record_offset":8116,"ancestor_record_offset":7544,"inlinee":10809,"source_line":306,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4609,"record_offset":11176,"parent_record_offset":11152,"ancestor_record_offset":7544,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":4609,"record_offset":11824,"parent_record_offset":11572,"ancestor_record_offset":7544,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4609,"record_offset":9860,"parent_record_offset":9208,"ancestor_record_offset":7544,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4609,"record_offset":11072,"parent_record_offset":9208,"ancestor_record_offset":7544,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4609,"record_offset":8036,"parent_record_offset":7872,"ancestor_record_offset":7544,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4609,"record_offset":7940,"parent_record_offset":7912,"ancestor_record_offset":7544,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4609,"record_offset":8332,"parent_record_offset":8284,"ancestor_record_offset":7544,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4609,"record_offset":7968,"parent_record_offset":7940,"ancestor_record_offset":7544,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] zune_core::bytestream::reader::ZByteReader::get_u16_be_err
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4609,"record_offset":9432,"parent_record_offset":9208,"ancestor_record_offset":7544,"inlinee":10823,"source_line":378,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] zune_core::bytestream::reader::ZByteReader::get_u16_inner_or_die
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4609,"record_offset":7872,"parent_record_offset":7840,"ancestor_record_offset":7544,"inlinee":10824,"source_line":353,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] zune_core::bytestream::reader::ZByteReader::has
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4609,"record_offset":9804,"parent_record_offset":9208,"ancestor_record_offset":7544,"inlinee":10810,"source_line":121,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] zune_core::bytestream::reader::ZByteReader::skip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4609,"record_offset":9980,"parent_record_offset":9208,"ancestor_record_offset":7544,"inlinee":10814,"source_line":96,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] zune_core::bytestream::traits::impl$2::get_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/traits.rs
//     evidence={"module":4609,"record_offset":8252,"parent_record_offset":8212,"ancestor_record_offset":7544,"inlinee":10819,"source_line":105,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] zune_core::bytestream::traits::impl$2::get_slice
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/traits.rs
//     evidence={"module":4609,"record_offset":7912,"parent_record_offset":7872,"ancestor_record_offset":7544,"inlinee":10816,"source_line":110,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: zune_jpeg::decoder::JpegDecoder::decode_headers_internal<ref$<slice2$<u8> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4609,"record_offset":3176,"rva":40465344,"va":5409174464}
// 编译实例 RVAs: 0x26973c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34094719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409174464,"raw_hex":"e881c1f7fd","size":5,"soft_fail":false,"target_va":5375081488,"va":5409176202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14287ed50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1996081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409174464,"raw_hex":"e831751e00","size":5,"soft_fail":false,"target_va":5411171664,"va":5409175578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14287f1d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1997248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409174464,"raw_hex":"e8c0791e00","size":5,"soft_fail":false,"target_va":5411172816,"va":5409175563,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14287f460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1997874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409174464,"raw_hex":"e8327c1e00","size":5,"soft_fail":false,"target_va":5411173472,"va":5409175593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14287f850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1999000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409174464,"raw_hex":"e898801e00","size":5,"soft_fail":false,"target_va":5411174480,"va":5409175475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428806f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2002712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409174464,"raw_hex":"e8188f1e00","size":5,"soft_fail":false,"target_va":5411178224,"va":5409175507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142881700
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2006841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409174464,"raw_hex":"e8399f1e00","size":5,"soft_fail":false,"target_va":5411182336,"va":5409175490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142882910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2010856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409174464,"raw_hex":"e8e8ae1e00","size":5,"soft_fail":false,"target_va":5411186960,"va":5409176099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-26335555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409174464,"raw_hex":"e8bd266efe","size":5,"soft_fail":false,"target_va":5382840208,"va":5409175758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2364444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409174464,"raw_hex":"e81c142400","size":5,"soft_fail":false,"target_va":5411540963,"va":5409176514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::slice::hack::impl$1::to_vec<u8,alloc::alloc::Global>
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-3101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409174464,"raw_hex":"e8e3f3ffff","size":5,"soft_fail":false,"target_va":5409173168,"va":5409176264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2365330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409174464,"raw_hex":"e892172400","size":5,"soft_fail":false,"target_va":5411541776,"va":5409176441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2366383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409174464,"raw_hex":"e8af1b2400","size":5,"soft_fail":false,"target_va":5411542880,"va":5409176492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zune_core::bytestream::reader::ZByteReader$LT$T$GT$::read_exact
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1994489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409174464,"raw_hex":"e8f96e1e00","size":5,"soft_fail":false,"target_va":5411169680,"va":5409175186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zune_jpeg::color_convert::choose_ycbcr_to_rgb_convert_func
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zune-jpeg","version":"0.4.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-28141998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409174464,"raw_hex":"e8529652fe","size":5,"soft_fail":false,"target_va":5381032560,"va":5409174553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zune_jpeg::errors::UnsupportedSchemes::from_int
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zune-jpeg","version":"0.4.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-28134524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409174464,"raw_hex":"e884b352fe","size":5,"soft_fail":false,"target_va":5381041648,"va":5409176167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zune_jpeg::marker::Marker::from_u8
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zune-jpeg","version":"0.4.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-1315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409174464,"raw_hex":"e8ddfaffff","size":5,"soft_fail":false,"target_va":5409173536,"va":5409174846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":4609,"record_offset":7372,"parent_record_offset":3176,"ancestor_record_offset":3176,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4609,"record_offset":3408,"parent_record_offset":3176,"ancestor_record_offset":3176,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4609,"record_offset":3784,"parent_record_offset":3176,"ancestor_record_offset":3176,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_direct] zune_core::bytestream::reader::ZByteReader::get_u16_be_err
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4609,"record_offset":3444,"parent_record_offset":3176,"ancestor_record_offset":3176,"inlinee":10823,"source_line":378,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_direct] zune_core::bytestream::reader::ZByteReader::get_u8_err
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4609,"record_offset":3812,"parent_record_offset":3176,"ancestor_record_offset":3176,"inlinee":10825,"source_line":439,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_direct] zune_core::bytestream::reader::ZByteReader::skip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4609,"record_offset":4392,"parent_record_offset":3176,"ancestor_record_offset":3176,"inlinee":10814,"source_line":96,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_direct] zune_core::options::decoder::DecoderOptions::get_strict_mode
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/options/decoder.rs
//     evidence={"module":4609,"record_offset":4448,"parent_record_offset":3176,"ancestor_record_offset":3176,"inlinee":10822,"source_line":228,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_direct] zune_core::options::decoder::DecoderOptions::jpeg_get_out_colorspace
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/options/decoder.rs
//     evidence={"module":4609,"record_offset":3376,"parent_record_offset":3176,"ancestor_record_offset":3176,"inlinee":15200,"source_line":431,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_direct] zune_jpeg::decoder::JpegDecoder::parse_marker_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-jpeg","version":"0.4.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-jpeg-0.4.11/src/decoder.rs
//     evidence={"module":4609,"record_offset":4480,"parent_record_offset":3176,"ancestor_record_offset":3176,"inlinee":15201,"source_line":453,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_direct] zune_jpeg::marker::impl$5::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-jpeg","version":"0.4.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-jpeg-0.4.11/src/marker.rs
//     evidence={"module":4609,"record_offset":7312,"parent_record_offset":3176,"ancestor_record_offset":3176,"inlinee":15203,"source_line":11,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4609,"record_offset":6832,"parent_record_offset":6808,"ancestor_record_offset":3176,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4609,"record_offset":6856,"parent_record_offset":6832,"ancestor_record_offset":3176,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4609,"record_offset":6808,"parent_record_offset":6780,"ancestor_record_offset":3176,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":4609,"record_offset":6236,"parent_record_offset":4480,"ancestor_record_offset":3176,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":4609,"record_offset":6284,"parent_record_offset":6260,"ancestor_record_offset":3176,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4609,"record_offset":6720,"parent_record_offset":6692,"ancestor_record_offset":3176,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4609,"record_offset":6780,"parent_record_offset":6748,"ancestor_record_offset":3176,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4609,"record_offset":6748,"parent_record_offset":6720,"ancestor_record_offset":3176,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4609,"record_offset":6656,"parent_record_offset":6628,"ancestor_record_offset":3176,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4609,"record_offset":6628,"parent_record_offset":6600,"ancestor_record_offset":3176,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4609,"record_offset":6572,"parent_record_offset":6544,"ancestor_record_offset":3176,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4609,"record_offset":6600,"parent_record_offset":6572,"ancestor_record_offset":3176,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4609,"record_offset":6544,"parent_record_offset":6516,"ancestor_record_offset":3176,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":4609,"record_offset":6516,"parent_record_offset":6488,"ancestor_record_offset":3176,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$110::spec_to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4609,"record_offset":6460,"parent_record_offset":6432,"ancestor_record_offset":3176,"inlinee":10837,"source_line":2877,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4609,"record_offset":6432,"parent_record_offset":4480,"ancestor_record_offset":3176,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4609,"record_offset":6488,"parent_record_offset":6460,"ancestor_record_offset":3176,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4609,"record_offset":6692,"parent_record_offset":6656,"ancestor_record_offset":3176,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":4609,"record_offset":5084,"parent_record_offset":5060,"ancestor_record_offset":3176,"inlinee":10090,"source_line":9,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":4609,"record_offset":5108,"parent_record_offset":5084,"ancestor_record_offset":3176,"inlinee":9478,"source_line":146,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4609,"record_offset":5060,"parent_record_offset":4480,"ancestor_record_offset":3176,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":4609,"record_offset":3660,"parent_record_offset":3636,"ancestor_record_offset":3176,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4609,"record_offset":3596,"parent_record_offset":3572,"ancestor_record_offset":3176,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4609,"record_offset":4416,"parent_record_offset":4392,"ancestor_record_offset":3176,"inlinee":4914,"source_line":1980,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::from_be
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4609,"record_offset":3716,"parent_record_offset":3692,"ancestor_record_offset":3176,"inlinee":10808,"source_line":355,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::from_be_bytes
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4609,"record_offset":3692,"parent_record_offset":3476,"ancestor_record_offset":3176,"inlinee":10807,"source_line":3510,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::swap_bytes
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4609,"record_offset":3740,"parent_record_offset":3716,"ancestor_record_offset":3176,"inlinee":10809,"source_line":306,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4609,"record_offset":6260,"parent_record_offset":6236,"ancestor_record_offset":3176,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":4609,"record_offset":6908,"parent_record_offset":6656,"ancestor_record_offset":3176,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4609,"record_offset":5024,"parent_record_offset":4480,"ancestor_record_offset":3176,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4609,"record_offset":6156,"parent_record_offset":4480,"ancestor_record_offset":3176,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4609,"record_offset":3636,"parent_record_offset":3476,"ancestor_record_offset":3176,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4609,"record_offset":3548,"parent_record_offset":3516,"ancestor_record_offset":3176,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4609,"record_offset":3912,"parent_record_offset":3884,"ancestor_record_offset":3176,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4609,"record_offset":3572,"parent_record_offset":3548,"ancestor_record_offset":3176,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] zune_core::bytestream::reader::ZByteReader::get_u16_be_err
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4609,"record_offset":4704,"parent_record_offset":4480,"ancestor_record_offset":3176,"inlinee":10823,"source_line":378,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] zune_core::bytestream::reader::ZByteReader::get_u16_inner_or_die
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4609,"record_offset":3476,"parent_record_offset":3444,"ancestor_record_offset":3176,"inlinee":10824,"source_line":353,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] zune_core::bytestream::reader::ZByteReader::skip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4609,"record_offset":5144,"parent_record_offset":4480,"ancestor_record_offset":3176,"inlinee":10814,"source_line":96,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] zune_core::bytestream::traits::impl$0::get_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/traits.rs
//     evidence={"module":4609,"record_offset":3852,"parent_record_offset":3812,"ancestor_record_offset":3176,"inlinee":10818,"source_line":71,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] zune_core::bytestream::traits::impl$0::get_slice
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/traits.rs
//     evidence={"module":4609,"record_offset":3516,"parent_record_offset":3476,"ancestor_record_offset":3176,"inlinee":10806,"source_line":76,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: zune_jpeg::decoder::JpegDecoder::decode_into<ref$<slice2$<u8> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4609,"record_offset":940,"rva":40464560,"va":5409173680}
// 编译实例 RVAs: 0x26970b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1426973c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409173680,"raw_hex":"e8f1020000","size":5,"soft_fail":false,"target_va":5409174464,"va":5409173706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2367910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409173680,"raw_hex":"e8a6212400","size":5,"soft_fail":false,"target_va":5411541776,"va":5409173861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zune_jpeg::mcu::_$LT$impl$u20$zune_jpeg..decoder..JpegDecoder$LT$T$GT$$GT$::decode_mcu_ycbcr_baseline
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zune-jpeg","version":"0.4.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-28210652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409173680,"raw_hex":"e8248a51fe","size":5,"soft_fail":false,"target_va":5380963200,"va":5409173847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zune_jpeg::mcu_prog::_$LT$impl$u20$zune_jpeg..decoder..JpegDecoder$LT$T$GT$$GT$::decode_mcu_ycbcr_progressive
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zune-jpeg","version":"0.4.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-28193532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5409173680,"raw_hex":"e804cd51fe","size":5,"soft_fail":false,"target_va":5380980304,"va":5409173831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4609,"record_offset":1328,"parent_record_offset":940,"ancestor_record_offset":940,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4609,"record_offset":1100,"parent_record_offset":940,"ancestor_record_offset":940,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_direct] zune_jpeg::decoder::JpegDecoder::output_buffer_size
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-jpeg","version":"0.4.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-jpeg-0.4.11/src/decoder.rs
//     evidence={"module":4609,"record_offset":1132,"parent_record_offset":940,"ancestor_record_offset":940,"inlinee":15197,"source_line":255,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4609,"record_offset":1220,"parent_record_offset":1188,"ancestor_record_offset":940,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] zune_core::options::decoder::DecoderOptions::jpeg_get_out_colorspace
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/options/decoder.rs
//     evidence={"module":4609,"record_offset":1252,"parent_record_offset":1132,"ancestor_record_offset":940,"inlinee":15200,"source_line":431,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] zune_jpeg::decoder::JpegDecoder::height
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-jpeg","version":"0.4.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-jpeg-0.4.11/src/decoder.rs
//     evidence={"module":4609,"record_offset":1188,"parent_record_offset":1132,"ancestor_record_offset":940,"inlinee":15199,"source_line":817,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   [pdb_inline_transitive] zune_jpeg::decoder::JpegDecoder::width
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-jpeg","version":"0.4.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-jpeg-0.4.11/src/decoder.rs
//     evidence={"module":4609,"record_offset":1160,"parent_record_offset":1132,"ancestor_record_offset":940,"inlinee":15198,"source_line":809,"archive":"_pdb/B/modules/04609.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: zune_jpeg::headers::parse_app14<ref$<alloc::vec::Vec<u8,alloc::alloc::Global> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4756,"record_offset":10944,"rva":42464864,"va":5411173984}
// 编译实例 RVAs: 0x287f660
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-28334213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411173984,"raw_hex":"e87ba74ffe","size":5,"soft_fail":false,"target_va":5382840208,"va":5411174416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":4756,"record_offset":12192,"parent_record_offset":10944,"ancestor_record_offset":10944,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4756,"record_offset":11880,"parent_record_offset":10944,"ancestor_record_offset":10944,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$31::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4756,"record_offset":11708,"parent_record_offset":10944,"ancestor_record_offset":10944,"inlinee":10821,"source_line":524,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] zune_core::bytestream::reader::ZByteReader::get_u16_be
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4756,"record_offset":11144,"parent_record_offset":10944,"ancestor_record_offset":10944,"inlinee":10803,"source_line":393,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] zune_core::bytestream::reader::ZByteReader::get_u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4756,"record_offset":12048,"parent_record_offset":10944,"ancestor_record_offset":10944,"inlinee":10817,"source_line":425,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] zune_core::bytestream::reader::ZByteReader::has
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4756,"record_offset":11508,"parent_record_offset":10944,"ancestor_record_offset":10944,"inlinee":10810,"source_line":121,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] zune_core::bytestream::reader::ZByteReader::peek_at
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4756,"record_offset":11564,"parent_record_offset":10944,"ancestor_record_offset":10944,"inlinee":10811,"source_line":187,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] zune_core::bytestream::reader::ZByteReader::skip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4756,"record_offset":11936,"parent_record_offset":10944,"ancestor_record_offset":10944,"inlinee":10814,"source_line":96,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] zune_core::options::decoder::DecoderOptions::get_strict_mode
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/options/decoder.rs
//     evidence={"module":4756,"record_offset":11848,"parent_record_offset":10944,"ancestor_record_offset":10944,"inlinee":10822,"source_line":228,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":4756,"record_offset":12240,"parent_record_offset":12216,"ancestor_record_offset":10944,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4756,"record_offset":11324,"parent_record_offset":11224,"ancestor_record_offset":10944,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4756,"record_offset":11732,"parent_record_offset":11708,"ancestor_record_offset":10944,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":4756,"record_offset":11384,"parent_record_offset":11360,"ancestor_record_offset":10944,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4756,"record_offset":11292,"parent_record_offset":11264,"ancestor_record_offset":10944,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4756,"record_offset":11532,"parent_record_offset":11508,"ancestor_record_offset":10944,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4756,"record_offset":11960,"parent_record_offset":11936,"ancestor_record_offset":10944,"inlinee":4914,"source_line":1980,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::from_be
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4756,"record_offset":11440,"parent_record_offset":11416,"ancestor_record_offset":10944,"inlinee":10808,"source_line":355,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::from_be_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4756,"record_offset":11416,"parent_record_offset":11164,"ancestor_record_offset":10944,"inlinee":10807,"source_line":3510,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::swap_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4756,"record_offset":11464,"parent_record_offset":11440,"ancestor_record_offset":10944,"inlinee":10809,"source_line":306,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4756,"record_offset":12216,"parent_record_offset":12192,"ancestor_record_offset":10944,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4756,"record_offset":11780,"parent_record_offset":11756,"ancestor_record_offset":10944,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4756,"record_offset":11804,"parent_record_offset":11780,"ancestor_record_offset":10944,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4756,"record_offset":11360,"parent_record_offset":11164,"ancestor_record_offset":10944,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4756,"record_offset":11224,"parent_record_offset":11196,"ancestor_record_offset":10944,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4756,"record_offset":12124,"parent_record_offset":12100,"ancestor_record_offset":10944,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4756,"record_offset":11264,"parent_record_offset":11224,"ancestor_record_offset":10944,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] zune_core::bytestream::reader::ZByteReader::get_u16_inner_or_default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4756,"record_offset":11164,"parent_record_offset":11144,"ancestor_record_offset":10944,"inlinee":10804,"source_line":329,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] zune_core::bytestream::traits::impl$2::get_byte
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/traits.rs
//     evidence={"module":4756,"record_offset":12076,"parent_record_offset":12048,"ancestor_record_offset":10944,"inlinee":10819,"source_line":105,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] zune_core::bytestream::traits::impl$2::get_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/traits.rs
//     evidence={"module":4756,"record_offset":11196,"parent_record_offset":11164,"ancestor_record_offset":10944,"inlinee":10816,"source_line":110,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: zune_jpeg::headers::parse_app14<ref$<slice2$<u8> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4756,"record_offset":9716,"rva":42464352,"va":5411173472}
// 编译实例 RVAs: 0x287f460
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-28333711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411173472,"raw_hex":"e871a94ffe","size":5,"soft_fail":false,"target_va":5382840208,"va":5411173914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":4756,"record_offset":10856,"parent_record_offset":9716,"ancestor_record_offset":9716,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4756,"record_offset":10568,"parent_record_offset":9716,"ancestor_record_offset":9716,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$31::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4756,"record_offset":10396,"parent_record_offset":9716,"ancestor_record_offset":9716,"inlinee":10821,"source_line":524,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] zune_core::bytestream::reader::ZByteReader::get_u16_be
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4756,"record_offset":9884,"parent_record_offset":9716,"ancestor_record_offset":9716,"inlinee":10803,"source_line":393,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] zune_core::bytestream::reader::ZByteReader::get_u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4756,"record_offset":10708,"parent_record_offset":9716,"ancestor_record_offset":9716,"inlinee":10817,"source_line":425,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] zune_core::bytestream::reader::ZByteReader::has
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4756,"record_offset":10196,"parent_record_offset":9716,"ancestor_record_offset":9716,"inlinee":10810,"source_line":121,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] zune_core::bytestream::reader::ZByteReader::peek_at
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4756,"record_offset":10252,"parent_record_offset":9716,"ancestor_record_offset":9716,"inlinee":10811,"source_line":187,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] zune_core::bytestream::reader::ZByteReader::skip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4756,"record_offset":10596,"parent_record_offset":9716,"ancestor_record_offset":9716,"inlinee":10814,"source_line":96,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] zune_core::options::decoder::DecoderOptions::get_strict_mode
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/options/decoder.rs
//     evidence={"module":4756,"record_offset":10536,"parent_record_offset":9716,"ancestor_record_offset":9716,"inlinee":10822,"source_line":228,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":4756,"record_offset":10904,"parent_record_offset":10880,"ancestor_record_offset":9716,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4756,"record_offset":10420,"parent_record_offset":10396,"ancestor_record_offset":9716,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":4756,"record_offset":10072,"parent_record_offset":10048,"ancestor_record_offset":9716,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4756,"record_offset":10008,"parent_record_offset":9984,"ancestor_record_offset":9716,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4756,"record_offset":10220,"parent_record_offset":10196,"ancestor_record_offset":9716,"inlinee":5558,"source_line":1811,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4756,"record_offset":10620,"parent_record_offset":10596,"ancestor_record_offset":9716,"inlinee":4914,"source_line":1980,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::from_be
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4756,"record_offset":10128,"parent_record_offset":10104,"ancestor_record_offset":9716,"inlinee":10808,"source_line":355,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::from_be_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4756,"record_offset":10104,"parent_record_offset":9904,"ancestor_record_offset":9716,"inlinee":10807,"source_line":3510,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::swap_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4756,"record_offset":10152,"parent_record_offset":10128,"ancestor_record_offset":9716,"inlinee":10809,"source_line":306,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4756,"record_offset":10880,"parent_record_offset":10856,"ancestor_record_offset":9716,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4756,"record_offset":10468,"parent_record_offset":10444,"ancestor_record_offset":9716,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":4756,"record_offset":10492,"parent_record_offset":10468,"ancestor_record_offset":9716,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4756,"record_offset":10048,"parent_record_offset":9904,"ancestor_record_offset":9716,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4756,"record_offset":9960,"parent_record_offset":9936,"ancestor_record_offset":9716,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4756,"record_offset":10788,"parent_record_offset":10764,"ancestor_record_offset":9716,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4756,"record_offset":9984,"parent_record_offset":9960,"ancestor_record_offset":9716,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] zune_core::bytestream::reader::ZByteReader::get_u16_inner_or_default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4756,"record_offset":9904,"parent_record_offset":9884,"ancestor_record_offset":9716,"inlinee":10804,"source_line":329,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] zune_core::bytestream::traits::impl$0::get_byte
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/traits.rs
//     evidence={"module":4756,"record_offset":10740,"parent_record_offset":10708,"ancestor_record_offset":9716,"inlinee":10818,"source_line":71,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] zune_core::bytestream::traits::impl$0::get_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/traits.rs
//     evidence={"module":4756,"record_offset":9936,"parent_record_offset":9904,"ancestor_record_offset":9716,"inlinee":10806,"source_line":76,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: zune_jpeg::headers::parse_dqt<ref$<alloc::vec::Vec<u8,alloc::alloc::Global> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4756,"record_offset":26472,"rva":42475520,"va":5411184640}
// 编译实例 RVAs: 0x2882000
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t661726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411184640,"raw_hex":"e8de180a00","size":5,"soft_fail":false,"target_va":5411847168,"va":5411185437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-28346194","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411184640,"raw_hex":"e8ae784ffe","size":5,"soft_fail":false,"target_va":5382840208,"va":5411186397,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t354819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411184640,"raw_hex":"e8036a0500","size":5,"soft_fail":false,"target_va":5411541776,"va":5411186952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t355351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411184640,"raw_hex":"e8176c0500","size":5,"soft_fail":false,"target_va":5411542296,"va":5411186940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t356176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411184640,"raw_hex":"e8506f0500","size":5,"soft_fail":false,"target_va":5411543104,"va":5411186923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":4756,"record_offset":28732,"parent_record_offset":26472,"ancestor_record_offset":26472,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":4756,"record_offset":27276,"parent_record_offset":26472,"ancestor_record_offset":26472,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$7::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4756,"record_offset":27076,"parent_record_offset":26472,"ancestor_record_offset":26472,"inlinee":10849,"source_line":653,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4756,"record_offset":27104,"parent_record_offset":26472,"ancestor_record_offset":26472,"inlinee":9461,"source_line":1252,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4756,"record_offset":27952,"parent_record_offset":26472,"ancestor_record_offset":26472,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4756,"record_offset":27048,"parent_record_offset":26472,"ancestor_record_offset":26472,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] zune_core::bytestream::reader::ZByteReader::get_u16_be_err
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4756,"record_offset":26672,"parent_record_offset":26472,"ancestor_record_offset":26472,"inlinee":10823,"source_line":378,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] zune_core::bytestream::reader::ZByteReader::get_u8_err
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4756,"record_offset":27160,"parent_record_offset":26472,"ancestor_record_offset":26472,"inlinee":10825,"source_line":439,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] zune_core::bytestream::reader::ZByteReader::read_exact
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4756,"record_offset":27476,"parent_record_offset":26472,"ancestor_record_offset":26472,"inlinee":10826,"source_line":306,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] zune_jpeg::headers::un_zig_zag
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"zune-jpeg","version":"0.4.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-jpeg-0.4.11/src/headers.rs
//     evidence={"module":4756,"record_offset":28112,"parent_record_offset":26472,"ancestor_record_offset":26472,"inlinee":10851,"source_line":532,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":4756,"record_offset":28008,"parent_record_offset":27984,"ancestor_record_offset":26472,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":4756,"record_offset":28064,"parent_record_offset":28036,"ancestor_record_offset":26472,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4756,"record_offset":26864,"parent_record_offset":26764,"ancestor_record_offset":26472,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4756,"record_offset":27580,"parent_record_offset":27556,"ancestor_record_offset":26472,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4756,"record_offset":27440,"parent_record_offset":27324,"ancestor_record_offset":26472,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4756,"record_offset":27556,"parent_record_offset":27516,"ancestor_record_offset":26472,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":4756,"record_offset":26924,"parent_record_offset":26900,"ancestor_record_offset":26472,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":4756,"record_offset":27376,"parent_record_offset":27324,"ancestor_record_offset":26472,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":4756,"record_offset":27324,"parent_record_offset":27276,"ancestor_record_offset":26472,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":4756,"record_offset":28148,"parent_record_offset":28112,"ancestor_record_offset":26472,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4756,"record_offset":26832,"parent_record_offset":26804,"ancestor_record_offset":26472,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4756,"record_offset":27404,"parent_record_offset":27376,"ancestor_record_offset":26472,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::from_be
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4756,"record_offset":26980,"parent_record_offset":26956,"ancestor_record_offset":26472,"inlinee":10808,"source_line":355,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::from_be_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4756,"record_offset":26956,"parent_record_offset":26700,"ancestor_record_offset":26472,"inlinee":10807,"source_line":3510,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::swap_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4756,"record_offset":27004,"parent_record_offset":26980,"ancestor_record_offset":26472,"inlinee":10809,"source_line":306,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4756,"record_offset":28036,"parent_record_offset":28008,"ancestor_record_offset":26472,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4756,"record_offset":26900,"parent_record_offset":26700,"ancestor_record_offset":26472,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4756,"record_offset":26764,"parent_record_offset":26736,"ancestor_record_offset":26472,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4756,"record_offset":27816,"parent_record_offset":27792,"ancestor_record_offset":26472,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4756,"record_offset":27792,"parent_record_offset":27736,"ancestor_record_offset":26472,"inlinee":4612,"source_line":101,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4756,"record_offset":27612,"parent_record_offset":27516,"ancestor_record_offset":26472,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4756,"record_offset":27236,"parent_record_offset":27212,"ancestor_record_offset":26472,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4756,"record_offset":26804,"parent_record_offset":26764,"ancestor_record_offset":26472,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4756,"record_offset":27648,"parent_record_offset":27612,"ancestor_record_offset":26472,"inlinee":5308,"source_line":444,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] zune_core::bytestream::reader::ZByteReader::get_u16_inner_or_die
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4756,"record_offset":26700,"parent_record_offset":26672,"ancestor_record_offset":26472,"inlinee":10824,"source_line":353,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] zune_core::bytestream::reader::ZByteReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4756,"record_offset":27516,"parent_record_offset":27476,"ancestor_record_offset":26472,"inlinee":10827,"source_line":292,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] zune_core::bytestream::reader::ZByteReader::skip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4756,"record_offset":27916,"parent_record_offset":27516,"ancestor_record_offset":26472,"inlinee":10814,"source_line":96,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] zune_core::bytestream::traits::impl$2::get_byte
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/traits.rs
//     evidence={"module":4756,"record_offset":27188,"parent_record_offset":27160,"ancestor_record_offset":26472,"inlinee":10819,"source_line":105,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] zune_core::bytestream::traits::impl$2::get_slice
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/traits.rs
//     evidence={"module":4756,"record_offset":26736,"parent_record_offset":26700,"ancestor_record_offset":26472,"inlinee":10816,"source_line":110,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] zune_jpeg::headers::parse_dqt::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-jpeg","version":"0.4.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-jpeg-0.4.11/src/headers.rs
//     evidence={"module":4756,"record_offset":27984,"parent_record_offset":27952,"ancestor_record_offset":26472,"inlinee":10853,"source_line":147,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: zune_jpeg::headers::parse_dqt<ref$<slice2$<u8> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4756,"record_offset":24044,"rva":42473216,"va":5411182336}
// 编译实例 RVAs: 0x2881700
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t664060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411182336,"raw_hex":"e8fc210a00","size":5,"soft_fail":false,"target_va":5411847168,"va":5411183103,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-28343843","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411182336,"raw_hex":"e8dd814ffe","size":5,"soft_fail":false,"target_va":5382840208,"va":5411184046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t357144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411182336,"raw_hex":"e818730500","size":5,"soft_fail":false,"target_va":5411541776,"va":5411184627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t357676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411182336,"raw_hex":"e82c750500","size":5,"soft_fail":false,"target_va":5411542296,"va":5411184615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t358501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411182336,"raw_hex":"e865780500","size":5,"soft_fail":false,"target_va":5411543104,"va":5411184598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":4756,"record_offset":26188,"parent_record_offset":24044,"ancestor_record_offset":24044,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":4756,"record_offset":24776,"parent_record_offset":24044,"ancestor_record_offset":24044,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$7::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4756,"record_offset":24576,"parent_record_offset":24044,"ancestor_record_offset":24044,"inlinee":10849,"source_line":653,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4756,"record_offset":24604,"parent_record_offset":24044,"ancestor_record_offset":24044,"inlinee":9461,"source_line":1252,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4756,"record_offset":25424,"parent_record_offset":24044,"ancestor_record_offset":24044,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4756,"record_offset":24548,"parent_record_offset":24044,"ancestor_record_offset":24044,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] zune_core::bytestream::reader::ZByteReader::get_u16_be_err
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4756,"record_offset":24216,"parent_record_offset":24044,"ancestor_record_offset":24044,"inlinee":10823,"source_line":378,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] zune_core::bytestream::reader::ZByteReader::get_u8_err
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4756,"record_offset":24660,"parent_record_offset":24044,"ancestor_record_offset":24044,"inlinee":10825,"source_line":439,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] zune_core::bytestream::reader::ZByteReader::read_exact
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4756,"record_offset":24976,"parent_record_offset":24044,"ancestor_record_offset":24044,"inlinee":10826,"source_line":306,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] zune_jpeg::headers::un_zig_zag
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"zune-jpeg","version":"0.4.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-jpeg-0.4.11/src/headers.rs
//     evidence={"module":4756,"record_offset":25584,"parent_record_offset":24044,"ancestor_record_offset":24044,"inlinee":10851,"source_line":532,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":4756,"record_offset":25480,"parent_record_offset":25456,"ancestor_record_offset":24044,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":4756,"record_offset":25536,"parent_record_offset":25508,"ancestor_record_offset":24044,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4756,"record_offset":25080,"parent_record_offset":25056,"ancestor_record_offset":24044,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4756,"record_offset":24940,"parent_record_offset":24824,"ancestor_record_offset":24044,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4756,"record_offset":25056,"parent_record_offset":25016,"ancestor_record_offset":24044,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":4756,"record_offset":24424,"parent_record_offset":24400,"ancestor_record_offset":24044,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":4756,"record_offset":24876,"parent_record_offset":24824,"ancestor_record_offset":24044,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":4756,"record_offset":24824,"parent_record_offset":24776,"ancestor_record_offset":24044,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":4756,"record_offset":25620,"parent_record_offset":25584,"ancestor_record_offset":24044,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4756,"record_offset":24360,"parent_record_offset":24336,"ancestor_record_offset":24044,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4756,"record_offset":24904,"parent_record_offset":24876,"ancestor_record_offset":24044,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::from_be
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4756,"record_offset":24480,"parent_record_offset":24456,"ancestor_record_offset":24044,"inlinee":10808,"source_line":355,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::from_be_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4756,"record_offset":24456,"parent_record_offset":24244,"ancestor_record_offset":24044,"inlinee":10807,"source_line":3510,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::swap_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4756,"record_offset":24504,"parent_record_offset":24480,"ancestor_record_offset":24044,"inlinee":10809,"source_line":306,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4756,"record_offset":25508,"parent_record_offset":25480,"ancestor_record_offset":24044,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4756,"record_offset":24400,"parent_record_offset":24244,"ancestor_record_offset":24044,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4756,"record_offset":24312,"parent_record_offset":24280,"ancestor_record_offset":24044,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4756,"record_offset":25112,"parent_record_offset":25016,"ancestor_record_offset":24044,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4756,"record_offset":24736,"parent_record_offset":24712,"ancestor_record_offset":24044,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4756,"record_offset":24336,"parent_record_offset":24312,"ancestor_record_offset":24044,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4756,"record_offset":25164,"parent_record_offset":25112,"ancestor_record_offset":24044,"inlinee":5308,"source_line":444,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] zune_core::bytestream::reader::ZByteReader::get_u16_inner_or_die
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4756,"record_offset":24244,"parent_record_offset":24216,"ancestor_record_offset":24044,"inlinee":10824,"source_line":353,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] zune_core::bytestream::reader::ZByteReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4756,"record_offset":25016,"parent_record_offset":24976,"ancestor_record_offset":24044,"inlinee":10827,"source_line":292,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] zune_core::bytestream::reader::ZByteReader::skip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4756,"record_offset":25388,"parent_record_offset":25016,"ancestor_record_offset":24044,"inlinee":10814,"source_line":96,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] zune_core::bytestream::traits::impl$0::get_byte
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/traits.rs
//     evidence={"module":4756,"record_offset":24688,"parent_record_offset":24660,"ancestor_record_offset":24044,"inlinee":10818,"source_line":71,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] zune_core::bytestream::traits::impl$0::get_slice
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/traits.rs
//     evidence={"module":4756,"record_offset":24280,"parent_record_offset":24244,"ancestor_record_offset":24044,"inlinee":10806,"source_line":76,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] zune_jpeg::headers::parse_dqt::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-jpeg","version":"0.4.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-jpeg-0.4.11/src/headers.rs
//     evidence={"module":4756,"record_offset":25456,"parent_record_offset":25424,"ancestor_record_offset":24044,"inlinee":10853,"source_line":147,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: zune_jpeg::headers::parse_start_of_frame<ref$<alloc::vec::Vec<u8,alloc::alloc::Global> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4756,"record_offset":20564,"rva":42471104,"va":5411180224}
// 编译实例 RVAs: 0x2880ec0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140601910
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-36174943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411180224,"raw_hex":"e8a103d8fd","size":5,"soft_fail":false,"target_va":5375006992,"va":5411181930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-36098809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411180224,"raw_hex":"e8072dd9fd","size":5,"soft_fail":false,"target_va":5375081488,"va":5411180292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t665550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411180224,"raw_hex":"e8ce270a00","size":5,"soft_fail":false,"target_va":5411847168,"va":5411181613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-28340508","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411180224,"raw_hex":"e8e48e4ffe","size":5,"soft_fail":false,"target_va":5382840208,"va":5411180711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner::with_capacity_in<alloc::alloc::Global>
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-11269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411180224,"raw_hex":"e8fbd3ffff","size":5,"soft_fail":false,"target_va":5411170080,"va":5411181344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t358906","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411180224,"raw_hex":"e8fa790500","size":5,"soft_fail":false,"target_va":5411540963,"va":5411182052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::slice::hack::impl$1::to_vec<u8,alloc::alloc::Global>
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-11863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411180224,"raw_hex":"e8a9d1ffff","size":5,"soft_fail":false,"target_va":5411169824,"va":5411181682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec::push<zune_jpeg::components::Components,alloc::alloc::Global>
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-11726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411180224,"raw_hex":"e832d2ffff","size":5,"soft_fail":false,"target_va":5411169936,"va":5411181657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zune_core::bytestream::reader::ZByteReader$LT$T$GT$::read_exact
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-11916","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411180224,"raw_hex":"e874d1ffff","size":5,"soft_fail":false,"target_va":5411169536,"va":5411181447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zune_jpeg::components::Components::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zune-jpeg","version":"0.4.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-11335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411180224,"raw_hex":"e8b9d3ffff","size":5,"soft_fail":false,"target_va":5411170208,"va":5411181538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zune_jpeg::headers::parse_start_of_frame::closure$0<ref$<alloc::vec::Vec<u8,alloc::alloc::Global> > >
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zune-jpeg","version":"0.4.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411180224,"raw_hex":"e8f1020000","size":5,"soft_fail":false,"target_va":5411182224,"va":5411181466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":4756,"record_offset":22596,"parent_record_offset":20564,"ancestor_record_offset":20564,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4756,"record_offset":20800,"parent_record_offset":20564,"ancestor_record_offset":20564,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4756,"record_offset":23112,"parent_record_offset":20564,"ancestor_record_offset":20564,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] core::convert::num::impl$96::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\num.rs
//     evidence={"module":4756,"record_offset":22848,"parent_record_offset":20564,"ancestor_record_offset":20564,"inlinee":10844,"source_line":77,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":4756,"record_offset":23196,"parent_record_offset":20564,"ancestor_record_offset":20564,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4756,"record_offset":23292,"parent_record_offset":20564,"ancestor_record_offset":20564,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4756,"record_offset":23324,"parent_record_offset":20564,"ancestor_record_offset":20564,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4756,"record_offset":22568,"parent_record_offset":20564,"ancestor_record_offset":20564,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] zune_core::bytestream::reader::ZByteReader::get_u16_be_err
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4756,"record_offset":21368,"parent_record_offset":20564,"ancestor_record_offset":20564,"inlinee":10823,"source_line":378,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] zune_core::bytestream::reader::ZByteReader::get_u8_err
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4756,"record_offset":21744,"parent_record_offset":20564,"ancestor_record_offset":20564,"inlinee":10825,"source_line":439,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] zune_core::options::decoder::DecoderOptions::get_max_height
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/options/decoder.rs
//     evidence={"module":4756,"record_offset":22884,"parent_record_offset":20564,"ancestor_record_offset":20564,"inlinee":10845,"source_line":222,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] zune_core::options::decoder::DecoderOptions::get_max_width
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/options/decoder.rs
//     evidence={"module":4756,"record_offset":22708,"parent_record_offset":20564,"ancestor_record_offset":20564,"inlinee":10842,"source_line":216,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] zune_jpeg::decoder::ImageInfo::set_density
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-jpeg","version":"0.4.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-jpeg-0.4.11/src/decoder.rs
//     evidence={"module":4756,"record_offset":21860,"parent_record_offset":20564,"ancestor_record_offset":20564,"inlinee":10839,"source_line":878,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] zune_jpeg::decoder::ImageInfo::set_height
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-jpeg","version":"0.4.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-jpeg-0.4.11/src/decoder.rs
//     evidence={"module":4756,"record_offset":22208,"parent_record_offset":20564,"ancestor_record_offset":20564,"inlinee":10840,"source_line":870,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] zune_jpeg::decoder::ImageInfo::set_sof_marker
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-jpeg","version":"0.4.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-jpeg-0.4.11/src/decoder.rs
//     evidence={"module":4756,"record_offset":23700,"parent_record_offset":20564,"ancestor_record_offset":20564,"inlinee":10848,"source_line":886,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] zune_jpeg::decoder::ImageInfo::set_width
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-jpeg","version":"0.4.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-jpeg-0.4.11/src/decoder.rs
//     evidence={"module":4756,"record_offset":22680,"parent_record_offset":20564,"ancestor_record_offset":20564,"inlinee":10841,"source_line":862,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4756,"record_offset":21200,"parent_record_offset":21176,"ancestor_record_offset":20564,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4756,"record_offset":21224,"parent_record_offset":21200,"ancestor_record_offset":20564,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4756,"record_offset":21176,"parent_record_offset":21148,"ancestor_record_offset":20564,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":4756,"record_offset":22644,"parent_record_offset":22620,"ancestor_record_offset":20564,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4756,"record_offset":21088,"parent_record_offset":21060,"ancestor_record_offset":20564,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4756,"record_offset":21148,"parent_record_offset":21116,"ancestor_record_offset":20564,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4756,"record_offset":21116,"parent_record_offset":21088,"ancestor_record_offset":20564,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4756,"record_offset":21024,"parent_record_offset":20996,"ancestor_record_offset":20564,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4756,"record_offset":20996,"parent_record_offset":20968,"ancestor_record_offset":20564,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4756,"record_offset":20940,"parent_record_offset":20912,"ancestor_record_offset":20564,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4756,"record_offset":20968,"parent_record_offset":20940,"ancestor_record_offset":20564,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4756,"record_offset":20912,"parent_record_offset":20884,"ancestor_record_offset":20564,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":4756,"record_offset":20884,"parent_record_offset":20856,"ancestor_record_offset":20564,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$110::spec_to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4756,"record_offset":20828,"parent_record_offset":20800,"ancestor_record_offset":20564,"inlinee":10837,"source_line":2877,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4756,"record_offset":20856,"parent_record_offset":20828,"ancestor_record_offset":20564,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4756,"record_offset":21560,"parent_record_offset":21456,"ancestor_record_offset":20564,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4756,"record_offset":21060,"parent_record_offset":21024,"ancestor_record_offset":20564,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$60::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4756,"record_offset":23252,"parent_record_offset":23224,"ancestor_record_offset":20564,"inlinee":10847,"source_line":1824,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":4756,"record_offset":21300,"parent_record_offset":21276,"ancestor_record_offset":20564,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":4756,"record_offset":23224,"parent_record_offset":23196,"ancestor_record_offset":20564,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4756,"record_offset":21528,"parent_record_offset":21500,"ancestor_record_offset":20564,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::from_be
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4756,"record_offset":21676,"parent_record_offset":21652,"ancestor_record_offset":20564,"inlinee":10808,"source_line":355,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::from_be_bytes
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4756,"record_offset":21652,"parent_record_offset":21392,"ancestor_record_offset":20564,"inlinee":10807,"source_line":3510,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::swap_bytes
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4756,"record_offset":21700,"parent_record_offset":21676,"ancestor_record_offset":20564,"inlinee":10809,"source_line":306,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4756,"record_offset":22620,"parent_record_offset":22596,"ancestor_record_offset":20564,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":4756,"record_offset":21276,"parent_record_offset":21024,"ancestor_record_offset":20564,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4756,"record_offset":21596,"parent_record_offset":21392,"ancestor_record_offset":20564,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4756,"record_offset":21456,"parent_record_offset":21428,"ancestor_record_offset":20564,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4756,"record_offset":21820,"parent_record_offset":21796,"ancestor_record_offset":20564,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4756,"record_offset":21500,"parent_record_offset":21456,"ancestor_record_offset":20564,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] zune_core::bytestream::reader::ZByteReader::get_u16_inner_or_die
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4756,"record_offset":21392,"parent_record_offset":21368,"ancestor_record_offset":20564,"inlinee":10824,"source_line":353,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] zune_core::bytestream::traits::impl$2::get_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/traits.rs
//     evidence={"module":4756,"record_offset":21772,"parent_record_offset":21744,"ancestor_record_offset":20564,"inlinee":10819,"source_line":105,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] zune_core::bytestream::traits::impl$2::get_slice
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/traits.rs
//     evidence={"module":4756,"record_offset":21428,"parent_record_offset":21392,"ancestor_record_offset":20564,"inlinee":10816,"source_line":110,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: zune_jpeg::headers::parse_start_of_frame<ref$<slice2$<u8> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 1
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":4756,"record_offset":17452,"rva":42469104,"va":5411178224}
// 编译实例 RVAs: 0x28806f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140601910
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-36172945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411178224,"raw_hex":"e86f0bd8fd","size":5,"soft_fail":false,"target_va":5375006992,"va":5411179932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-36096809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411178224,"raw_hex":"e8d734d9fd","size":5,"soft_fail":false,"target_va":5375081488,"va":5411178292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t667548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411178224,"raw_hex":"e89c2f0a00","size":5,"soft_fail":false,"target_va":5411847168,"va":5411179615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-28338510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411178224,"raw_hex":"e8b2964ffe","size":5,"soft_fail":false,"target_va":5382840208,"va":5411178713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner::with_capacity_in<alloc::alloc::Global>
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9271","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411178224,"raw_hex":"e8c9dbffff","size":5,"soft_fail":false,"target_va":5411170080,"va":5411179346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t360904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411178224,"raw_hex":"e8c8810500","size":5,"soft_fail":false,"target_va":5411540963,"va":5411180054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::slice::hack::impl$1::to_vec<u8,alloc::alloc::Global>
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411178224,"raw_hex":"e877d9ffff","size":5,"soft_fail":false,"target_va":5411169824,"va":5411179684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec::push<zune_jpeg::components::Components,alloc::alloc::Global>
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411178224,"raw_hex":"e800daffff","size":5,"soft_fail":false,"target_va":5411169936,"va":5411179659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zune_core::bytestream::reader::ZByteReader$LT$T$GT$::read_exact
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-9774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411178224,"raw_hex":"e8d2d9ffff","size":5,"soft_fail":false,"target_va":5411169680,"va":5411179449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zune_jpeg::components::Components::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zune-jpeg","version":"0.4.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-9337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411178224,"raw_hex":"e887dbffff","size":5,"soft_fail":false,"target_va":5411170208,"va":5411179540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] zune_jpeg::headers::parse_start_of_frame::closure$0<ref$<alloc::vec::Vec<u8,alloc::alloc::Global> > >
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"zune-jpeg","version":"0.4.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411178224,"raw_hex":"e8bf0a0000","size":5,"soft_fail":false,"target_va":5411182224,"va":5411179468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":4756,"record_offset":19400,"parent_record_offset":17452,"ancestor_record_offset":17452,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4756,"record_offset":17660,"parent_record_offset":17452,"ancestor_record_offset":17452,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4756,"record_offset":19916,"parent_record_offset":17452,"ancestor_record_offset":17452,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] core::convert::num::impl$96::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\num.rs
//     evidence={"module":4756,"record_offset":19652,"parent_record_offset":17452,"ancestor_record_offset":17452,"inlinee":10844,"source_line":77,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":4756,"record_offset":20000,"parent_record_offset":17452,"ancestor_record_offset":17452,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4756,"record_offset":20096,"parent_record_offset":17452,"ancestor_record_offset":17452,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4756,"record_offset":20128,"parent_record_offset":17452,"ancestor_record_offset":17452,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4756,"record_offset":19372,"parent_record_offset":17452,"ancestor_record_offset":17452,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] zune_core::bytestream::reader::ZByteReader::get_u16_be_err
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4756,"record_offset":18228,"parent_record_offset":17452,"ancestor_record_offset":17452,"inlinee":10823,"source_line":378,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] zune_core::bytestream::reader::ZByteReader::get_u8_err
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4756,"record_offset":18548,"parent_record_offset":17452,"ancestor_record_offset":17452,"inlinee":10825,"source_line":439,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] zune_core::options::decoder::DecoderOptions::get_max_height
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/options/decoder.rs
//     evidence={"module":4756,"record_offset":19688,"parent_record_offset":17452,"ancestor_record_offset":17452,"inlinee":10845,"source_line":222,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] zune_core::options::decoder::DecoderOptions::get_max_width
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/options/decoder.rs
//     evidence={"module":4756,"record_offset":19512,"parent_record_offset":17452,"ancestor_record_offset":17452,"inlinee":10842,"source_line":216,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] zune_jpeg::decoder::ImageInfo::set_density
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-jpeg","version":"0.4.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-jpeg-0.4.11/src/decoder.rs
//     evidence={"module":4756,"record_offset":18664,"parent_record_offset":17452,"ancestor_record_offset":17452,"inlinee":10839,"source_line":878,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] zune_jpeg::decoder::ImageInfo::set_height
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-jpeg","version":"0.4.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-jpeg-0.4.11/src/decoder.rs
//     evidence={"module":4756,"record_offset":19012,"parent_record_offset":17452,"ancestor_record_offset":17452,"inlinee":10840,"source_line":870,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] zune_jpeg::decoder::ImageInfo::set_sof_marker
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-jpeg","version":"0.4.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-jpeg-0.4.11/src/decoder.rs
//     evidence={"module":4756,"record_offset":20504,"parent_record_offset":17452,"ancestor_record_offset":17452,"inlinee":10848,"source_line":886,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_direct] zune_jpeg::decoder::ImageInfo::set_width
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"zune-jpeg","version":"0.4.11","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-jpeg-0.4.11/src/decoder.rs
//     evidence={"module":4756,"record_offset":19484,"parent_record_offset":17452,"ancestor_record_offset":17452,"inlinee":10841,"source_line":862,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4756,"record_offset":18060,"parent_record_offset":18036,"ancestor_record_offset":17452,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4756,"record_offset":18084,"parent_record_offset":18060,"ancestor_record_offset":17452,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4756,"record_offset":18036,"parent_record_offset":18008,"ancestor_record_offset":17452,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":4756,"record_offset":19448,"parent_record_offset":19424,"ancestor_record_offset":17452,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4756,"record_offset":17948,"parent_record_offset":17920,"ancestor_record_offset":17452,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4756,"record_offset":18008,"parent_record_offset":17976,"ancestor_record_offset":17452,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":4756,"record_offset":17976,"parent_record_offset":17948,"ancestor_record_offset":17452,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4756,"record_offset":17884,"parent_record_offset":17856,"ancestor_record_offset":17452,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4756,"record_offset":17856,"parent_record_offset":17828,"ancestor_record_offset":17452,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4756,"record_offset":17800,"parent_record_offset":17772,"ancestor_record_offset":17452,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4756,"record_offset":17828,"parent_record_offset":17800,"ancestor_record_offset":17452,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":4756,"record_offset":17772,"parent_record_offset":17744,"ancestor_record_offset":17452,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":4756,"record_offset":17744,"parent_record_offset":17716,"ancestor_record_offset":17452,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$110::spec_to_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4756,"record_offset":17688,"parent_record_offset":17660,"ancestor_record_offset":17452,"inlinee":10837,"source_line":2877,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":4756,"record_offset":17716,"parent_record_offset":17688,"ancestor_record_offset":17452,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":4756,"record_offset":17920,"parent_record_offset":17884,"ancestor_record_offset":17452,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$60::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":4756,"record_offset":20056,"parent_record_offset":20028,"ancestor_record_offset":17452,"inlinee":10847,"source_line":1824,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":4756,"record_offset":18160,"parent_record_offset":18136,"ancestor_record_offset":17452,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":4756,"record_offset":20028,"parent_record_offset":20000,"ancestor_record_offset":17452,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4756,"record_offset":18360,"parent_record_offset":18336,"ancestor_record_offset":17452,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::from_be
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4756,"record_offset":18480,"parent_record_offset":18456,"ancestor_record_offset":17452,"inlinee":10808,"source_line":355,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::from_be_bytes
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4756,"record_offset":18456,"parent_record_offset":18252,"ancestor_record_offset":17452,"inlinee":10807,"source_line":3510,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::swap_bytes
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":4756,"record_offset":18504,"parent_record_offset":18480,"ancestor_record_offset":17452,"inlinee":10809,"source_line":306,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4756,"record_offset":19424,"parent_record_offset":19400,"ancestor_record_offset":17452,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":4756,"record_offset":18136,"parent_record_offset":17884,"ancestor_record_offset":17452,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4756,"record_offset":18400,"parent_record_offset":18252,"ancestor_record_offset":17452,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":4756,"record_offset":18312,"parent_record_offset":18288,"ancestor_record_offset":17452,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4756,"record_offset":18624,"parent_record_offset":18600,"ancestor_record_offset":17452,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":4756,"record_offset":18336,"parent_record_offset":18312,"ancestor_record_offset":17452,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] zune_core::bytestream::reader::ZByteReader::get_u16_inner_or_die
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/reader.rs
//     evidence={"module":4756,"record_offset":18252,"parent_record_offset":18228,"ancestor_record_offset":17452,"inlinee":10824,"source_line":353,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] zune_core::bytestream::traits::impl$0::get_byte
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/traits.rs
//     evidence={"module":4756,"record_offset":18576,"parent_record_offset":18548,"ancestor_record_offset":17452,"inlinee":10818,"source_line":71,"archive":"_pdb/B/modules/04756.jsonl.gz"}
//   [pdb_inline_transitive] zune_core::bytestream::traits::impl$0::get_slice
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"zune-core","version":"0.4.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zune-core-0.4.12/src/bytestream/traits.rs
//     evidence={"module":4756,"record_offset":18288,"parent_record_offset":18252,"ancestor_record_offset":17452,"inlinee":10806,"source_line":76,"archive":"_pdb/B/modules/04756.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: ddsfile::header::Header::read<ref_mut$<ref_mut$<std::io::cursor::Cursor<alloc::vec::Vec<u8,alloc::alloc::Global> > > > >
// 信息 ID: 5852c30bf0b59d9a5a1c1623
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] std::io::cursor::Cursor -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["std::io::cursor::Cursor<alloc::vec::Vec<u8,alloc::alloc::Global> >"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["ref_mut$<std::io::cursor::Cursor<alloc::vec::Vec<u8,alloc::alloc::Global> > >"]; 字段布局 = unknown。
// [类型关系] ddsfile::header::Header::read -> ["ref_mut$<ref_mut$<std::io::cursor::Cursor<alloc::vec::Vec<u8,alloc::alloc::Global> > > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ddsfile::Dds::read<ref_mut$<std::io::cursor::Cursor<alloc::vec::Vec<u8,alloc::alloc::Global> > > > @ 0x20e109a -> 0x245c890 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: image::codecs::png::PngDecoder::with_limits<std::io::buffered::bufreader::BufReader<std::fs::File> >
// 信息 ID: 57936c9f1d52c91ddbbb141d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] std::io::buffered::bufreader::BufReader -> ["std::fs::File"]; 字段布局 = unknown。
// [类型关系] image::codecs::png::PngDecoder::with_limits -> ["std::io::buffered::bufreader::BufReader<std::fs::File>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] image::io::reader::Reader::decode<std::io::buffered::bufreader::BufReader<std::fs::File> > @ 0x20e23e0 -> 0x245e120 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: image::dynimage::decoder_to_image<alloc::boxed::Box<dyn$<image::image::ImageDecoder>,alloc::alloc::Global> >
// 信息 ID: 29359991fd4bb88dac6a9e85
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] dyn$ -> ["image::image::ImageDecoder"]; 字段布局 = unknown。
// [类型关系] alloc::boxed::Box -> ["dyn$<image::image::ImageDecoder>", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] image::dynimage::decoder_to_image -> ["alloc::boxed::Box<dyn$<image::image::ImageDecoder>,alloc::alloc::Global>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] image::io::reader::Reader::decode<std::io::buffered::bufreader::BufReader<std::fs::File> > @ 0x20e27a3 -> 0x245ef80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: png::decoder::ReadDecoder::decode_next<std::io::buffered::bufreader::BufReader<std::fs::File> >
// 信息 ID: d932c1cd724a18390b4a398e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] std::io::buffered::bufreader::BufReader -> ["std::fs::File"]; 字段布局 = unknown。
// [类型关系] png::decoder::ReadDecoder::decode_next -> ["std::io::buffered::bufreader::BufReader<std::fs::File>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 14 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] png::decoder::Reader::read_until_image_data<std::io::buffered::bufreader::BufReader<std::fs::File> > @ 0x2fd11 -> 0x30ff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] png::decoder::Reader::next_raw_interlaced_row<std::io::buffered::bufreader::BufReader<std::fs::File> > @ 0x302cc -> 0x30ff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] png::decoder::Decoder::read_header_info<std::io::buffered::bufreader::BufReader<std::fs::File> > | png::decoder::Decoder::read_header_info<std::io::buffered::bufreader::BufReader<std::fs::File> > | png::decoder::Decoder::read_header_info<std::io::buffered::bufreader::BufReader<std::fs::File> > @ 0x30ae1 -> 0x30ff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] png::decoder::Decoder::read_info<std::io::buffered::bufreader::BufReader<std::fs::File> > @ 0x30c67 -> 0x30ff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] png::decoder::Reader::read_until_image_data<std::io::buffered::bufreader::BufReader<std::fs::File> > @ 0x172ce81 -> 0x30ff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] png::decoder::Reader::next_raw_interlaced_row<std::io::buffered::bufreader::BufReader<std::fs::File> > @ 0x172d43c -> 0x30ff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] image::codecs::png::PngDecoder::with_limits<std::io::buffered::bufreader::BufReader<std::fs::File> > @ 0x172de6a -> 0x30ff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] image::codecs::png::PngDecoder::with_limits<std::io::buffered::bufreader::BufReader<std::fs::File> > @ 0x172e08a -> 0x30ff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] png::decoder::Reader::read_until_image_data<std::io::buffered::bufreader::BufReader<std::fs::File> > @ 0x23784c1 -> 0x30ff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] png::decoder::Reader::next_raw_interlaced_row<std::io::buffered::bufreader::BufReader<std::fs::File> > @ 0x2378a7c -> 0x30ff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] png::decoder::Decoder::read_info<std::io::buffered::bufreader::BufReader<std::fs::File> > @ 0x2379297 -> 0x30ff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] png::decoder::Reader::read_until_image_data<std::io::buffered::bufreader::BufReader<std::fs::File> > @ 0x2698df1 -> 0x30ff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] png::decoder::Reader::next_raw_interlaced_row<std::io::buffered::bufreader::BufReader<std::fs::File> > @ 0x26993ac -> 0x30ff0 (direct_call); 同地址别名未裁定。
// [静态直接引用] png::decoder::Decoder::read_info<std::io::buffered::bufreader::BufReader<std::fs::File> > @ 0x2699bc7 -> 0x30ff0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: png::decoder::Reader::next_frame<std::io::buffered::bufreader::BufReader<std::fs::File> >
// 信息 ID: dc00daf0beb41a3e88e5be9a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] std::io::buffered::bufreader::BufReader -> ["std::fs::File"]; 字段布局 = unknown。
// [类型关系] png::decoder::Reader::next_frame -> ["std::io::buffered::bufreader::BufReader<std::fs::File>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] image::codecs::png::impl$1::read_image_boxed<std::io::buffered::bufreader::BufReader<std::fs::File> > @ 0x245ebb7 -> 0x26983f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: png::decoder::Reader::next_interlaced_row_impl<std::io::buffered::bufreader::BufReader<std::fs::File> >
// 信息 ID: 20712ad430ea7a1abd59bfdd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] std::io::buffered::bufreader::BufReader -> ["std::fs::File"]; 字段布局 = unknown。
// [类型关系] png::decoder::Reader::next_interlaced_row_impl -> ["std::io::buffered::bufreader::BufReader<std::fs::File>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] png::decoder::Reader::next_frame<std::io::buffered::bufreader::BufReader<std::fs::File> > @ 0x269855b -> 0x2699640 (direct_call); 同地址别名未裁定。
// [静态直接引用] png::decoder::Reader::next_frame<std::io::buffered::bufreader::BufReader<std::fs::File> > @ 0x26985d9 -> 0x2699640 (direct_call); 同地址别名未裁定。
// [静态直接引用] png::decoder::Reader::next_frame<std::io::buffered::bufreader::BufReader<std::fs::File> > @ 0x2698616 -> 0x2699640 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: png::decoder::Reader::read_until_image_data<std::io::buffered::bufreader::BufReader<std::fs::File> >
// 信息 ID: 5aa90828c281231f7990d0c2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] std::io::buffered::bufreader::BufReader -> ["std::fs::File"]; 字段布局 = unknown。
// [类型关系] png::decoder::Reader::read_until_image_data -> ["std::io::buffered::bufreader::BufReader<std::fs::File>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] png::decoder::Reader::next_frame<std::io::buffered::bufreader::BufReader<std::fs::File> > @ 0x2698426 -> 0x2698c80 (direct_call); 同地址别名未裁定。
// [静态直接引用] png::decoder::Decoder::read_info<std::io::buffered::bufreader::BufReader<std::fs::File> > @ 0x2699d41 -> 0x2698c80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::Deserializer::handle_struct_after_name<serde::de::ignored_any::IgnoredAny>
// 信息 ID: 296a0959628243f3b75a81b2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::de::Deserializer::handle_struct_after_name -> ["serde::de::ignored_any::IgnoredAny"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_any<serde::de::ignored_any::IgnoredAny> @ 0x24471ff -> 0x2445640 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::impl$2::deserialize_any<serde::de::ignored_any::IgnoredAny>
// 信息 ID: 3282683fbe2ad277464e785e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::de::impl$2::deserialize_any -> ["serde::de::ignored_any::IgnoredAny"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::Deserializer::handle_any_struct<serde::de::ignored_any::IgnoredAny> @ 0x24454db -> 0x2447170 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$6::next_key_seed<core::marker::PhantomData<serde::de::ignored_any::IgnoredAny> > @ 0x24465df -> 0x2447170 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$6::next_value_seed<core::marker::PhantomData<serde::de::ignored_any::IgnoredAny> > @ 0x2446679 -> 0x2447170 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$2::deserialize_any<serde::de::ignored_any::IgnoredAny> @ 0x2447354 -> 0x2447170 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$2::deserialize_any<serde::de::ignored_any::IgnoredAny> @ 0x24474fb -> 0x2447170 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$2::deserialize_seq<serde::de::ignored_any::IgnoredAny> @ 0x24497fa -> 0x2447170 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$2::deserialize_tuple<serde::de::ignored_any::IgnoredAny> @ 0x244a67a -> 0x2447170 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::impl$2::deserialize_byte_buf<serde::de::ignored_any::IgnoredAny>
// 信息 ID: 62058663b0563402c5007028
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::de::impl$2::deserialize_byte_buf -> ["serde::de::ignored_any::IgnoredAny"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::impl$2::deserialize_option<serde::de::impls::OptionVisitor<array$<u32,2> > >
// 信息 ID: 743fbff1ffbd82ce5eeba2cb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] array$ -> ["u32", "2"]; 字段布局 = unknown。
// [类型关系] serde::de::impls::OptionVisitor -> ["array$<u32,2>"]; 字段布局 = unknown。
// [类型关系] ron::de::impl$2::deserialize_option -> ["serde::de::impls::OptionVisitor<array$<u32,2> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_struct<asset::texture::desc::_::impl$0::deserialize::__Visitor> @ 0x244f73b -> 0x244a860 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::impl$2::deserialize_seq<serde::de::ignored_any::IgnoredAny>
// 信息 ID: 3759e9c7f2df70aaeaeecf06
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::de::impl$2::deserialize_seq -> ["serde::de::ignored_any::IgnoredAny"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::impl$2::deserialize_struct<asset::texture::desc::_::impl$0::deserialize::__Visitor>
// 信息 ID: 879c4a842cf3e7ec279a6e9b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::de::impl$2::deserialize_struct -> ["asset::texture::desc::_::impl$0::deserialize::__Visitor"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_struct<asset::texture::desc::_::impl$0::deserialize::__Visitor> @ 0x244db8a -> 0x244f1a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::impl$2::deserialize_struct<asset_pipe::texture::_::impl$0::deserialize::__Visitor>
// 信息 ID: 9be52c16a23cc12af0fe0a0f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::de::impl$2::deserialize_struct -> ["asset_pipe::texture::_::impl$0::deserialize::__Visitor"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::options::Options::from_str_seed<core::marker::PhantomData<asset_pipe::texture::TextureAssetLibraryMeta>,asset_pipe::texture::TextureAssetLibraryMeta> @ 0x20d7cde -> 0x244c500 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::impl$2::deserialize_struct<serde::de::impls::impl$36::deserialize::DurationVisitor>
// 信息 ID: 01ba31eff830a217821cf1cf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::de::impl$2::deserialize_struct -> ["serde::de::impls::impl$36::deserialize::DurationVisitor"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_struct<asset_pipe::texture::_::impl$0::deserialize::__Visitor> @ 0x244b1b7 -> 0x244c270 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::impl$2::deserialize_tuple<serde::de::ignored_any::IgnoredAny>
// 信息 ID: 2c93ce1ac2b3a69c852f3d03
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::de::impl$2::deserialize_tuple -> ["serde::de::ignored_any::IgnoredAny"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::Deserializer::handle_any_struct<serde::de::ignored_any::IgnoredAny> @ 0x244556e -> 0x244a5b0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$2::deserialize_any<serde::de::ignored_any::IgnoredAny> @ 0x2447240 -> 0x244a5b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::impl$2::deserialize_tuple<serde::de::impls::ArrayVisitor<array$<u32,2> > >
// 信息 ID: 28ae96c52783b85578f45cbc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] array$ -> ["u32", "2"]; 字段布局 = unknown。
// [类型关系] serde::de::impls::ArrayVisitor -> ["array$<u32,2>"]; 字段布局 = unknown。
// [类型关系] ron::de::impl$2::deserialize_tuple -> ["serde::de::impls::ArrayVisitor<array$<u32,2> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_option<serde::de::impls::OptionVisitor<array$<u32,2> > > @ 0x244a92d -> 0x244a130 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$2::deserialize_option<serde::de::impls::OptionVisitor<array$<u32,2> > > @ 0x244aa02 -> 0x244a130 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::impl$5::next_element_seed<core::marker::PhantomData<relative_path::RelativePathBuf> >
// 信息 ID: 5d8841171e21718805252db7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::marker::PhantomData -> ["relative_path::RelativePathBuf"]; 字段布局 = unknown。
// [类型关系] ron::de::impl$5::next_element_seed -> ["core::marker::PhantomData<relative_path::RelativePathBuf>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::de::impls::impl$19::deserialize::impl$0::visit_seq<relative_path::RelativePathBuf,ron::de::CommaSeparated> @ 0x268d6ab -> 0x2446ec0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::impl$5::next_element_seed<core::marker::PhantomData<u8> >
// 信息 ID: e54237ff034fab4c5f3ab273
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::marker::PhantomData -> ["u8"]; 字段布局 = unknown。
// [类型关系] ron::de::impl$5::next_element_seed -> ["core::marker::PhantomData<u8>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::de::impls::impl$19::deserialize::impl$0::visit_seq<u8,ron::de::CommaSeparated> @ 0x268db25 -> 0x2446d80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::impl$6::next_key_seed<core::marker::PhantomData<serde::de::ignored_any::IgnoredAny> >
// 信息 ID: 62207184257aaaab6e3f0b3a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::marker::PhantomData -> ["serde::de::ignored_any::IgnoredAny"]; 字段布局 = unknown。
// [类型关系] ron::de::impl$6::next_key_seed -> ["core::marker::PhantomData<serde::de::ignored_any::IgnoredAny>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::Deserializer::handle_struct_after_name<serde::de::ignored_any::IgnoredAny> @ 0x24456ae -> 0x2446420 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::Deserializer::handle_struct_after_name<serde::de::ignored_any::IgnoredAny> @ 0x244570c -> 0x2446420 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$2::deserialize_map<serde::de::ignored_any::IgnoredAny> @ 0x244909e -> 0x2446420 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$2::deserialize_map<serde::de::ignored_any::IgnoredAny> @ 0x24490e3 -> 0x2446420 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::impl$6::next_key_seed<core::marker::PhantomData<serde::de::impls::impl$36::deserialize::Field> >
// 信息 ID: 305a8c941fea63bd623d095d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::marker::PhantomData -> ["serde::de::impls::impl$36::deserialize::Field"]; 字段布局 = unknown。
// [类型关系] ron::de::impl$6::next_key_seed -> ["core::marker::PhantomData<serde::de::impls::impl$36::deserialize::Field>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::de::impls::impl$36::deserialize::impl$1::visit_map<ron::de::CommaSeparated> @ 0x268d41e -> 0x2446240 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde::de::impls::impl$36::deserialize::impl$1::visit_map<ron::de::CommaSeparated> @ 0x268d446 -> 0x2446240 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::impl$6::next_value_seed<core::marker::PhantomData<serde::de::ignored_any::IgnoredAny> >
// 信息 ID: 561ad986ecb5d28782abeae0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::marker::PhantomData -> ["serde::de::ignored_any::IgnoredAny"]; 字段布局 = unknown。
// [类型关系] ron::de::impl$6::next_value_seed -> ["core::marker::PhantomData<serde::de::ignored_any::IgnoredAny>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::Deserializer::handle_struct_after_name<serde::de::ignored_any::IgnoredAny> @ 0x24456f4 -> 0x2446600 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$2::deserialize_map<serde::de::ignored_any::IgnoredAny> @ 0x24490cc -> 0x2446600 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$2::deserialize_struct<asset_pipe::texture::_::impl$0::deserialize::__Visitor> @ 0x244b0dc -> 0x2446600 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$2::deserialize_struct<asset_pipe::texture::_::impl$0::deserialize::__Visitor> @ 0x244ca1d -> 0x2446600 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$2::deserialize_struct<asset::texture::desc::_::impl$0::deserialize::__Visitor> @ 0x244d835 -> 0x2446600 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::impl$6::next_value_seed<core::marker::PhantomData<u32> >
// 信息 ID: e6c673aa8887a8473a507879
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::marker::PhantomData -> ["u32"]; 字段布局 = unknown。
// [类型关系] ron::de::impl$6::next_value_seed -> ["core::marker::PhantomData<u32>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::de::impls::impl$36::deserialize::impl$1::visit_map<ron::de::CommaSeparated> @ 0x268d472 -> 0x2446910 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::impl$6::next_value_seed<core::marker::PhantomData<u64> >
// 信息 ID: 975f222fa3ccea23b190e8b2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::marker::PhantomData -> ["u64"]; 字段布局 = unknown。
// [类型关系] ron::de::impl$6::next_value_seed -> ["core::marker::PhantomData<u64>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::de::impls::impl$36::deserialize::impl$1::visit_map<ron::de::CommaSeparated> @ 0x268d4a0 -> 0x2446770 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::impl$8::variant_seed<core::marker::PhantomData<asset::texture::desc::_::impl$0::deserialize::__Field> >
// 信息 ID: 6cc44f945f09b1afaf6a07bc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::marker::PhantomData -> ["asset::texture::desc::_::impl$0::deserialize::__Field"]; 字段布局 = unknown。
// [类型关系] ron::de::impl$8::variant_seed -> ["core::marker::PhantomData<asset::texture::desc::_::impl$0::deserialize::__Field>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_enum<asset::texture::desc::_::impl$0::deserialize::__Visitor> @ 0x2449a33 -> 0x24459e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$2::deserialize_enum<asset::texture::desc::_::impl$0::deserialize::__Visitor> @ 0x2449f43 -> 0x2445bc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$2::deserialize_enum<asset::texture::desc::_::impl$0::deserialize::__Visitor> @ 0x2449d03 -> 0x2445da0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::de::impl$9::newtype_variant_seed<core::marker::PhantomData<u32> >
// 信息 ID: d4f42c76a005dcd18cffd2cc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::marker::PhantomData -> ["u32"]; 字段布局 = unknown。
// [类型关系] ron::de::impl$9::newtype_variant_seed -> ["core::marker::PhantomData<u32>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_enum<asset::texture::desc::_::impl$0::deserialize::__Visitor> @ 0x2449bcd -> 0x2445fa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::options::Options::from_str_seed<core::marker::PhantomData<alloc::vec::Vec<relative_path::RelativePathBuf,alloc::alloc::Global> >,alloc::vec::Vec<relative_path::RelativePathBuf,alloc::alloc::Global> >
// 信息 ID: 994b58ddabf07e7f8b322225
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["relative_path::RelativePathBuf", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] core::marker::PhantomData -> ["alloc::vec::Vec<relative_path::RelativePathBuf,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] alloc::vec::Vec -> ["relative_path::RelativePathBuf", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ron::options::Options::from_str_seed -> ["core::marker::PhantomData<alloc::vec::Vec<relative_path::RelativePathBuf,alloc::alloc::Global> >", "alloc::vec::Vec<relative_path::RelativePathBuf,alloc::alloc::Global>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset_pipe::texture::compile_or_load_texture_assets @ 0x17b73c0 -> 0x20d7f60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::parse::Parser::integer<u32>
// 信息 ID: 2c75fcd2338f0d488278e0b5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::parse::Parser::integer -> ["u32"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$9::newtype_variant_seed<core::marker::PhantomData<u32> > @ 0x244602b -> 0x2690050 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$6::next_value_seed<core::marker::PhantomData<u32> > @ 0x244698a -> 0x2690050 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$2::deserialize_tuple<serde::de::impls::ArrayVisitor<array$<u32,2> > > @ 0x244a1e2 -> 0x2690050 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::de::impl$2::deserialize_tuple<serde::de::impls::ArrayVisitor<array$<u32,2> > > @ 0x244a290 -> 0x2690050 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::parse::Parser::integer<u64>
// 信息 ID: a81898eca4692f0248ff9c12
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::parse::Parser::integer -> ["u64"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$6::next_value_seed<core::marker::PhantomData<u64> > @ 0x24467ed -> 0x268f890 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::parse::Parser::integer<u8>
// 信息 ID: 0782b66770f266a0d21403d9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::parse::Parser::integer -> ["u8"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$5::next_element_seed<core::marker::PhantomData<u8> > @ 0x2446dda -> 0x2690810 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::parse::Parser::parse_integer<i16>
// 信息 ID: 639889e6a7229028496c5682
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::parse::Parser::parse_integer -> ["i16"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::parse::Parser::integer<u64> @ 0x268fb4a -> 0x268dc30 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::parse::Parser::integer<u32> @ 0x269030a -> 0x268dc30 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::parse::Parser::integer<u8> @ 0x2690aca -> 0x268dc30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::parse::Parser::parse_integer<i32>
// 信息 ID: 46cda47c34f8fdc036cc47f5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::parse::Parser::parse_integer -> ["i32"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::parse::Parser::integer<u64> @ 0x268fbbe -> 0x268ea50 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::parse::Parser::integer<u32> @ 0x269037e -> 0x268ea50 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::parse::Parser::integer<u8> @ 0x2690b3e -> 0x268ea50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::parse::Parser::parse_integer<i64>
// 信息 ID: 3bc9e15b7bb00d9b054d3d3b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::parse::Parser::parse_integer -> ["i64"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::parse::Parser::integer<u64> @ 0x268fc2f -> 0x268dfb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::parse::Parser::integer<u32> @ 0x26903ef -> 0x268dfb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::parse::Parser::integer<u8> @ 0x2690baf -> 0x268dfb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::parse::Parser::parse_integer<i8>
// 信息 ID: 2b263b6eaeac4f90ffe80fb0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::parse::Parser::parse_integer -> ["i8"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::parse::Parser::integer<u64> @ 0x268fa94 -> 0x268e6d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::parse::Parser::integer<u32> @ 0x2690254 -> 0x268e6d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::parse::Parser::integer<u8> @ 0x2690a14 -> 0x268e6d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::parse::Parser::parse_integer<u16>
// 信息 ID: da1bb673121ba2fb1cf2d66f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::parse::Parser::parse_integer -> ["u16"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::parse::Parser::integer<u64> @ 0x268fd61 -> 0x268e330 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::parse::Parser::integer<u32> @ 0x2690521 -> 0x268e330 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::parse::Parser::integer<u8> @ 0x2690ce1 -> 0x268e330 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::parse::Parser::parse_integer<u32>
// 信息 ID: 7adca1a0360a86e65ba7d55f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::parse::Parser::parse_integer -> ["u32"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::parse::Parser::integer<u64> @ 0x268fe1f -> 0x268f500 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::parse::Parser::integer<u32> @ 0x26905df -> 0x268f500 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::parse::Parser::integer<u8> @ 0x2690d9f -> 0x268f500 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::parse::Parser::parse_integer<u64>
// 信息 ID: b63c462ae8755ec57f384c42
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::parse::Parser::parse_integer -> ["u64"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::parse::Parser::integer<u64> @ 0x268fea2 -> 0x268edd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::parse::Parser::integer<u32> @ 0x2690662 -> 0x268edd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::parse::Parser::integer<u8> @ 0x2690e22 -> 0x268edd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::parse::Parser::parse_integer<u8>
// 信息 ID: 51c7d7f3cf76e39e232cc666
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ron::parse::Parser::parse_integer -> ["u8"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::parse::Parser::integer<u64> @ 0x268fca3 -> 0x268f170 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::parse::Parser::integer<u32> @ 0x2690463 -> 0x268f170 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::parse::Parser::integer<u8> @ 0x2690c23 -> 0x268f170 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::ser::Serializer::write_identifier<ref_mut$<alloc::string::String> >
// 信息 ID: 27e08818cd141abfb057bc22
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["alloc::string::String"]; 字段布局 = unknown。
// [类型关系] ron::ser::Serializer::write_identifier -> ["ref_mut$<alloc::string::String>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 20 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset::texture::desc::_::impl$0::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > > @ 0x20e0284 -> 0x2450ed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::texture::desc::_::impl$0::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > > @ 0x20e02c5 -> 0x2450ed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::texture::desc::_::impl$0::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > > @ 0x20e0388 -> 0x2450ed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::texture::desc::_::impl$0::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > > @ 0x20e087f -> 0x2450ed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::texture::desc::_::impl$0::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > > @ 0x20e08dd -> 0x2450ed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset::texture::desc::_::impl$0::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > > @ 0x20e08ae -> 0x2450ed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,asset::texture::desc::Gamma> @ 0x2451c84 -> 0x2450ed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,asset::texture::desc::CompressionMethod> @ 0x2451f64 -> 0x2450ed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,std::collections::hash::map::HashMap<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta,std::hash::random::RandomState> > @ 0x2452280 -> 0x2450ed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,asset::texture::desc::TextureAssetDesc> @ 0x2452da4 -> 0x2450ed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,enum2$<asset::texture::desc::MipChain> > @ 0x2453084 -> 0x2450ed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,alloc::vec::Vec<asset::texture::desc::TextureAssetDesc,alloc::alloc::Global> > @ 0x2453388 -> 0x2450ed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,asset::texture::desc::TextureOpts> @ 0x2453ba4 -> 0x2450ed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,std::path::PathBuf> @ 0x2453e84 -> 0x2450ed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,u32> @ 0x24541a4 -> 0x2450ed0 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,u64> @ 0x2454564 -> 0x2450ed0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,enum2$<core::option::Option<array$<u32,2> > > >
// 信息 ID: be7df5117b1e6a353a27ab36
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["alloc::string::String"]; 字段布局 = unknown。
// [类型关系] array$ -> ["u32", "2"]; 字段布局 = unknown。
// [类型关系] core::option::Option -> ["array$<u32,2>"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["core::option::Option<array$<u32,2> >"]; 字段布局 = unknown。
// [类型关系] ron::ser::impl$11::serialize_field -> ["ref_mut$<alloc::string::String>", "enum2$<core::option::Option<array$<u32,2> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset::texture::desc::_::impl$0::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > > @ 0x20e049f -> 0x24547c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,std::collections::hash::map::HashMap<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta,std::hash::random::RandomState> >
// 信息 ID: 6a1a5f07f8aa854b3264f83b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["alloc::string::String"]; 字段布局 = unknown。
// [类型关系] std::collections::hash::map::HashMap -> ["relative_path::RelativePathBuf", "asset_pipe::texture::TextureAssetMeta", "std::hash::random::RandomState"]; 字段布局 = unknown。
// [类型关系] ron::ser::impl$11::serialize_field -> ["ref_mut$<alloc::string::String>", "std::collections::hash::map::HashMap<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta,std::hash::random::RandomState>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset_pipe::texture::_::impl$0::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > > @ 0x20e345b -> 0x24520e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,u32>
// 信息 ID: af3c6162b7c375b5ef7f49b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["alloc::string::String"]; 字段布局 = unknown。
// [类型关系] ron::ser::impl$11::serialize_field -> ["ref_mut$<alloc::string::String>", "u32"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::ser::impls::impl$23::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > > @ 0x2691177 -> 0x2454040 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,u64>
// 信息 ID: 81cb800fb65d55b48b2071f7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["alloc::string::String"]; 字段布局 = unknown。
// [类型关系] ron::ser::impl$11::serialize_field -> ["ref_mut$<alloc::string::String>", "u64"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::ser::impls::impl$23::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > > @ 0x269114b -> 0x2454400 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: ron::ser::impl$3::serialize_newtype_variant<ref_mut$<alloc::string::String>,u32>
// 信息 ID: 8c5cd7117b29cc6791f15fb0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["alloc::string::String"]; 字段布局 = unknown。
// [类型关系] ron::ser::impl$3::serialize_newtype_variant -> ["ref_mut$<alloc::string::String>", "u32"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset::texture::desc::_::impl$0::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > > @ 0x20e0403 -> 0x2455ff0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: serde::de::impls::impl$36::deserialize::impl$1::visit_map<ron::de::CommaSeparated>
// 信息 ID: 50cb1be9bbbeee4a0bf2f81b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] serde::de::impls::impl$36::deserialize::impl$1::visit_map -> ["ron::de::CommaSeparated"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::de::impl$2::deserialize_struct<serde::de::impls::impl$36::deserialize::DurationVisitor> @ 0x244c327 -> 0x268d400 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: zune_jpeg::decoder::JpegDecoder::decode_headers_internal<ref$<alloc::vec::Vec<u8,alloc::alloc::Global> > >
// 信息 ID: d599336988475a2bebe2c394
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] zune_jpeg::decoder::JpegDecoder::decode_headers_internal -> ["ref$<alloc::vec::Vec<u8,alloc::alloc::Global> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] image::codecs::jpeg::decoder::impl$1::icc_profile<std::io::buffered::bufreader::BufReader<std::fs::File> > @ 0x245dac7 -> 0x2697bd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: zune_jpeg::decoder::JpegDecoder::decode_headers_internal<ref$<slice2$<u8> > >
// 信息 ID: e26cb50d7c6839e2b3f86c78
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["u8"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<u8>"]; 字段布局 = unknown。
// [类型关系] zune_jpeg::decoder::JpegDecoder::decode_headers_internal -> ["ref$<slice2$<u8> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] image::codecs::jpeg::decoder::JpegDecoder::new<std::io::buffered::bufreader::BufReader<std::fs::File> > @ 0x245d5ef -> 0x26973c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] zune_jpeg::decoder::JpegDecoder::decode_into<ref$<slice2$<u8> > > @ 0x26970ca -> 0x26973c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: zune_jpeg::decoder::JpegDecoder::decode_into<ref$<slice2$<u8> > >
// 信息 ID: 275b25fde3ee2dabd0105c64
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["u8"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<u8>"]; 字段布局 = unknown。
// [类型关系] zune_jpeg::decoder::JpegDecoder::decode_into -> ["ref$<slice2$<u8> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] image::codecs::jpeg::decoder::impl$1::read_image_boxed<std::io::buffered::bufreader::BufReader<std::fs::File> > @ 0x245dcf6 -> 0x26970b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: zune_jpeg::headers::parse_app14<ref$<alloc::vec::Vec<u8,alloc::alloc::Global> > >
// 信息 ID: 486c9cea2fb6a6a1eb59b1f1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] zune_jpeg::headers::parse_app14 -> ["ref$<alloc::vec::Vec<u8,alloc::alloc::Global> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: zune_jpeg::headers::parse_app14<ref$<slice2$<u8> > >
// 信息 ID: a0e97594b7b74d2561590590
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["u8"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<u8>"]; 字段布局 = unknown。
// [类型关系] zune_jpeg::headers::parse_app14 -> ["ref$<slice2$<u8> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: zune_jpeg::headers::parse_dqt<ref$<alloc::vec::Vec<u8,alloc::alloc::Global> > >
// 信息 ID: 0865a9e086a30997ed0ec81c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] zune_jpeg::headers::parse_dqt -> ["ref$<alloc::vec::Vec<u8,alloc::alloc::Global> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: zune_jpeg::headers::parse_dqt<ref$<slice2$<u8> > >
// 信息 ID: daa132f13db3c0e2ec779116
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["u8"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<u8>"]; 字段布局 = unknown。
// [类型关系] zune_jpeg::headers::parse_dqt -> ["ref$<slice2$<u8> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: zune_jpeg::headers::parse_start_of_frame<ref$<alloc::vec::Vec<u8,alloc::alloc::Global> > >
// 信息 ID: a6020fbde6d4a949026a09ca
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] zune_jpeg::headers::parse_start_of_frame -> ["ref$<alloc::vec::Vec<u8,alloc::alloc::Global> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] zune_jpeg::decoder::JpegDecoder::decode_headers_internal<ref$<alloc::vec::Vec<u8,alloc::alloc::Global> > > @ 0x269801e -> 0x2880ec0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: zune_jpeg::headers::parse_start_of_frame<ref$<slice2$<u8> > >
// 信息 ID: c2a28c1ddd53ed14d685301b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["u8"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<u8>"]; 字段布局 = unknown。
// [类型关系] zune_jpeg::headers::parse_start_of_frame -> ["ref$<slice2$<u8> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] zune_jpeg::decoder::JpegDecoder::decode_headers_internal<ref$<slice2$<u8> > > @ 0x26977d3 -> 0x28806f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
