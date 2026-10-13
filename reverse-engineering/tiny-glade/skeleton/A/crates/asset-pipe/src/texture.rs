// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/asset-pipe/src/texture.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: ba5907ef276d345c982a4f646435610ab0861b7a864c4409c967d7b8fe934336
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: texture.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 2828, 3915, 3916, 3918, 3930, 4396, 4397
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"PDB_registry_path_only","version":"1.0.86"}]
// use asset; // namespace candidate
//   package identities: []
// use asset_pipe; // namespace candidate
//   package identities: []
// use byteorder; // namespace candidate
//   package identities: [{"name":"byteorder","source":"PDB_registry_path_only","version":"1.5.0"}]
// use core; // namespace candidate
//   package identities: []
// use ddsfile; // namespace candidate
//   package identities: [{"name":"ddsfile","source":"PDB_registry_path_only","version":"0.5.1"}]
// use easy_parallel; // namespace candidate
//   package identities: [{"name":"easy-parallel","source":"PDB_registry_path_only","version":"3.2.0"}]
// use image; // namespace candidate
//   package identities: [{"name":"image","source":"PDB_registry_path_only","version":"0.25.1"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use relative_path; // namespace candidate
//   package identities: [{"name":"relative-path","source":"PDB_registry_path_only","version":"1.9.0"}]
// use rkyv; // namespace candidate
//   package identities: []
// use ron; // namespace candidate
//   package identities: []
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"}]
// use std; // namespace candidate
//   package identities: []
// use walkdir; // namespace candidate
//   package identities: [{"name":"walkdir","source":"PDB_registry_path_only","version":"2.3.2"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::compile_texture_asset
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2828,"record_offset":24708,"rva":25236464,"va":5393945584}
// 编译实例 RVAs: 0x18113f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$std..fs..File$u20$as$u20$std..io..Write$GT$::write
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-16543040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e8c09203ff","size":5,"soft_fail":false,"target_va":5377407888,"va":5393950923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=10
//     target_aliases=89 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21631985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e80fecb5fe","size":5,"soft_fail":false,"target_va":5372314208,"va":5393946188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21197013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e82b8fbcfe","size":5,"soft_fail":false,"target_va":5372749696,"va":5393946704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=29
//     target_aliases=1 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21196088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e8c892bcfe","size":5,"soft_fail":false,"target_va":5372749712,"va":5393945795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140431000
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20841541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e8bbfbc1fe","size":5,"soft_fail":false,"target_va":5373104128,"va":5393945664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=9
//     target_aliases=28 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18774680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e86885e1fe","size":5,"soft_fail":false,"target_va":5375171056,"va":5393945731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14084b050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16542908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e8449303ff","size":5,"soft_fail":false,"target_va":5377405008,"va":5393947911,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14085a130
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16485481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e8977304ff","size":5,"soft_fail":false,"target_va":5377466672,"va":5393952148,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14180cac0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e8a88cffff","size":5,"soft_fail":false,"target_va":5393926848,"va":5393956371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141c8b1e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4687425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e841864700","size":5,"soft_fail":false,"target_va":5398639072,"va":5393951642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141c8c660
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4694121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e869a04700","size":5,"soft_fail":false,"target_va":5398644320,"va":5393950194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141c8c7f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4695558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e806a64700","size":5,"soft_fail":false,"target_va":5398644720,"va":5393949157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141c8c950
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4694996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e8d4a34700","size":5,"soft_fail":false,"target_va":5398645072,"va":5393950071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141c8cad0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4693282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e8229d4700","size":5,"soft_fail":false,"target_va":5398645456,"va":5393952169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141c92aa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4717941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e875fd4700","size":5,"soft_fail":false,"target_va":5398669984,"va":5393952038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9644802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e8022b9300","size":5,"soft_fail":false,"target_va":5403593344,"va":5393948537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9642007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e817209300","size":5,"soft_fail":false,"target_va":5403593360,"va":5393951348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t9026818","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e802bd8900","size":5,"soft_fail":false,"target_va":5402978144,"va":5393951321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t-16419501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e8537505ff","size":5,"soft_fail":false,"target_va":5377529888,"va":5393949384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8971919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e88fe68800","size":5,"soft_fail":false,"target_va":5402919072,"va":5393947148,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-16427575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e8c95505ff","size":5,"soft_fail":false,"target_va":5377527120,"va":5393954690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::__private::format_err
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t9566252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e82cf89100","size":5,"soft_fail":false,"target_va":5403519216,"va":5393952959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h9ad512a5a77fe080 (.llvm.16495867581533459325)
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9599410","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e8b2799200","size":5,"soft_fail":false,"target_va":5403547648,"va":5393948233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::msg
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t9597434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e8fa719200","size":5,"soft_fail":false,"target_va":5403546832,"va":5393949393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t9598614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e896769200","size":5,"soft_fail":false,"target_va":5403549296,"va":5393950677,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset_pipe::texture::process_regular_image
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t10623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e87f290000","size":5,"soft_fail":false,"target_va":5393957152,"va":5393946524,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::iter::traits::iterator::Iterator::eq_by
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4709025","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e8a1da4700","size":5,"soft_fail":false,"target_va":5398657856,"va":5393948826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t9024475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e8dbb38900","size":5,"soft_fail":false,"target_va":5402978880,"va":5393954400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9596691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e8136f9200","size":5,"soft_fail":false,"target_va":5403551568,"va":5393954872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9023882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e88ab18900","size":5,"soft_fail":false,"target_va":5402978944,"va":5393955057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$asset..texture..desc..TextureAssetDesc$GT$
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t4668584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e8a83c4700","size":5,"soft_fail":false,"target_va":5398619376,"va":5393950787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=9
//     evidence={"assembly":"\tcallq\t9025881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e859b98900","size":5,"soft_fail":false,"target_va":5402980128,"va":5393954242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_start_index_len_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9025773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e8edb88900","size":5,"soft_fail":false,"target_va":5402980288,"va":5393954510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ddsfile::Dds::get_d3d_format
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4713224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e808eb4700","size":5,"soft_fail":false,"target_va":5398664704,"va":5393951475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ddsfile::Dds::get_data
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4714132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e894ee4700","size":5,"soft_fail":false,"target_va":5398666720,"va":5393952583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ddsfile::Dds::get_dxgi_format
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t4713260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e82ceb4700","size":5,"soft_fail":false,"target_va":5398664720,"va":5393951455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ddsfile::Dds::read
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4712630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e8b6e84700","size":5,"soft_fail":false,"target_va":5398662800,"va":5393950165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-16592616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e818d102ff","size":5,"soft_fail":false,"target_va":5377353776,"va":5393946387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] relative_path::RelativePath::to_path
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4680918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e8d66c4700","size":5,"soft_fail":false,"target_va":5398626688,"va":5393945765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] relative_path::RelativePathBuf::set_extension
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4681346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e8826e4700","size":5,"soft_fail":false,"target_va":5398627040,"va":5393945689,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rkyv::util::to_bytes<asset::texture::blob::TextureAssetBlob,256>
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-23038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e802a6ffff","size":5,"soft_fail":false,"target_va":5393927360,"va":5393950393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-16552584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e8786d03ff","size":5,"soft_fail":false,"target_va":5377395552,"va":5393948131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirBuilder::_create
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-16539349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e82ba103ff","size":5,"soft_fail":false,"target_va":5377408752,"va":5393948096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::File::open
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-13186437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e87bca36ff","size":5,"soft_fail":false,"target_va":5380765664,"va":5393952096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::Metadata::modified
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-16537921","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e8bfa603ff","size":5,"soft_fail":false,"target_va":5377408048,"va":5393945964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::OpenOptions::_open
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-16541131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e8359a03ff","size":5,"soft_fail":false,"target_va":5377408016,"va":5393949142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-16544960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e8408b03ff","size":5,"soft_fail":false,"target_va":5377405072,"va":5393950027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::io::Write::write_all
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4712403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e8d3e74700","size":5,"soft_fail":false,"target_va":5398663648,"va":5393951240,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::components
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-16507607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e8291d04ff","size":5,"soft_fail":false,"target_va":5377440784,"va":5393948386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::extension
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-16508213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e8cb1a04ff","size":5,"soft_fail":false,"target_va":5377439664,"va":5393947872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::parent
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-16511123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e86d0f04ff","size":5,"soft_fail":false,"target_va":5377436944,"va":5393948062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::stat
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-16472580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393945584,"raw_hex":"e8fca504ff","size":5,"soft_fail":false,"target_va":5377473264,"va":5393945839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t*17332789(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5393945584,"raw_hex":"ff15357a0801","size":6,"soft_fail":false,"va":5393951765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2828,"record_offset":26204,"parent_record_offset":24708,"ancestor_record_offset":24708,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":2828,"record_offset":31876,"parent_record_offset":24708,"ancestor_record_offset":24708,"inlinee":10988,"source_line":46,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] asset::texture::desc::impl$27::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":2828,"record_offset":39644,"parent_record_offset":24708,"ancestor_record_offset":24708,"inlinee":90292,"source_line":35,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] asset::texture::desc::impl$31::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":2828,"record_offset":32012,"parent_record_offset":24708,"ancestor_record_offset":24708,"inlinee":90286,"source_line":35,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] asset_pipe::texture::process_dds_image
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":2828,"record_offset":33784,"parent_record_offset":24708,"ancestor_record_offset":24708,"inlinee":90289,"source_line":516,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] asset_pipe::texture::process_image_array
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":2828,"record_offset":26232,"parent_record_offset":24708,"ancestor_record_offset":24708,"inlinee":90272,"source_line":580,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] byteorder::io::ReadBytesExt::read_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"byteorder","version":"1.5.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/byteorder-1.5.0/src/io.rs
//     evidence={"module":2828,"record_offset":41612,"parent_record_offset":24708,"ancestor_record_offset":24708,"inlinee":45283,"source_line":217,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] byteorder::io::WriteBytesExt::write_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"byteorder","version":"1.5.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/byteorder-1.5.0/src/io.rs
//     evidence={"module":2828,"record_offset":38444,"parent_record_offset":24708,"ancestor_record_offset":24708,"inlinee":84846,"source_line":1259,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2828,"record_offset":26116,"parent_record_offset":24708,"ancestor_record_offset":24708,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2828,"record_offset":26148,"parent_record_offset":24708,"ancestor_record_offset":24708,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2828,"record_offset":31340,"parent_record_offset":24708,"ancestor_record_offset":24708,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2828,"record_offset":25208,"parent_record_offset":24708,"ancestor_record_offset":24708,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2828,"record_offset":25668,"parent_record_offset":24708,"ancestor_record_offset":24708,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2828,"record_offset":31584,"parent_record_offset":24708,"ancestor_record_offset":24708,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2828,"record_offset":31984,"parent_record_offset":24708,"ancestor_record_offset":24708,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2828,"record_offset":31468,"parent_record_offset":24708,"ancestor_record_offset":24708,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":2828,"record_offset":26176,"parent_record_offset":24708,"ancestor_record_offset":24708,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":2828,"record_offset":26016,"parent_record_offset":24708,"ancestor_record_offset":24708,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] relative_path::RelativePath::with_extension
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"relative-path","version":"1.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/relative-path-1.9.0/src/lib.rs
//     evidence={"module":2828,"record_offset":24936,"parent_record_offset":24708,"ancestor_record_offset":24708,"inlinee":51223,"source_line":1536,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":2828,"record_offset":31376,"parent_record_offset":24708,"ancestor_record_offset":24708,"inlinee":6730,"source_line":812,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] std::fs::File::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":2828,"record_offset":33232,"parent_record_offset":24708,"ancestor_record_offset":24708,"inlinee":5265,"source_line":403,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":2828,"record_offset":31792,"parent_record_offset":24708,"ancestor_record_offset":24708,"inlinee":4768,"source_line":2306,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::exists
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":25892,"parent_record_offset":24708,"ancestor_record_offset":24708,"inlinee":4766,"source_line":2862,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":25580,"parent_record_offset":24708,"ancestor_record_offset":24708,"inlinee":7564,"source_line":2750,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":25440,"parent_record_offset":24708,"ancestor_record_offset":24708,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] std::time::impl$29::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\time.rs
//     evidence={"module":2828,"record_offset":41236,"parent_record_offset":24708,"ancestor_record_offset":24708,"inlinee":13899,"source_line":246,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2828,"record_offset":27228,"parent_record_offset":27204,"ancestor_record_offset":24708,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2828,"record_offset":27252,"parent_record_offset":27228,"ancestor_record_offset":24708,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2828,"record_offset":25152,"parent_record_offset":25128,"ancestor_record_offset":24708,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2828,"record_offset":27204,"parent_record_offset":27168,"ancestor_record_offset":24708,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2828,"record_offset":25128,"parent_record_offset":25072,"ancestor_record_offset":24708,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2828,"record_offset":29536,"parent_record_offset":26232,"ancestor_record_offset":24708,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2828,"record_offset":29584,"parent_record_offset":29560,"ancestor_record_offset":24708,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":25100,"parent_record_offset":25072,"ancestor_record_offset":24708,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":28692,"parent_record_offset":28664,"ancestor_record_offset":24708,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":28664,"parent_record_offset":28640,"ancestor_record_offset":24708,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":27168,"parent_record_offset":27136,"ancestor_record_offset":24708,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":27136,"parent_record_offset":27104,"ancestor_record_offset":24708,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":27324,"parent_record_offset":27136,"ancestor_record_offset":24708,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":25072,"parent_record_offset":25044,"ancestor_record_offset":24708,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2828,"record_offset":35856,"parent_record_offset":35808,"ancestor_record_offset":24708,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2828,"record_offset":35808,"parent_record_offset":35760,"ancestor_record_offset":24708,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2828,"record_offset":35712,"parent_record_offset":35664,"ancestor_record_offset":24708,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2828,"record_offset":35760,"parent_record_offset":35712,"ancestor_record_offset":24708,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2828,"record_offset":35664,"parent_record_offset":33784,"ancestor_record_offset":24708,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2828,"record_offset":27400,"parent_record_offset":27364,"ancestor_record_offset":24708,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2828,"record_offset":28640,"parent_record_offset":28600,"ancestor_record_offset":24708,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2828,"record_offset":27072,"parent_record_offset":27020,"ancestor_record_offset":24708,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2828,"record_offset":27104,"parent_record_offset":27072,"ancestor_record_offset":24708,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2828,"record_offset":39860,"parent_record_offset":39816,"ancestor_record_offset":24708,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2828,"record_offset":26932,"parent_record_offset":26888,"ancestor_record_offset":24708,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2828,"record_offset":28528,"parent_record_offset":26232,"ancestor_record_offset":24708,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2828,"record_offset":27672,"parent_record_offset":27624,"ancestor_record_offset":24708,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":2828,"record_offset":33144,"parent_record_offset":32012,"ancestor_record_offset":24708,"inlinee":4578,"source_line":16,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":2828,"record_offset":27784,"parent_record_offset":27672,"ancestor_record_offset":24708,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":2828,"record_offset":28760,"parent_record_offset":28600,"ancestor_record_offset":24708,"inlinee":5454,"source_line":13,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":2828,"record_offset":29332,"parent_record_offset":29304,"ancestor_record_offset":24708,"inlinee":5437,"source_line":30,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":2828,"record_offset":27364,"parent_record_offset":27020,"ancestor_record_offset":24708,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":2828,"record_offset":26976,"parent_record_offset":26932,"ancestor_record_offset":24708,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":2828,"record_offset":27020,"parent_record_offset":26976,"ancestor_record_offset":24708,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":2828,"record_offset":31944,"parent_record_offset":31908,"ancestor_record_offset":24708,"inlinee":5381,"source_line":173,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":2828,"record_offset":31908,"parent_record_offset":31876,"ancestor_record_offset":24708,"inlinee":5405,"source_line":23,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::blob::impl$3::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/blob.rs
//     evidence={"module":2828,"record_offset":28240,"parent_record_offset":28216,"ancestor_record_offset":24708,"inlinee":90280,"source_line":5,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::blob::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/blob.rs
//     evidence={"module":2828,"record_offset":28296,"parent_record_offset":28272,"ancestor_record_offset":24708,"inlinee":90282,"source_line":196,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$13::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":2828,"record_offset":33084,"parent_record_offset":32888,"ancestor_record_offset":24708,"inlinee":90300,"source_line":11,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$20::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":2828,"record_offset":33112,"parent_record_offset":32888,"ancestor_record_offset":24708,"inlinee":90302,"source_line":19,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$26::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":2828,"record_offset":32888,"parent_record_offset":32012,"ancestor_record_offset":24708,"inlinee":90288,"source_line":26,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$3::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":2828,"record_offset":33056,"parent_record_offset":32888,"ancestor_record_offset":24708,"inlinee":90298,"source_line":4,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::process_dds_image::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":2828,"record_offset":34928,"parent_record_offset":34896,"ancestor_record_offset":24708,"inlinee":90296,"source_line":528,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::process_dds_image::closure$3
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":2828,"record_offset":35040,"parent_record_offset":34896,"ancestor_record_offset":24708,"inlinee":90303,"source_line":529,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] byteorder::impl$3::write_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"byteorder","version":"1.5.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/byteorder-1.5.0/src/lib.rs
//     evidence={"module":2828,"record_offset":38480,"parent_record_offset":38444,"ancestor_record_offset":24708,"inlinee":90290,"source_line":2169,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":2828,"record_offset":32992,"parent_record_offset":32956,"ancestor_record_offset":24708,"inlinee":4535,"source_line":10,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":2828,"record_offset":28132,"parent_record_offset":26232,"ancestor_record_offset":24708,"inlinee":8003,"source_line":14,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":2828,"record_offset":28180,"parent_record_offset":28156,"ancestor_record_offset":24708,"inlinee":6198,"source_line":147,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":2828,"record_offset":28156,"parent_record_offset":28132,"ancestor_record_offset":24708,"inlinee":6955,"source_line":153,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$19::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":2828,"record_offset":32576,"parent_record_offset":32484,"ancestor_record_offset":24708,"inlinee":4516,"source_line":219,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2828,"record_offset":28216,"parent_record_offset":26232,"ancestor_record_offset":24708,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2828,"record_offset":34812,"parent_record_offset":33784,"ancestor_record_offset":24708,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2828,"record_offset":31492,"parent_record_offset":31468,"ancestor_record_offset":24708,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":2828,"record_offset":33380,"parent_record_offset":33232,"ancestor_record_offset":24708,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$6::try_into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":2828,"record_offset":35572,"parent_record_offset":33784,"ancestor_record_offset":24708,"inlinee":4103,"source_line":797,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$88::try_from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\num.rs
//     evidence={"module":2828,"record_offset":35596,"parent_record_offset":35572,"ancestor_record_offset":24708,"inlinee":9589,"source_line":248,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2828,"record_offset":36284,"parent_record_offset":33784,"ancestor_record_offset":24708,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2828,"record_offset":35156,"parent_record_offset":33784,"ancestor_record_offset":24708,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2828,"record_offset":36232,"parent_record_offset":36208,"ancestor_record_offset":24708,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\copied.rs
//     evidence={"module":2828,"record_offset":27576,"parent_record_offset":27508,"ancestor_record_offset":24708,"inlinee":17354,"source_line":32,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\copied.rs
//     evidence={"module":2828,"record_offset":27472,"parent_record_offset":27436,"ancestor_record_offset":24708,"inlinee":17352,"source_line":73,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":2828,"record_offset":26544,"parent_record_offset":26232,"ancestor_record_offset":24708,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\rev.rs
//     evidence={"module":2828,"record_offset":32668,"parent_record_offset":32636,"ancestor_record_offset":24708,"inlinee":4521,"source_line":20,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2828,"record_offset":26888,"parent_record_offset":26232,"ancestor_record_offset":24708,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2828,"record_offset":27436,"parent_record_offset":27400,"ancestor_record_offset":24708,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2828,"record_offset":27624,"parent_record_offset":27576,"ancestor_record_offset":24708,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::rev
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2828,"record_offset":32636,"parent_record_offset":32332,"ancestor_record_offset":24708,"inlinee":4519,"source_line":3348,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2828,"record_offset":29188,"parent_record_offset":28852,"ancestor_record_offset":24708,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2828,"record_offset":28716,"parent_record_offset":28692,"ancestor_record_offset":24708,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2828,"record_offset":29560,"parent_record_offset":29536,"ancestor_record_offset":24708,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2828,"record_offset":29500,"parent_record_offset":26232,"ancestor_record_offset":24708,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2828,"record_offset":34864,"parent_record_offset":34840,"ancestor_record_offset":24708,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2828,"record_offset":32928,"parent_record_offset":32888,"ancestor_record_offset":24708,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2828,"record_offset":32956,"parent_record_offset":32928,"ancestor_record_offset":24708,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2828,"record_offset":32516,"parent_record_offset":32484,"ancestor_record_offset":24708,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2828,"record_offset":26732,"parent_record_offset":26708,"ancestor_record_offset":24708,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2828,"record_offset":36208,"parent_record_offset":35856,"ancestor_record_offset":24708,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=111
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2828,"record_offset":24972,"parent_record_offset":24936,"ancestor_record_offset":24708,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2828,"record_offset":26508,"parent_record_offset":26484,"ancestor_record_offset":24708,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2828,"record_offset":27828,"parent_record_offset":27508,"ancestor_record_offset":24708,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2828,"record_offset":26612,"parent_record_offset":26572,"ancestor_record_offset":24708,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2828,"record_offset":27280,"parent_record_offset":27252,"ancestor_record_offset":24708,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2828,"record_offset":27732,"parent_record_offset":27672,"ancestor_record_offset":24708,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2828,"record_offset":25628,"parent_record_offset":25604,"ancestor_record_offset":24708,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2828,"record_offset":31400,"parent_record_offset":31376,"ancestor_record_offset":24708,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2828,"record_offset":26768,"parent_record_offset":26232,"ancestor_record_offset":24708,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2828,"record_offset":26652,"parent_record_offset":26232,"ancestor_record_offset":24708,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2828,"record_offset":30540,"parent_record_offset":26232,"ancestor_record_offset":24708,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":2828,"record_offset":31516,"parent_record_offset":31492,"ancestor_record_offset":24708,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":2828,"record_offset":31540,"parent_record_offset":31516,"ancestor_record_offset":24708,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2828,"record_offset":38504,"parent_record_offset":38480,"ancestor_record_offset":24708,"inlinee":5235,"source_line":3626,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2828,"record_offset":26460,"parent_record_offset":26232,"ancestor_record_offset":24708,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2828,"record_offset":38820,"parent_record_offset":38564,"ancestor_record_offset":24708,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2828,"record_offset":38908,"parent_record_offset":38884,"ancestor_record_offset":24708,"inlinee":5475,"source_line":373,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2828,"record_offset":38884,"parent_record_offset":38848,"ancestor_record_offset":24708,"inlinee":5473,"source_line":489,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2828,"record_offset":38848,"parent_record_offset":38820,"ancestor_record_offset":24708,"inlinee":5472,"source_line":501,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":2828,"record_offset":26484,"parent_record_offset":26460,"ancestor_record_offset":24708,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2828,"record_offset":27508,"parent_record_offset":27472,"ancestor_record_offset":24708,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2828,"record_offset":26572,"parent_record_offset":26544,"ancestor_record_offset":24708,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2828,"record_offset":26044,"parent_record_offset":26016,"ancestor_record_offset":24708,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2828,"record_offset":26076,"parent_record_offset":26044,"ancestor_record_offset":24708,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] ddsfile::Dds::get_depth
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ddsfile","version":"0.5.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ddsfile-0.5.1/src/lib.rs
//     evidence={"module":2828,"record_offset":34840,"parent_record_offset":33784,"ancestor_record_offset":24708,"inlinee":90294,"source_line":249,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] ddsfile::Dds::get_num_array_layers
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ddsfile","version":"0.5.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ddsfile-0.5.1/src/lib.rs
//     evidence={"module":2828,"record_offset":35544,"parent_record_offset":33784,"ancestor_record_offset":24708,"inlinee":90305,"source_line":314,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] ddsfile::Dds::get_num_mipmap_levels
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ddsfile","version":"0.5.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ddsfile-0.5.1/src/lib.rs
//     evidence={"module":2828,"record_offset":35516,"parent_record_offset":33784,"ancestor_record_offset":24708,"inlinee":90304,"source_line":322,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":2828,"record_offset":35184,"parent_record_offset":33784,"ancestor_record_offset":24708,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":2828,"record_offset":34728,"parent_record_offset":33784,"ancestor_record_offset":24708,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/rel_ptr/mod.rs
//     evidence={"module":2828,"record_offset":28428,"parent_record_offset":28404,"ancestor_record_offset":24708,"inlinee":25179,"source_line":212,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::offset
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/rel_ptr/mod.rs
//     evidence={"module":2828,"record_offset":28452,"parent_record_offset":28428,"ancestor_record_offset":24708,"inlinee":25180,"source_line":200,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RelPtr::as_ptr
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/rel_ptr/mod.rs
//     evidence={"module":2828,"record_offset":28404,"parent_record_offset":28376,"ancestor_record_offset":24708,"inlinee":25177,"source_line":418,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::impl$18::to_isize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/rel_ptr/mod.rs
//     evidence={"module":2828,"record_offset":28476,"parent_record_offset":28452,"ancestor_record_offset":24708,"inlinee":25182,"source_line":110,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::util::aligned_vec::impl$0::drop
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/util/aligned_vec.rs
//     evidence={"module":2828,"record_offset":29432,"parent_record_offset":29404,"ancestor_record_offset":24708,"inlinee":90284,"source_line":22,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::util::archived_root
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/util/mod.rs
//     evidence={"module":2828,"record_offset":26684,"parent_record_offset":26232,"ancestor_record_offset":24708,"inlinee":90274,"source_line":147,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::util::archived_value
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/util/mod.rs
//     evidence={"module":2828,"record_offset":26708,"parent_record_offset":26684,"ancestor_record_offset":24708,"inlinee":90275,"source_line":63,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::as_ptr
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/vec/mod.rs
//     evidence={"module":2828,"record_offset":28376,"parent_record_offset":28352,"ancestor_record_offset":24708,"inlinee":25175,"source_line":32,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::as_slice
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/vec/mod.rs
//     evidence={"module":2828,"record_offset":26828,"parent_record_offset":26804,"ancestor_record_offset":24708,"inlinee":25174,"source_line":50,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::len
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/vec/mod.rs
//     evidence={"module":2828,"record_offset":26852,"parent_record_offset":26828,"ancestor_record_offset":24708,"inlinee":90278,"source_line":38,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::impl$4::deref
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/vec/mod.rs
//     evidence={"module":2828,"record_offset":26804,"parent_record_offset":26232,"ancestor_record_offset":24708,"inlinee":90277,"source_line":207,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":2828,"record_offset":39736,"parent_record_offset":39696,"ancestor_record_offset":24708,"inlinee":4734,"source_line":605,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":2828,"record_offset":25488,"parent_record_offset":25464,"ancestor_record_offset":24708,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":2828,"record_offset":25536,"parent_record_offset":25512,"ancestor_record_offset":24708,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":2828,"record_offset":25512,"parent_record_offset":25488,"ancestor_record_offset":24708,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":2828,"record_offset":31844,"parent_record_offset":31792,"ancestor_record_offset":24708,"inlinee":4771,"source_line":2561,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":2828,"record_offset":31816,"parent_record_offset":31792,"ancestor_record_offset":24708,"inlinee":4770,"source_line":2537,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":2828,"record_offset":33256,"parent_record_offset":33232,"ancestor_record_offset":24708,"inlinee":5267,"source_line":925,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":2828,"record_offset":33576,"parent_record_offset":33232,"ancestor_record_offset":24708,"inlinee":5272,"source_line":1130,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::metadata
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":2828,"record_offset":25604,"parent_record_offset":25580,"ancestor_record_offset":24708,"inlinee":4767,"source_line":1926,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":2828,"record_offset":33996,"parent_record_offset":33784,"ancestor_record_offset":24708,"inlinee":7565,"source_line":259,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::io::Write::write_all
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\mod.rs
//     evidence={"module":2828,"record_offset":38564,"parent_record_offset":38444,"ancestor_record_offset":24708,"inlinee":11087,"source_line":1701,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::Cursor::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\cursor.rs
//     evidence={"module":2828,"record_offset":34252,"parent_record_offset":33784,"ancestor_record_offset":24708,"inlinee":18799,"source_line":99,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::io::cursor::impl$4::read_exact
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\cursor.rs
//     evidence={"module":2828,"record_offset":41636,"parent_record_offset":41612,"ancestor_record_offset":24708,"inlinee":24848,"source_line":354,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::Error::is_interrupted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\error.rs
//     evidence={"module":2828,"record_offset":38624,"parent_record_offset":38564,"ancestor_record_offset":24708,"inlinee":11077,"source_line":958,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::impl$20::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\error.rs
//     evidence={"module":2828,"record_offset":38664,"parent_record_offset":38624,"ancestor_record_offset":24708,"inlinee":11079,"source_line":187,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::Repr::data
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":2828,"record_offset":38692,"parent_record_offset":38624,"ancestor_record_offset":24708,"inlinee":11080,"source_line":215,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":2828,"record_offset":38720,"parent_record_offset":38692,"ancestor_record_offset":24708,"inlinee":11082,"source_line":251,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::kind_from_prim
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":2828,"record_offset":38752,"parent_record_offset":38720,"ancestor_record_offset":24708,"inlinee":11083,"source_line":294,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$8::read_exact
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\impls.rs
//     evidence={"module":2828,"record_offset":41660,"parent_record_offset":41636,"ancestor_record_offset":24708,"inlinee":24844,"source_line":282,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\os\windows\io\handle.rs
//     evidence={"module":2828,"record_offset":40804,"parent_record_offset":40780,"ancestor_record_offset":24708,"inlinee":5199,"source_line":392,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":25464,"parent_record_offset":25440,"ancestor_record_offset":24708,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$112::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":32484,"parent_record_offset":32332,"ancestor_record_offset":24708,"inlinee":4510,"source_line":594,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$23::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":32332,"parent_record_offset":32052,"ancestor_record_offset":24708,"inlinee":4500,"source_line":992,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":39696,"parent_record_offset":39644,"ancestor_record_offset":24708,"inlinee":4732,"source_line":1627,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":32076,"parent_record_offset":32052,"ancestor_record_offset":24708,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$57::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":32052,"parent_record_offset":32012,"ancestor_record_offset":24708,"inlinee":4680,"source_line":1914,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":33404,"parent_record_offset":33380,"ancestor_record_offset":24708,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$85::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":32544,"parent_record_offset":32516,"ancestor_record_offset":24708,"inlinee":4514,"source_line":141,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$94::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":32604,"parent_record_offset":32484,"ancestor_record_offset":24708,"inlinee":4518,"source_line":376,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$96::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":32372,"parent_record_offset":32332,"ancestor_record_offset":24708,"inlinee":4502,"source_line":376,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$9::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":2828,"record_offset":39776,"parent_record_offset":39736,"ancestor_record_offset":24708,"inlinee":4736,"source_line":12,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":2828,"record_offset":33320,"parent_record_offset":33256,"ancestor_record_offset":24708,"inlinee":5270,"source_line":218,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":2828,"record_offset":33348,"parent_record_offset":33256,"ancestor_record_offset":24708,"inlinee":5271,"source_line":215,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":2828,"record_offset":33292,"parent_record_offset":33256,"ancestor_record_offset":24708,"inlinee":5269,"source_line":209,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::time::SystemTime::intervals
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\time.rs
//     evidence={"module":2828,"record_offset":41284,"parent_record_offset":41260,"ancestor_record_offset":24708,"inlinee":16278,"source_line":85,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::time::impl$2::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\time.rs
//     evidence={"module":2828,"record_offset":41260,"parent_record_offset":41236,"ancestor_record_offset":24708,"inlinee":13901,"source_line":111,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$34::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys_common\wtf8.rs
//     evidence={"module":2828,"record_offset":39816,"parent_record_offset":39776,"ancestor_record_offset":24708,"inlinee":4738,"source_line":135,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 35, 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B645
// 首条位置: {"module":3918,"inlinee":112197}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::tag::impl$1::deserialize_struct
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/tag.rs
//     evidence={"module":4396,"record_offset":16400,"parent_record_offset":16376,"ancestor_record_offset":16376,"inlinee":8724,"source_line":221,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 35, 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B696
// 首条位置: {"module":4396,"inlinee":112278}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":4396,"record_offset":28256,"parent_record_offset":28228,"ancestor_record_offset":28228,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":4396,"record_offset":28280,"parent_record_offset":28256,"ancestor_record_offset":28228,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":4396,"record_offset":28304,"parent_record_offset":28280,"ancestor_record_offset":28228,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":4396,"record_offset":28328,"parent_record_offset":28304,"ancestor_record_offset":28228,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 35, 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D8BF
// 首条位置: {"module":4396,"inlinee":121023}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":4396,"record_offset":28120,"parent_record_offset":28088,"ancestor_record_offset":28088,"inlinee":10204,"source_line":747,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":4396,"record_offset":28228,"parent_record_offset":28200,"ancestor_record_offset":28088,"inlinee":112278,"source_line":35,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":4396,"record_offset":28280,"parent_record_offset":28256,"ancestor_record_offset":28088,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4396,"record_offset":28164,"parent_record_offset":28120,"ancestor_record_offset":28088,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":4396,"record_offset":28304,"parent_record_offset":28280,"ancestor_record_offset":28088,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":4396,"record_offset":28328,"parent_record_offset":28304,"ancestor_record_offset":28088,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":4396,"record_offset":28256,"parent_record_offset":28228,"ancestor_record_offset":28088,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4396,"record_offset":28200,"parent_record_offset":28120,"ancestor_record_offset":28088,"inlinee":9500,"source_line":1504,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 35, 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D8BD
// 首条位置: {"module":4396,"inlinee":121021}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4396,"record_offset":28644,"parent_record_offset":27324,"ancestor_record_offset":27324,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4396,"record_offset":29848,"parent_record_offset":27324,"ancestor_record_offset":27324,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4396,"record_offset":28544,"parent_record_offset":27324,"ancestor_record_offset":27324,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4396,"record_offset":29876,"parent_record_offset":27324,"ancestor_record_offset":27324,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4396,"record_offset":27376,"parent_record_offset":27324,"ancestor_record_offset":27324,"inlinee":9790,"source_line":1846,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_direct] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4396,"record_offset":28580,"parent_record_offset":27324,"ancestor_record_offset":27324,"inlinee":9792,"source_line":1863,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":4396,"record_offset":29588,"parent_record_offset":29564,"ancestor_record_offset":27324,"inlinee":112265,"source_line":11,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":4396,"record_offset":28228,"parent_record_offset":28200,"ancestor_record_offset":27324,"inlinee":112278,"source_line":35,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":4396,"record_offset":27724,"parent_record_offset":27700,"ancestor_record_offset":27324,"inlinee":121023,"source_line":35,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":4396,"record_offset":28280,"parent_record_offset":28256,"ancestor_record_offset":27324,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":4396,"record_offset":27580,"parent_record_offset":27412,"ancestor_record_offset":27324,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":4396,"record_offset":27620,"parent_record_offset":27580,"ancestor_record_offset":27324,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":4396,"record_offset":27756,"parent_record_offset":27412,"ancestor_record_offset":27324,"inlinee":4980,"source_line":1499,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4396,"record_offset":29048,"parent_record_offset":28972,"ancestor_record_offset":27324,"inlinee":7550,"source_line":1231,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4396,"record_offset":38448,"parent_record_offset":37968,"ancestor_record_offset":36736,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4396,"record_offset":27540,"parent_record_offset":27412,"ancestor_record_offset":27324,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":4396,"record_offset":29772,"parent_record_offset":29328,"ancestor_record_offset":27324,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":4396,"record_offset":28304,"parent_record_offset":28280,"ancestor_record_offset":27324,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":4396,"record_offset":28328,"parent_record_offset":28304,"ancestor_record_offset":27324,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":4396,"record_offset":28256,"parent_record_offset":28228,"ancestor_record_offset":27324,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":4396,"record_offset":38520,"parent_record_offset":38496,"ancestor_record_offset":36736,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::id::Deserializer::new
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/id.rs
//     evidence={"module":4396,"record_offset":27660,"parent_record_offset":27412,"ancestor_record_offset":27324,"inlinee":10226,"source_line":11,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$2::deserialize_identifier
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":4396,"record_offset":28120,"parent_record_offset":28088,"ancestor_record_offset":27324,"inlinee":10204,"source_line":747,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_key_seed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":4396,"record_offset":27412,"parent_record_offset":27376,"ancestor_record_offset":27324,"inlinee":10224,"source_line":838,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::impl$6::next_value_seed
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":4396,"record_offset":28712,"parent_record_offset":28672,"ancestor_record_offset":27324,"inlinee":10245,"source_line":857,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_map
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/tag.rs
//     evidence={"module":4396,"record_offset":38248,"parent_record_offset":38224,"ancestor_record_offset":36736,"inlinee":83450,"source_line":214,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::tag::impl$1::deserialize_struct
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/tag.rs
//     evidence={"module":4396,"record_offset":29020,"parent_record_offset":28972,"ancestor_record_offset":27324,"inlinee":8724,"source_line":221,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_borrowed_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4396,"record_offset":28200,"parent_record_offset":28120,"ancestor_record_offset":27324,"inlinee":9500,"source_line":1504,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":4396,"record_offset":27700,"parent_record_offset":27412,"ancestor_record_offset":27324,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$204::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4396,"record_offset":38224,"parent_record_offset":38200,"ancestor_record_offset":36736,"inlinee":24937,"source_line":1505,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$36::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4396,"record_offset":28972,"parent_record_offset":28936,"ancestor_record_offset":27324,"inlinee":121025,"source_line":2276,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$36::deserialize::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":4396,"record_offset":29084,"parent_record_offset":29048,"ancestor_record_offset":27324,"inlinee":121027,"source_line":2414,"archive":"_pdb/A/modules/04396.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::_::impl$0::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > >
// 记录形式: compiled_procedure; instances=2
// 源码行: 35, 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3930,"record_offset":7864,"rva":29965888,"va":5398675008}
// 编译实例 RVAs: 0x1c93e40, 0x1c94050
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$$RF$mut$u20$ron..ser..Serializer$LT$W$GT$$u20$as$u20$serde..ser..Serializer$GT$::serialize_struct
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1974520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398675008,"raw_hex":"e8f8201e00","size":5,"soft_fail":false,"target_va":5400649600,"va":5398675075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$ron..ser..Compound$LT$W$GT$$u20$as$u20$serde..ser..SerializeStruct$GT$::serialize_field
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1961477","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398675008,"raw_hex":"e805ee1d00","size":5,"soft_fail":false,"target_va":5400636640,"va":5398675158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400105d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29899072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398675008,"raw_hex":"e8c0c637fe","size":5,"soft_fail":false,"target_va":5368776144,"va":5398675211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3930,"record_offset":8196,"parent_record_offset":7864,"ancestor_record_offset":7864,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3930,"record_offset":8068,"parent_record_offset":7864,"ancestor_record_offset":7864,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3930,"record_offset":8140,"parent_record_offset":7864,"ancestor_record_offset":7864,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3930,"record_offset":8248,"parent_record_offset":8220,"ancestor_record_offset":7864,"inlinee":4980,"source_line":1499,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] ron::ser::impl$5::drop
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/ser/mod.rs
//     evidence={"module":3930,"record_offset":8220,"parent_record_offset":8196,"ancestor_record_offset":7864,"inlinee":4979,"source_line":1098,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$1::default
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x16098
// 首条位置: {"module":2828,"inlinee":90264}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2828,"record_offset":14480,"parent_record_offset":14452,"ancestor_record_offset":14452,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2828,"record_offset":14752,"parent_record_offset":14724,"ancestor_record_offset":14452,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2828,"record_offset":14804,"parent_record_offset":14780,"ancestor_record_offset":14452,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2828,"record_offset":14780,"parent_record_offset":14724,"ancestor_record_offset":14452,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":2828,"record_offset":14828,"parent_record_offset":14804,"ancestor_record_offset":14452,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2828,"record_offset":14680,"parent_record_offset":14656,"ancestor_record_offset":14452,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2828,"record_offset":14852,"parent_record_offset":14828,"ancestor_record_offset":14452,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":2828,"record_offset":14528,"parent_record_offset":14504,"ancestor_record_offset":14452,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":2828,"record_offset":14604,"parent_record_offset":14576,"ancestor_record_offset":14452,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":2828,"record_offset":14724,"parent_record_offset":14576,"ancestor_record_offset":14452,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":2828,"record_offset":14504,"parent_record_offset":14480,"ancestor_record_offset":14452,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":2828,"record_offset":14628,"parent_record_offset":14604,"ancestor_record_offset":14452,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":2828,"record_offset":14656,"parent_record_offset":14628,"ancestor_record_offset":14452,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":2828,"record_offset":14576,"parent_record_offset":14552,"ancestor_record_offset":14452,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":2828,"record_offset":14552,"parent_record_offset":14528,"ancestor_record_offset":14452,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::compile_or_load_texture_assets
// 记录形式: compiled_procedure; instances=1
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2828,"record_offset":9484,"rva":25229424,"va":5393938544}
// 编译实例 RVAs: 0x180f870
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$std..path..Path$u20$as$u20$relative_path..path_ext..PathExt$GT$::relative_to
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4684143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393938544,"raw_hex":"e86f794700","size":5,"soft_fail":false,"target_va":5398623856,"va":5393939708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$walkdir..IntoIter$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14304627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393938544,"raw_hex":"e88dba25ff","size":5,"soft_fail":false,"target_va":5379634672,"va":5393939294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403666f0
//     owner=unresolved_namespace; count=1
//     target_aliases=26 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21665928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393938544,"raw_hex":"e87867b5fe","size":5,"soft_fail":false,"target_va":5372274416,"va":5393940339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=2
//     target_aliases=89 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21624568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393938544,"raw_hex":"e80809b6fe","size":5,"soft_fail":false,"target_va":5372314208,"va":5393938771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140382c20
//     owner=unresolved_namespace; count=1
//     target_aliases=37 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21550232","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393938544,"raw_hex":"e8682bb7fe","size":5,"soft_fail":false,"target_va":5372390432,"va":5393940659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21189576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393938544,"raw_hex":"e838acbcfe","size":5,"soft_fail":false,"target_va":5372749712,"va":5393939283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=2
//     target_aliases=28 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18767567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393938544,"raw_hex":"e831a1e1fe","size":5,"soft_fail":false,"target_va":5375171056,"va":5393938618,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14084b050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16534605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393938544,"raw_hex":"e8b3b303ff","size":5,"soft_fail":false,"target_va":5377405008,"va":5393939608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140852b50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16502119","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393938544,"raw_hex":"e8993204ff","size":5,"soft_fail":false,"target_va":5377436496,"va":5393938610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a5fc30
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14352872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393938544,"raw_hex":"e818fe24ff","size":5,"soft_fail":false,"target_va":5379587120,"va":5393939987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-22633056","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393938544,"raw_hex":"e8a0a5a6fe","size":5,"soft_fail":false,"target_va":5371306912,"va":5393939963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset_pipe::texture::compile_texture_assets
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t672","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393938544,"raw_hex":"e8a0020000","size":5,"soft_fail":false,"target_va":5393940768,"va":5393940091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9038951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393938544,"raw_hex":"e867ec8900","size":5,"soft_fail":false,"target_va":5402979152,"va":5393940196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] relative_path::RelativePath::file_stem
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14289549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393938544,"raw_hex":"e873f525ff","size":5,"soft_fail":false,"target_va":5379650240,"va":5393939784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::extension
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-16499922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393938544,"raw_hex":"e82e3b04ff","size":5,"soft_fail":false,"target_va":5377439664,"va":5393939581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::stat
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-16465377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393938544,"raw_hex":"e81fc204ff","size":5,"soft_fail":false,"target_va":5377473264,"va":5393938636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] walkdir::dent::DirEntry::path
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-14296170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393938544,"raw_hex":"e896db25ff","size":5,"soft_fail":false,"target_va":5379643408,"va":5393939573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2828,"record_offset":10180,"parent_record_offset":9484,"ancestor_record_offset":9484,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$25::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2828,"record_offset":10024,"parent_record_offset":9484,"ancestor_record_offset":9484,"inlinee":10163,"source_line":3216,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":2828,"record_offset":9672,"parent_record_offset":9484,"ancestor_record_offset":9484,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":2828,"record_offset":10808,"parent_record_offset":9484,"ancestor_record_offset":9484,"inlinee":8593,"source_line":63,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2828,"record_offset":11968,"parent_record_offset":9484,"ancestor_record_offset":9484,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2828,"record_offset":12096,"parent_record_offset":9484,"ancestor_record_offset":9484,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2828,"record_offset":10556,"parent_record_offset":9484,"ancestor_record_offset":9484,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2828,"record_offset":12972,"parent_record_offset":9484,"ancestor_record_offset":9484,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] relative_path::RelativePathBuf::set_extension
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"relative-path","version":"1.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/relative-path-1.9.0/src/lib.rs
//     evidence={"module":2828,"record_offset":12292,"parent_record_offset":9484,"ancestor_record_offset":9484,"inlinee":51220,"source_line":783,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::exists
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":9896,"parent_record_offset":9484,"ancestor_record_offset":9484,"inlinee":4766,"source_line":2862,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":9756,"parent_record_offset":9484,"ancestor_record_offset":9484,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] walkdir::WalkDir::follow_links
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"walkdir","version":"2.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/walkdir-2.3.2/src/lib.rs
//     evidence={"module":2828,"record_offset":10124,"parent_record_offset":9484,"ancestor_record_offset":9484,"inlinee":90259,"source_line":342,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] walkdir::WalkDir::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"walkdir","version":"2.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/walkdir-2.3.2/src/lib.rs
//     evidence={"module":2828,"record_offset":10080,"parent_record_offset":9484,"ancestor_record_offset":9484,"inlinee":17286,"source_line":286,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] walkdir::impl$2::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"walkdir","version":"2.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/walkdir-2.3.2/src/lib.rs
//     evidence={"module":2828,"record_offset":10152,"parent_record_offset":9484,"ancestor_record_offset":9484,"inlinee":17288,"source_line":517,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2828,"record_offset":10500,"parent_record_offset":10476,"ancestor_record_offset":9484,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2828,"record_offset":10476,"parent_record_offset":10420,"ancestor_record_offset":9484,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":10448,"parent_record_offset":10420,"ancestor_record_offset":9484,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":10264,"parent_record_offset":10232,"ancestor_record_offset":9484,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":10420,"parent_record_offset":10396,"ancestor_record_offset":9484,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":2828,"record_offset":12536,"parent_record_offset":12292,"ancestor_record_offset":9484,"inlinee":28376,"source_line":1408,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2828,"record_offset":10232,"parent_record_offset":10180,"ancestor_record_offset":9484,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2828,"record_offset":10048,"parent_record_offset":10024,"ancestor_record_offset":9484,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2828,"record_offset":12636,"parent_record_offset":12536,"ancestor_record_offset":9484,"inlinee":8655,"source_line":1192,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::compile_or_load_texture_assets::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":2828,"record_offset":10968,"parent_record_offset":10936,"ancestor_record_offset":9484,"inlinee":90261,"source_line":81,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::compile_or_load_texture_assets::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":2828,"record_offset":12000,"parent_record_offset":11968,"ancestor_record_offset":9484,"inlinee":90262,"source_line":83,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2828,"record_offset":12144,"parent_record_offset":12120,"ancestor_record_offset":9484,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2828,"record_offset":10840,"parent_record_offset":10808,"ancestor_record_offset":9484,"inlinee":8594,"source_line":2898,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2828,"record_offset":10908,"parent_record_offset":10872,"ancestor_record_offset":9484,"inlinee":8630,"source_line":2905,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2828,"record_offset":10872,"parent_record_offset":10840,"ancestor_record_offset":9484,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2828,"record_offset":12504,"parent_record_offset":12292,"ancestor_record_offset":9484,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":2828,"record_offset":12604,"parent_record_offset":12576,"ancestor_record_offset":9484,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":2828,"record_offset":10936,"parent_record_offset":10908,"ancestor_record_offset":9484,"inlinee":4399,"source_line":293,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2828,"record_offset":12120,"parent_record_offset":12096,"ancestor_record_offset":9484,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2828,"record_offset":12456,"parent_record_offset":12432,"ancestor_record_offset":9484,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=52
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2828,"record_offset":9992,"parent_record_offset":9896,"ancestor_record_offset":9484,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2828,"record_offset":10296,"parent_record_offset":10180,"ancestor_record_offset":9484,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2828,"record_offset":9956,"parent_record_offset":9928,"ancestor_record_offset":9484,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2828,"record_offset":10996,"parent_record_offset":10968,"ancestor_record_offset":9484,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":2828,"record_offset":12216,"parent_record_offset":12192,"ancestor_record_offset":9484,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":2828,"record_offset":12240,"parent_record_offset":12216,"ancestor_record_offset":9484,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":2828,"record_offset":12576,"parent_record_offset":12536,"ancestor_record_offset":9484,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2828,"record_offset":12384,"parent_record_offset":12360,"ancestor_record_offset":9484,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2828,"record_offset":12408,"parent_record_offset":12384,"ancestor_record_offset":9484,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2828,"record_offset":12360,"parent_record_offset":12336,"ancestor_record_offset":9484,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2828,"record_offset":12168,"parent_record_offset":12144,"ancestor_record_offset":9484,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2828,"record_offset":12336,"parent_record_offset":12292,"ancestor_record_offset":9484,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":2828,"record_offset":12432,"parent_record_offset":12408,"ancestor_record_offset":9484,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_os_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":2828,"record_offset":9720,"parent_record_offset":9696,"ancestor_record_offset":9484,"inlinee":5424,"source_line":884,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":2828,"record_offset":12028,"parent_record_offset":12000,"ancestor_record_offset":9484,"inlinee":6730,"source_line":812,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":2828,"record_offset":9804,"parent_record_offset":9780,"ancestor_record_offset":9484,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":2828,"record_offset":9852,"parent_record_offset":9828,"ancestor_record_offset":9484,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":2828,"record_offset":9828,"parent_record_offset":9804,"ancestor_record_offset":9484,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":2828,"record_offset":9928,"parent_record_offset":9896,"ancestor_record_offset":9484,"inlinee":4767,"source_line":1926,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":9780,"parent_record_offset":9756,"ancestor_record_offset":9484,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$36::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":9696,"parent_record_offset":9672,"ancestor_record_offset":9484,"inlinee":5860,"source_line":1700,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::compile_or_load_texture_assets::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x16095
// 首条位置: {"module":2828,"inlinee":90261}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2828,"record_offset":10996,"parent_record_offset":10968,"ancestor_record_offset":10968,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2828,"record_offset":11360,"parent_record_offset":11336,"ancestor_record_offset":10968,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2828,"record_offset":11336,"parent_record_offset":11280,"ancestor_record_offset":10968,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":11308,"parent_record_offset":11280,"ancestor_record_offset":10968,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":11280,"parent_record_offset":11252,"ancestor_record_offset":10968,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2828,"record_offset":11036,"parent_record_offset":10996,"ancestor_record_offset":10968,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::compile_or_load_texture_assets::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 83
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x16096
// 首条位置: {"module":2828,"inlinee":90262}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":2828,"record_offset":12028,"parent_record_offset":12000,"ancestor_record_offset":12000,"inlinee":6730,"source_line":812,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2828,"record_offset":12056,"parent_record_offset":12028,"ancestor_record_offset":12000,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::compile_texture_assets
// 记录形式: compiled_procedure; instances=1
// 源码行: 95
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2828,"record_offset":13872,"rva":25231648,"va":5393940768}
// 编译实例 RVAs: 0x1810120
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e16c0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24310076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393940768,"raw_hex":"e8c40e8dfe","size":5,"soft_fail":false,"target_va":5369632448,"va":5393942519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=3
//     target_aliases=89 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21626799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393940768,"raw_hex":"e85100b6fe","size":5,"soft_fail":false,"target_va":5372314208,"va":5393941002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140382c20
//     owner=unresolved_namespace; count=1
//     target_aliases=37 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21554559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393940768,"raw_hex":"e8811ab7fe","size":5,"soft_fail":false,"target_va":5372390432,"va":5393944986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=3
//     target_aliases=16 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21316010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393940768,"raw_hex":"e856bebafe","size":5,"soft_fail":false,"target_va":5372625024,"va":5393941029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=17
//     target_aliases=1 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21193000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393940768,"raw_hex":"e8d89ebcfe","size":5,"soft_fail":false,"target_va":5372749712,"va":5393942707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=5
//     target_aliases=28 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18769792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393940768,"raw_hex":"e88098e1fe","size":5,"soft_fail":false,"target_va":5375171056,"va":5393940843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140852b50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16504344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393940768,"raw_hex":"e8e82904ff","size":5,"soft_fail":false,"target_va":5377436496,"va":5393940835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141c8b470
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4698429","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393940768,"raw_hex":"e83db14700","size":5,"soft_fail":false,"target_va":5398639728,"va":5393941294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141c8cba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4703493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393940768,"raw_hex":"e805c54700","size":5,"soft_fail":false,"target_va":5398645664,"va":5393942166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141c8e010
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4708741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393940768,"raw_hex":"e885d94700","size":5,"soft_fail":false,"target_va":5398650896,"va":5393942150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141c8eee0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4711123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393940768,"raw_hex":"e8d3e24700","size":5,"soft_fail":false,"target_va":5398654688,"va":5393943560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141c8f340
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4712821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393940768,"raw_hex":"e875e94700","size":5,"soft_fail":false,"target_va":5398655808,"va":5393942982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h9ad512a5a77fe080 (.llvm.16495867581533459325)
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t9605765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393940768,"raw_hex":"e885929200","size":5,"soft_fail":false,"target_va":5403547648,"va":5393941878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t9606037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393940768,"raw_hex":"e895939200","size":5,"soft_fail":false,"target_va":5403549152,"va":5393943110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-16579983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393940768,"raw_hex":"e8710203ff","size":5,"soft_fail":false,"target_va":5377364336,"va":5393944314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset::texture::scan::scan_asset_tree
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4686870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393940768,"raw_hex":"e816844700","size":5,"soft_fail":false,"target_va":5398628032,"va":5393941157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9034544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393940768,"raw_hex":"e830db8900","size":5,"soft_fail":false,"target_va":5402978880,"va":5393944331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$alloc..vec..into_iter..IntoIter$LT$$LP$relative_path..RelativePathBuf$C$core..result..Result$LT$asset_pipe..texture..TextureAssetMeta$C$anyhow..Error$GT$$RP$$GT$$GT$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4672503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393940768,"raw_hex":"e8f74b4700","size":5,"soft_fail":false,"target_va":5398615232,"va":5393942724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$asset..texture..desc..TextureAssetDesc$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4676698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393940768,"raw_hex":"e85a5c4700","size":5,"soft_fail":false,"target_va":5398619376,"va":5393942673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-18657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393940768,"raw_hex":"e81fb7ffff","size":5,"soft_fail":false,"target_va":5393926592,"va":5393945244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<ron::error::Error> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-18936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393940768,"raw_hex":"e808b6ffff","size":5,"soft_fail":false,"target_va":5393926624,"va":5393945555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9035675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393940768,"raw_hex":"e89bdf8900","size":5,"soft_fail":false,"target_va":5402980128,"va":5393944448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::merge_sort
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4699314","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393940768,"raw_hex":"e8b2b44700","size":5,"soft_fail":false,"target_va":5398640656,"va":5393941337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::map::HashMap$LT$K$C$V$C$S$C$A$GT$::insert
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4711106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393940768,"raw_hex":"e8c2e24700","size":5,"soft_fail":false,"target_va":5398653760,"va":5393942649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t4713777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393940768,"raw_hex":"e831ed4700","size":5,"soft_fail":false,"target_va":5398657040,"va":5393943258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-16587737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393940768,"raw_hex":"e827e402ff","size":5,"soft_fail":false,"target_va":5377353776,"va":5393941508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-16546229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393940768,"raw_hex":"e84b8603ff","size":5,"soft_fail":false,"target_va":5377395552,"va":5393941776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirBuilder::_create
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-16534031","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393940768,"raw_hex":"e8f1b503ff","size":5,"soft_fail":false,"target_va":5377408752,"va":5393942778,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-16536106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393940768,"raw_hex":"e8d6ad03ff","size":5,"soft_fail":false,"target_va":5377405616,"va":5393941717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::write::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-16536761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393940768,"raw_hex":"e847ab03ff","size":5,"soft_fail":false,"target_va":5377406304,"va":5393943060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-16574788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393940768,"raw_hex":"e8bc1603ff","size":5,"soft_fail":false,"target_va":5377366224,"va":5393941007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::io::stdio::_print
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-16523137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393940768,"raw_hex":"e87fe003ff","size":5,"soft_fail":false,"target_va":5377418528,"va":5393941660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::parent
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-16505805","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393940768,"raw_hex":"e8332404ff","size":5,"soft_fail":false,"target_va":5377436944,"va":5393942744,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::stat
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-16467602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393940768,"raw_hex":"e86eb904ff","size":5,"soft_fail":false,"target_va":5377473264,"va":5393940861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2828,"record_offset":15656,"parent_record_offset":13872,"ancestor_record_offset":13872,"inlinee":8700,"source_line":207,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2828,"record_offset":17816,"parent_record_offset":13872,"ancestor_record_offset":13872,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":2828,"record_offset":17904,"parent_record_offset":13872,"ancestor_record_offset":13872,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":2828,"record_offset":18640,"parent_record_offset":13872,"ancestor_record_offset":13872,"inlinee":10988,"source_line":46,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::with_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":2828,"record_offset":16992,"parent_record_offset":13872,"ancestor_record_offset":13872,"inlinee":10987,"source_line":58,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] asset_pipe::texture::impl$1::default
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":2828,"record_offset":14452,"parent_record_offset":13872,"ancestor_record_offset":13872,"inlinee":90264,"source_line":35,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2828,"record_offset":15796,"parent_record_offset":13872,"ancestor_record_offset":13872,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":2828,"record_offset":14920,"parent_record_offset":13872,"ancestor_record_offset":13872,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2828,"record_offset":15824,"parent_record_offset":13872,"ancestor_record_offset":13872,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::cloned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2828,"record_offset":15544,"parent_record_offset":13872,"ancestor_record_offset":13872,"inlinee":12902,"source_line":3464,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2828,"record_offset":15600,"parent_record_offset":13872,"ancestor_record_offset":13872,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2828,"record_offset":18520,"parent_record_offset":13872,"ancestor_record_offset":13872,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2828,"record_offset":15908,"parent_record_offset":13872,"ancestor_record_offset":13872,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2828,"record_offset":21104,"parent_record_offset":13872,"ancestor_record_offset":13872,"inlinee":4730,"source_line":1319,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2828,"record_offset":18804,"parent_record_offset":13872,"ancestor_record_offset":13872,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2828,"record_offset":21728,"parent_record_offset":13872,"ancestor_record_offset":13872,"inlinee":6160,"source_line":1428,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2828,"record_offset":15152,"parent_record_offset":13872,"ancestor_record_offset":13872,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2828,"record_offset":15880,"parent_record_offset":13872,"ancestor_record_offset":13872,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] easy_parallel::Parallel::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"easy-parallel","version":"3.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/easy-parallel-3.2.0/src/lib.rs
//     evidence={"module":2828,"record_offset":17788,"parent_record_offset":13872,"ancestor_record_offset":13872,"inlinee":17154,"source_line":85,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":2828,"record_offset":15852,"parent_record_offset":13872,"ancestor_record_offset":13872,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":2828,"record_offset":15712,"parent_record_offset":13872,"ancestor_record_offset":13872,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] relative_path::impl$82::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"relative-path","version":"1.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/relative-path-1.9.0/src/lib.rs
//     evidence={"module":2828,"record_offset":18000,"parent_record_offset":13872,"ancestor_record_offset":13872,"inlinee":90266,"source_line":631,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] ron::ser::to_string
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/ser/mod.rs
//     evidence={"module":2828,"record_offset":18748,"parent_record_offset":13872,"ancestor_record_offset":13872,"inlinee":90267,"source_line":44,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2828,"record_offset":18064,"parent_record_offset":13872,"ancestor_record_offset":13872,"inlinee":8309,"source_line":1105,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::keys
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2828,"record_offset":15184,"parent_record_offset":13872,"ancestor_record_offset":13872,"inlinee":17284,"source_line":361,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$38::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2828,"record_offset":17112,"parent_record_offset":13872,"ancestor_record_offset":13872,"inlinee":21719,"source_line":2223,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":2828,"record_offset":18556,"parent_record_offset":13872,"ancestor_record_offset":13872,"inlinee":4768,"source_line":2306,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":2828,"record_offset":16768,"parent_record_offset":13872,"ancestor_record_offset":13872,"inlinee":11086,"source_line":302,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] std::fs::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":2828,"record_offset":18848,"parent_record_offset":13872,"ancestor_record_offset":13872,"inlinee":10620,"source_line":339,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::exists
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":14320,"parent_record_offset":13872,"ancestor_record_offset":13872,"inlinee":4766,"source_line":2862,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$36::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":14124,"parent_record_offset":13872,"ancestor_record_offset":13872,"inlinee":5860,"source_line":1700,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":14180,"parent_record_offset":13872,"ancestor_record_offset":13872,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2828,"record_offset":16180,"parent_record_offset":16156,"ancestor_record_offset":13872,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2828,"record_offset":16156,"parent_record_offset":16092,"ancestor_record_offset":13872,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":16124,"parent_record_offset":16092,"ancestor_record_offset":13872,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":16092,"parent_record_offset":16060,"ancestor_record_offset":13872,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2828,"record_offset":15680,"parent_record_offset":15656,"ancestor_record_offset":13872,"inlinee":6490,"source_line":842,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2828,"record_offset":15624,"parent_record_offset":15600,"ancestor_record_offset":13872,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2828,"record_offset":20164,"parent_record_offset":20140,"ancestor_record_offset":13872,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":2828,"record_offset":17072,"parent_record_offset":17036,"ancestor_record_offset":13872,"inlinee":5381,"source_line":173,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":2828,"record_offset":17036,"parent_record_offset":16992,"ancestor_record_offset":13872,"inlinee":5405,"source_line":23,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::compile_texture_assets::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":2828,"record_offset":21140,"parent_record_offset":21104,"ancestor_record_offset":13872,"inlinee":90269,"source_line":102,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::compile_texture_assets::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":2828,"record_offset":21768,"parent_record_offset":21728,"ancestor_record_offset":13872,"inlinee":90270,"source_line":103,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":2828,"record_offset":21976,"parent_record_offset":21768,"ancestor_record_offset":13872,"inlinee":90264,"source_line":35,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2828,"record_offset":14752,"parent_record_offset":14724,"ancestor_record_offset":13872,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2828,"record_offset":14804,"parent_record_offset":14780,"ancestor_record_offset":13872,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2828,"record_offset":14780,"parent_record_offset":14724,"ancestor_record_offset":13872,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2828,"record_offset":21892,"parent_record_offset":21768,"ancestor_record_offset":13872,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":2828,"record_offset":16792,"parent_record_offset":16768,"ancestor_record_offset":13872,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2828,"record_offset":15396,"parent_record_offset":15368,"ancestor_record_offset":13872,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2828,"record_offset":15456,"parent_record_offset":15424,"ancestor_record_offset":13872,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2828,"record_offset":21920,"parent_record_offset":21768,"ancestor_record_offset":13872,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::cloned::Cloned::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\cloned.rs
//     evidence={"module":2828,"record_offset":15568,"parent_record_offset":15544,"ancestor_record_offset":13872,"inlinee":12904,"source_line":23,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":2828,"record_offset":17844,"parent_record_offset":17816,"ancestor_record_offset":13872,"inlinee":4174,"source_line":70,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":2828,"record_offset":14828,"parent_record_offset":14804,"ancestor_record_offset":13872,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2828,"record_offset":17640,"parent_record_offset":17500,"ancestor_record_offset":13872,"inlinee":5724,"source_line":460,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2828,"record_offset":17532,"parent_record_offset":17500,"ancestor_record_offset":13872,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2828,"record_offset":17604,"parent_record_offset":17532,"ancestor_record_offset":13872,"inlinee":5725,"source_line":1975,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2828,"record_offset":17572,"parent_record_offset":17532,"ancestor_record_offset":13872,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2828,"record_offset":20556,"parent_record_offset":20528,"ancestor_record_offset":13872,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2828,"record_offset":14680,"parent_record_offset":14656,"ancestor_record_offset":13872,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2828,"record_offset":15340,"parent_record_offset":15308,"ancestor_record_offset":13872,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=82
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2828,"record_offset":14420,"parent_record_offset":14320,"ancestor_record_offset":13872,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2828,"record_offset":17872,"parent_record_offset":17816,"ancestor_record_offset":13872,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2828,"record_offset":17736,"parent_record_offset":17712,"ancestor_record_offset":13872,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2828,"record_offset":17712,"parent_record_offset":17440,"ancestor_record_offset":13872,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2828,"record_offset":17968,"parent_record_offset":17904,"ancestor_record_offset":13872,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2828,"record_offset":17936,"parent_record_offset":17904,"ancestor_record_offset":13872,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2828,"record_offset":14852,"parent_record_offset":14828,"ancestor_record_offset":13872,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2828,"record_offset":14380,"parent_record_offset":14352,"ancestor_record_offset":13872,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2828,"record_offset":21236,"parent_record_offset":21140,"ancestor_record_offset":13872,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2828,"record_offset":21492,"parent_record_offset":21140,"ancestor_record_offset":13872,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2828,"record_offset":15736,"parent_record_offset":15712,"ancestor_record_offset":13872,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2828,"record_offset":15760,"parent_record_offset":15736,"ancestor_record_offset":13872,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2828,"record_offset":15232,"parent_record_offset":15208,"ancestor_record_offset":13872,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$59::into_iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2828,"record_offset":17136,"parent_record_offset":17112,"ancestor_record_offset":13872,"inlinee":19625,"source_line":4716,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2828,"record_offset":15308,"parent_record_offset":15280,"ancestor_record_offset":13872,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::into_allocation
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2828,"record_offset":17440,"parent_record_offset":17416,"ancestor_record_offset":13872,"inlinee":19590,"source_line":1667,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::into_iter_from
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2828,"record_offset":17416,"parent_record_offset":17160,"ancestor_record_offset":13872,"inlinee":19589,"source_line":1653,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2828,"record_offset":15256,"parent_record_offset":15232,"ancestor_record_offset":13872,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2828,"record_offset":17472,"parent_record_offset":17440,"ancestor_record_offset":13872,"inlinee":6423,"source_line":2806,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2828,"record_offset":15280,"parent_record_offset":15256,"ancestor_record_offset":13872,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2828,"record_offset":17500,"parent_record_offset":17440,"ancestor_record_offset":13872,"inlinee":6234,"source_line":256,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2828,"record_offset":20096,"parent_record_offset":20072,"ancestor_record_offset":13872,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$19::into_iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2828,"record_offset":17160,"parent_record_offset":17136,"ancestor_record_offset":13872,"inlinee":19588,"source_line":3706,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2828,"record_offset":15368,"parent_record_offset":15308,"ancestor_record_offset":13872,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2828,"record_offset":15424,"parent_record_offset":15368,"ancestor_record_offset":13872,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":2828,"record_offset":21948,"parent_record_offset":21768,"ancestor_record_offset":13872,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":2828,"record_offset":21808,"parent_record_offset":21768,"ancestor_record_offset":13872,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] ron::de::from_str
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":2828,"record_offset":21176,"parent_record_offset":21140,"ancestor_record_offset":13872,"inlinee":10331,"source_line":108,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] ron::options::impl$0::default
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/options.rs
//     evidence={"module":2828,"record_offset":18772,"parent_record_offset":18748,"ancestor_record_offset":13872,"inlinee":4925,"source_line":51,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2828,"record_offset":15208,"parent_record_offset":15184,"ancestor_record_offset":13872,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2828,"record_offset":14480,"parent_record_offset":14452,"ancestor_record_offset":13872,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_os_string
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":2828,"record_offset":14148,"parent_record_offset":14124,"ancestor_record_offset":13872,"inlinee":5424,"source_line":884,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":2828,"record_offset":14228,"parent_record_offset":14204,"ancestor_record_offset":13872,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":2828,"record_offset":14276,"parent_record_offset":14252,"ancestor_record_offset":13872,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":2828,"record_offset":14252,"parent_record_offset":14228,"ancestor_record_offset":13872,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":2828,"record_offset":18608,"parent_record_offset":18556,"ancestor_record_offset":13872,"inlinee":4771,"source_line":2561,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":2828,"record_offset":18580,"parent_record_offset":18556,"ancestor_record_offset":13872,"inlinee":4770,"source_line":2537,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":2828,"record_offset":14352,"parent_record_offset":14320,"ancestor_record_offset":13872,"inlinee":4767,"source_line":1926,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":2828,"record_offset":14528,"parent_record_offset":14504,"ancestor_record_offset":13872,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":2828,"record_offset":14604,"parent_record_offset":14576,"ancestor_record_offset":13872,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":2828,"record_offset":14724,"parent_record_offset":14576,"ancestor_record_offset":13872,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":2828,"record_offset":14504,"parent_record_offset":14480,"ancestor_record_offset":13872,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":14204,"parent_record_offset":14180,"ancestor_record_offset":13872,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$36::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":14944,"parent_record_offset":14920,"ancestor_record_offset":13872,"inlinee":5860,"source_line":1700,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":16840,"parent_record_offset":16816,"ancestor_record_offset":13872,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":16816,"parent_record_offset":16792,"ancestor_record_offset":13872,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":2828,"record_offset":14628,"parent_record_offset":14604,"ancestor_record_offset":13872,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":2828,"record_offset":14656,"parent_record_offset":14628,"ancestor_record_offset":13872,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":2828,"record_offset":14576,"parent_record_offset":14552,"ancestor_record_offset":13872,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":2828,"record_offset":14552,"parent_record_offset":14528,"ancestor_record_offset":13872,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::compile_texture_assets::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 102
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1609D
// 首条位置: {"module":2828,"inlinee":90269}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2828,"record_offset":21272,"parent_record_offset":21140,"ancestor_record_offset":21140,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2828,"record_offset":21236,"parent_record_offset":21140,"ancestor_record_offset":21140,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2828,"record_offset":21492,"parent_record_offset":21140,"ancestor_record_offset":21140,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] ron::de::from_str
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/de/mod.rs
//     evidence={"module":2828,"record_offset":21176,"parent_record_offset":21140,"ancestor_record_offset":21140,"inlinee":10331,"source_line":108,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2828,"record_offset":21444,"parent_record_offset":21420,"ancestor_record_offset":21140,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2828,"record_offset":21420,"parent_record_offset":21356,"ancestor_record_offset":21140,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":21388,"parent_record_offset":21356,"ancestor_record_offset":21140,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":21356,"parent_record_offset":21328,"ancestor_record_offset":21140,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2828,"record_offset":21300,"parent_record_offset":21272,"ancestor_record_offset":21140,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] ron::options::impl$0::default
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/ron-a4572d20898be388/2787d8c/src/options.rs
//     evidence={"module":2828,"record_offset":21200,"parent_record_offset":21176,"ancestor_record_offset":21140,"inlinee":4925,"source_line":51,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::compile_texture_assets::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 103
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1609E
// 首条位置: {"module":2828,"inlinee":90270}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset_pipe::texture::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":2828,"record_offset":21976,"parent_record_offset":21768,"ancestor_record_offset":21768,"inlinee":90264,"source_line":35,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2828,"record_offset":21892,"parent_record_offset":21768,"ancestor_record_offset":21768,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2828,"record_offset":21920,"parent_record_offset":21768,"ancestor_record_offset":21768,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2828,"record_offset":22444,"parent_record_offset":21768,"ancestor_record_offset":21768,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":2828,"record_offset":21948,"parent_record_offset":21768,"ancestor_record_offset":21768,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":2828,"record_offset":21808,"parent_record_offset":21768,"ancestor_record_offset":21768,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2828,"record_offset":22276,"parent_record_offset":22248,"ancestor_record_offset":21768,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2828,"record_offset":22328,"parent_record_offset":22304,"ancestor_record_offset":21768,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2828,"record_offset":22304,"parent_record_offset":22248,"ancestor_record_offset":21768,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":2828,"record_offset":22352,"parent_record_offset":22328,"ancestor_record_offset":21768,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2828,"record_offset":22204,"parent_record_offset":22180,"ancestor_record_offset":21768,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2828,"record_offset":22376,"parent_record_offset":22352,"ancestor_record_offset":21768,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2828,"record_offset":21832,"parent_record_offset":21808,"ancestor_record_offset":21768,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2828,"record_offset":21856,"parent_record_offset":21832,"ancestor_record_offset":21768,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2828,"record_offset":22004,"parent_record_offset":21976,"ancestor_record_offset":21768,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":2828,"record_offset":22052,"parent_record_offset":22028,"ancestor_record_offset":21768,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":2828,"record_offset":22128,"parent_record_offset":22100,"ancestor_record_offset":21768,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":2828,"record_offset":22248,"parent_record_offset":22100,"ancestor_record_offset":21768,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":2828,"record_offset":22028,"parent_record_offset":22004,"ancestor_record_offset":21768,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":2828,"record_offset":22152,"parent_record_offset":22128,"ancestor_record_offset":21768,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":2828,"record_offset":22180,"parent_record_offset":22152,"ancestor_record_offset":21768,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":2828,"record_offset":22100,"parent_record_offset":22076,"ancestor_record_offset":21768,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":2828,"record_offset":22076,"parent_record_offset":22052,"ancestor_record_offset":21768,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::compile_texture_assets::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 139
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B690
// 首条位置: {"module":3930,"inlinee":112272}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3930,"record_offset":904,"parent_record_offset":524,"ancestor_record_offset":524,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3930,"record_offset":552,"parent_record_offset":524,"ancestor_record_offset":524,"inlinee":7345,"source_line":1160,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3930,"record_offset":844,"parent_record_offset":820,"ancestor_record_offset":524,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3930,"record_offset":820,"parent_record_offset":764,"ancestor_record_offset":524,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3930,"record_offset":792,"parent_record_offset":764,"ancestor_record_offset":524,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3930,"record_offset":764,"parent_record_offset":736,"ancestor_record_offset":524,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3930,"record_offset":664,"parent_record_offset":576,"ancestor_record_offset":524,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3930,"record_offset":576,"parent_record_offset":552,"ancestor_record_offset":524,"inlinee":7346,"source_line":1895,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3930,"record_offset":608,"parent_record_offset":576,"ancestor_record_offset":524,"inlinee":7347,"source_line":1934,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3930,"record_offset":632,"parent_record_offset":608,"ancestor_record_offset":524,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::compile_texture_assets::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 143
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B63F
// 首条位置: {"module":3916,"inlinee":112191}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3916,"record_offset":8868,"parent_record_offset":8788,"ancestor_record_offset":8788,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03916.jsonl.gz"}
//   [pdb_inline_direct] relative_path::impl$82::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"relative-path","version":"1.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/relative-path-1.9.0/src/lib.rs
//     evidence={"module":3916,"record_offset":8836,"parent_record_offset":8788,"ancestor_record_offset":8788,"inlinee":90266,"source_line":631,"archive":"_pdb/A/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3916,"record_offset":9048,"parent_record_offset":9024,"ancestor_record_offset":8788,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3916,"record_offset":9024,"parent_record_offset":8968,"ancestor_record_offset":8788,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3916,"record_offset":8996,"parent_record_offset":8968,"ancestor_record_offset":8788,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3916,"record_offset":8968,"parent_record_offset":8940,"ancestor_record_offset":8788,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3916,"record_offset":8892,"parent_record_offset":8868,"ancestor_record_offset":8788,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03916.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::compile_texture_asset::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 223
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B63B
// 首条位置: {"module":3915,"inlinee":112187}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3915,"record_offset":3228,"parent_record_offset":3208,"ancestor_record_offset":3208,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/03915.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3915,"record_offset":3276,"parent_record_offset":3252,"ancestor_record_offset":3208,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/03915.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3915,"record_offset":3252,"parent_record_offset":3228,"ancestor_record_offset":3208,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/03915.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::resize_image
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 239
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x160C9
// 首条位置: {"module":2828,"inlinee":90313}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::pad_image
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 298
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x160CA
// 首条位置: {"module":2828,"inlinee":90314}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset_pipe::texture::resize_image
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":2828,"record_offset":46768,"parent_record_offset":46676,"ancestor_record_offset":46676,"inlinee":90313,"source_line":239,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":2828,"record_offset":47080,"parent_record_offset":46676,"ancestor_record_offset":46676,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2828,"record_offset":47544,"parent_record_offset":46676,"ancestor_record_offset":46676,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] image::image::GenericImage::copy_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/image.rs
//     evidence={"module":2828,"record_offset":46796,"parent_record_offset":46676,"ancestor_record_offset":46676,"inlinee":68050,"source_line":963,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$60::lt
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2828,"record_offset":46956,"parent_record_offset":46864,"ancestor_record_offset":46676,"inlinee":5261,"source_line":1553,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$39::forward_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":2828,"record_offset":46896,"parent_record_offset":46864,"ancestor_record_offset":46676,"inlinee":5988,"source_line":207,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":2828,"record_offset":46864,"parent_record_offset":46836,"ancestor_record_offset":46676,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":2828,"record_offset":46836,"parent_record_offset":46796,"ancestor_record_offset":46676,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::unchecked_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2828,"record_offset":46920,"parent_record_offset":46896,"ancestor_record_offset":46676,"inlinee":5989,"source_line":520,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::process_regular_image
// 记录形式: compiled_procedure; instances=1
// 源码行: 335
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2828,"record_offset":43552,"rva":25248032,"va":5393957152}
// 编译实例 RVAs: 0x1814120
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21209249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393957152,"raw_hex":"e85f5fbcfe","size":5,"soft_fail":false,"target_va":5372749712,"va":5393958956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=1
//     target_aliases=28 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18786187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393957152,"raw_hex":"e87558e1fe","size":5,"soft_fail":false,"target_va":5375171056,"va":5393957238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141c8c490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4686428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393957152,"raw_hex":"e85c824700","size":5,"soft_fail":false,"target_va":5398643856,"va":5393957423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141c93700
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4714510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393957152,"raw_hex":"e80ef04700","size":5,"soft_fail":false,"target_va":5398673152,"va":5393958637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141c93b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4715748","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393957152,"raw_hex":"e8e4f44700","size":5,"soft_fail":false,"target_va":5398674224,"va":5393958471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e00
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9636541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393957152,"raw_hex":"e8bd0a9300","size":5,"soft_fail":false,"target_va":5403594240,"va":5393957694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9590277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393957152,"raw_hex":"e805569200","size":5,"soft_fail":false,"target_va":5403549296,"va":5393959014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset_pipe::texture::process_regular_image::closure$3
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393957152,"raw_hex":"e817070000","size":5,"soft_fail":false,"target_va":5393960336,"va":5393958516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9019653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393957152,"raw_hex":"e805a18900","size":5,"soft_fail":false,"target_va":5402978880,"va":5393959222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t9019965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393957152,"raw_hex":"e83da28900","size":5,"soft_fail":false,"target_va":5402979152,"va":5393959182,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<image::dynimage::DynamicImage> >
//     owner=rust_standard_library; count=9
//     evidence={"assembly":"\tcallq\t-31427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393957152,"raw_hex":"e83d85ffff","size":5,"soft_fail":false,"target_va":5393927248,"va":5393958670,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9021060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393957152,"raw_hex":"e884a68900","size":5,"soft_fail":false,"target_va":5402980304,"va":5393959239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::dynimage::DynamicImage::into_rgba32f
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17907386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393957152,"raw_hex":"e846c1eefe","size":5,"soft_fail":false,"target_va":5376050176,"va":5393957557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::dynimage::DynamicImage::resize_exact
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17907997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393957152,"raw_hex":"e8e3beeefe","size":5,"soft_fail":false,"target_va":5376050288,"va":5393958280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::dynimage::DynamicImage::to_rgba8
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17908203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393957152,"raw_hex":"e815beeefe","size":5,"soft_fail":false,"target_va":5376049776,"va":5393957974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::io::reader::Reader$LT$R$GT$::decode
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4713279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393957152,"raw_hex":"e83feb4700","size":5,"soft_fail":false,"target_va":5398670688,"va":5393957404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::io::reader::Reader$LT$std..io..buffered..bufreader..BufReader$LT$std..fs..File$GT$$GT$::open_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17878941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393957152,"raw_hex":"e86330effe","size":5,"soft_fail":false,"target_va":5376078320,"va":5393957256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rkyv::util::to_bytes<asset::texture::blob::TextureAssetBlob,256>
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-31539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393957152,"raw_hex":"e8cd84ffff","size":5,"soft_fail":false,"target_va":5393927360,"va":5393958894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::texture::desc::impl$3::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":2828,"record_offset":44952,"parent_record_offset":43552,"ancestor_record_offset":43552,"inlinee":90298,"source_line":4,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] asset_pipe::texture::apply_transfer_function
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":2828,"record_offset":44096,"parent_record_offset":43552,"ancestor_record_offset":43552,"inlinee":90306,"source_line":488,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] asset_pipe::texture::resize_image
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":2828,"record_offset":45128,"parent_record_offset":43552,"ancestor_record_offset":43552,"inlinee":90313,"source_line":239,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":2828,"record_offset":44896,"parent_record_offset":43552,"ancestor_record_offset":43552,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":2828,"record_offset":45156,"parent_record_offset":43552,"ancestor_record_offset":43552,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::next_multiple_of
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2828,"record_offset":45040,"parent_record_offset":43552,"ancestor_record_offset":43552,"inlinee":90312,"source_line":2641,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2828,"record_offset":45068,"parent_record_offset":43552,"ancestor_record_offset":43552,"inlinee":12684,"source_line":1444,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2828,"record_offset":45360,"parent_record_offset":43552,"ancestor_record_offset":43552,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2828,"record_offset":44032,"parent_record_offset":43552,"ancestor_record_offset":43552,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2828,"record_offset":44068,"parent_record_offset":43552,"ancestor_record_offset":43552,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] image::dynimage::open
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/dynimage.rs
//     evidence={"module":2828,"record_offset":43720,"parent_record_offset":43552,"ancestor_record_offset":43552,"inlinee":7596,"source_line":1083,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2828,"record_offset":45516,"parent_record_offset":45492,"ancestor_record_offset":43552,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2828,"record_offset":45492,"parent_record_offset":45436,"ancestor_record_offset":43552,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":45464,"parent_record_offset":45436,"ancestor_record_offset":43552,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":45436,"parent_record_offset":45408,"ancestor_record_offset":43552,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::srgb_eotf
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":2828,"record_offset":44692,"parent_record_offset":44096,"ancestor_record_offset":43552,"inlinee":90307,"source_line":508,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$60::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2828,"record_offset":45220,"parent_record_offset":45184,"ancestor_record_offset":43552,"inlinee":5261,"source_line":1553,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":2828,"record_offset":43768,"parent_record_offset":43744,"ancestor_record_offset":43552,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$39::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":2828,"record_offset":45252,"parent_record_offset":45184,"ancestor_record_offset":43552,"inlinee":5988,"source_line":207,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":2828,"record_offset":45184,"parent_record_offset":45156,"ancestor_record_offset":43552,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2828,"record_offset":44288,"parent_record_offset":44264,"ancestor_record_offset":43552,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2828,"record_offset":44312,"parent_record_offset":44288,"ancestor_record_offset":43552,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2828,"record_offset":46316,"parent_record_offset":46284,"ancestor_record_offset":43552,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2828,"record_offset":45276,"parent_record_offset":45252,"ancestor_record_offset":43552,"inlinee":5989,"source_line":520,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2828,"record_offset":44240,"parent_record_offset":44212,"ancestor_record_offset":43552,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2828,"record_offset":44476,"parent_record_offset":44172,"ancestor_record_offset":43552,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2828,"record_offset":45384,"parent_record_offset":45360,"ancestor_record_offset":43552,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2828,"record_offset":43968,"parent_record_offset":43720,"ancestor_record_offset":43552,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2828,"record_offset":44000,"parent_record_offset":43720,"ancestor_record_offset":43552,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2828,"record_offset":44512,"parent_record_offset":44172,"ancestor_record_offset":43552,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2828,"record_offset":44568,"parent_record_offset":44540,"ancestor_record_offset":43552,"inlinee":5972,"source_line":424,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$5::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2828,"record_offset":44540,"parent_record_offset":44512,"ancestor_record_offset":43552,"inlinee":5971,"source_line":468,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$97::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":2828,"record_offset":44652,"parent_record_offset":44620,"ancestor_record_offset":43552,"inlinee":5222,"source_line":2039,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::image_buffer_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":2828,"record_offset":44212,"parent_record_offset":44172,"ancestor_record_offset":43552,"inlinee":7583,"source_line":807,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::inner_pixels_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":2828,"record_offset":44172,"parent_record_offset":44140,"ancestor_record_offset":43552,"inlinee":68023,"source_line":892,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::pixels_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":2828,"record_offset":44140,"parent_record_offset":44096,"ancestor_record_offset":43552,"inlinee":13331,"source_line":898,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":2828,"record_offset":44264,"parent_record_offset":44240,"ancestor_record_offset":43552,"inlinee":7585,"source_line":809,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$1
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":2828,"record_offset":44380,"parent_record_offset":44356,"ancestor_record_offset":43552,"inlinee":7586,"source_line":810,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$5::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":2828,"record_offset":44620,"parent_record_offset":44096,"ancestor_record_offset":43552,"inlinee":13334,"source_line":98,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] image::dynimage::impl$10::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/dynimage.rs
//     evidence={"module":2828,"record_offset":44920,"parent_record_offset":44896,"ancestor_record_offset":43552,"inlinee":90309,"source_line":929,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] image::dynimage::impl$4::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/dynimage.rs
//     evidence={"module":2828,"record_offset":45008,"parent_record_offset":44984,"ancestor_record_offset":43552,"inlinee":90311,"source_line":893,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] image::io::reader::Reader::open
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/io/reader.rs
//     evidence={"module":2828,"record_offset":43744,"parent_record_offset":43720,"ancestor_record_offset":43552,"inlinee":7590,"source_line":293,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powf
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2828,"record_offset":44728,"parent_record_offset":44692,"ancestor_record_offset":43552,"inlinee":6118,"source_line":412,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":2828,"record_offset":43864,"parent_record_offset":43840,"ancestor_record_offset":43552,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":2828,"record_offset":43912,"parent_record_offset":43888,"ancestor_record_offset":43552,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":2828,"record_offset":43888,"parent_record_offset":43864,"ancestor_record_offset":43552,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":43840,"parent_record_offset":43816,"ancestor_record_offset":43552,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":43816,"parent_record_offset":43792,"ancestor_record_offset":43552,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":43792,"parent_record_offset":43768,"ancestor_record_offset":43552,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::process_regular_image::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 336
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B638
// 首条位置: {"module":3915,"inlinee":112184}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3915,"record_offset":1976,"parent_record_offset":1956,"ancestor_record_offset":1956,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/03915.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3915,"record_offset":2024,"parent_record_offset":2000,"ancestor_record_offset":1956,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/03915.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3915,"record_offset":2000,"parent_record_offset":1976,"ancestor_record_offset":1956,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/03915.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::process_regular_image::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 403
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B693
// 首条位置: {"module":3930,"inlinee":112275}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3930,"record_offset":6120,"parent_record_offset":6052,"ancestor_record_offset":6052,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_direct] image::buffer_::impl$41::deref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":3930,"record_offset":6092,"parent_record_offset":6052,"ancestor_record_offset":6052,"inlinee":9742,"source_line":1110,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3930,"record_offset":6372,"parent_record_offset":6348,"ancestor_record_offset":6052,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3930,"record_offset":6396,"parent_record_offset":6372,"ancestor_record_offset":6052,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3930,"record_offset":6348,"parent_record_offset":6312,"ancestor_record_offset":6052,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3930,"record_offset":6312,"parent_record_offset":6284,"ancestor_record_offset":6052,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3930,"record_offset":6284,"parent_record_offset":6256,"ancestor_record_offset":6052,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3930,"record_offset":6468,"parent_record_offset":6284,"ancestor_record_offset":6052,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3930,"record_offset":6216,"parent_record_offset":6184,"ancestor_record_offset":6052,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3930,"record_offset":6184,"parent_record_offset":6152,"ancestor_record_offset":6052,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3930,"record_offset":6152,"parent_record_offset":6120,"ancestor_record_offset":6052,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3930,"record_offset":6256,"parent_record_offset":6216,"ancestor_record_offset":6052,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3930,"record_offset":6532,"parent_record_offset":6508,"ancestor_record_offset":6052,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3930,"record_offset":6508,"parent_record_offset":6216,"ancestor_record_offset":6052,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3930,"record_offset":6424,"parent_record_offset":6396,"ancestor_record_offset":6052,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::process_regular_image::closure$3
// 记录形式: compiled_procedure; instances=1
// 源码行: 412
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2828,"record_offset":46428,"rva":25251216,"va":5393960336}
// 编译实例 RVAs: 0x1814d90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$image..dynimage..DynamicImage$u20$as$u20$image..image..GenericImage$GT$::put_pixel
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-17908415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393960336,"raw_hex":"e841bdeefe","size":5,"soft_fail":false,"target_va":5376052256,"va":5393960666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$image..dynimage..DynamicImage$u20$as$u20$image..image..GenericImageView$GT$::get_pixel
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-17909886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393960336,"raw_hex":"e882b7eefe","size":5,"soft_fail":false,"target_va":5376050768,"va":5393960649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t9018186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393960336,"raw_hex":"e84a9b8900","size":5,"soft_fail":false,"target_va":5402979152,"va":5393960961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<image::dynimage::DynamicImage> >
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t-33664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393960336,"raw_hex":"e8807cffff","size":5,"soft_fail":false,"target_va":5393927248,"va":5393960907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<image::error::ImageError> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-34357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393960336,"raw_hex":"e8cb79ffff","size":5,"soft_fail":false,"target_va":5393926912,"va":5393961264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9019055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393960336,"raw_hex":"e8af9e8900","size":5,"soft_fail":false,"target_va":5402980128,"va":5393961068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::dynimage::DynamicImage::new_rgba8
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17910946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393960336,"raw_hex":"e85eb3eefe","size":5,"soft_fail":false,"target_va":5376049616,"va":5393960557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::dynimage::DynamicImage::resize_exact
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-17910152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393960336,"raw_hex":"e878b6eefe","size":5,"soft_fail":false,"target_va":5376050288,"va":5393960435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset_pipe::texture::pad_image
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":2828,"record_offset":46676,"parent_record_offset":46428,"ancestor_record_offset":46428,"inlinee":90314,"source_line":298,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] asset_pipe::texture::resize_image
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":2828,"record_offset":46576,"parent_record_offset":46428,"ancestor_record_offset":46428,"inlinee":90313,"source_line":239,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::next_multiple_of
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2828,"record_offset":46604,"parent_record_offset":46428,"ancestor_record_offset":46428,"inlinee":90312,"source_line":2641,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::resize_image
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":2828,"record_offset":46768,"parent_record_offset":46676,"ancestor_record_offset":46428,"inlinee":90313,"source_line":239,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$60::lt
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2828,"record_offset":46956,"parent_record_offset":46864,"ancestor_record_offset":46428,"inlinee":5261,"source_line":1553,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$39::forward_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":2828,"record_offset":46896,"parent_record_offset":46864,"ancestor_record_offset":46428,"inlinee":5988,"source_line":207,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":2828,"record_offset":46864,"parent_record_offset":46836,"ancestor_record_offset":46428,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":2828,"record_offset":46836,"parent_record_offset":46796,"ancestor_record_offset":46428,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::unchecked_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2828,"record_offset":46920,"parent_record_offset":46896,"ancestor_record_offset":46428,"inlinee":5989,"source_line":520,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2828,"record_offset":47544,"parent_record_offset":46676,"ancestor_record_offset":46428,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] image::image::GenericImage::copy_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/image.rs
//     evidence={"module":2828,"record_offset":46796,"parent_record_offset":46676,"ancestor_record_offset":46428,"inlinee":68050,"source_line":963,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::process_regular_image::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 428
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B691
// 首条位置: {"module":3930,"inlinee":112273}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3930,"record_offset":2896,"parent_record_offset":1952,"ancestor_record_offset":1952,"inlinee":4270,"source_line":2537,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3930,"record_offset":1992,"parent_record_offset":1952,"ancestor_record_offset":1952,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_direct] image::buffer_::impl$41::deref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":3930,"record_offset":2868,"parent_record_offset":1952,"ancestor_record_offset":1952,"inlinee":9742,"source_line":1110,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3930,"record_offset":2548,"parent_record_offset":2524,"ancestor_record_offset":1952,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3930,"record_offset":2524,"parent_record_offset":2468,"ancestor_record_offset":1952,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3930,"record_offset":2228,"parent_record_offset":2200,"ancestor_record_offset":1952,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3930,"record_offset":2496,"parent_record_offset":2468,"ancestor_record_offset":1952,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3930,"record_offset":2200,"parent_record_offset":2168,"ancestor_record_offset":1952,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3930,"record_offset":2168,"parent_record_offset":2144,"ancestor_record_offset":1952,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3930,"record_offset":2468,"parent_record_offset":2444,"ancestor_record_offset":1952,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3930,"record_offset":2108,"parent_record_offset":2076,"ancestor_record_offset":1952,"inlinee":4273,"source_line":2090,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3930,"record_offset":2280,"parent_record_offset":2108,"ancestor_record_offset":1952,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3930,"record_offset":2144,"parent_record_offset":2108,"ancestor_record_offset":1952,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3930,"record_offset":2020,"parent_record_offset":1992,"ancestor_record_offset":1952,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3930,"record_offset":2372,"parent_record_offset":2348,"ancestor_record_offset":1952,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3930,"record_offset":2420,"parent_record_offset":2396,"ancestor_record_offset":1952,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$2::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3930,"record_offset":2076,"parent_record_offset":1992,"ancestor_record_offset":1952,"inlinee":4434,"source_line":31,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3930,"record_offset":2920,"parent_record_offset":2896,"ancestor_record_offset":1952,"inlinee":4272,"source_line":53,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":3930,"record_offset":2044,"parent_record_offset":2020,"ancestor_record_offset":1952,"inlinee":4174,"source_line":70,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3930,"record_offset":2348,"parent_record_offset":2076,"ancestor_record_offset":1952,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3930,"record_offset":2308,"parent_record_offset":2108,"ancestor_record_offset":1952,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$2,alloc::vec::Vec<u8,alloc::alloc::Global> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 469
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3930,"record_offset":5212,"rva":29965104,"va":5398674224}
// 编译实例 RVAs: 0x1c93b30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25924901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398674224,"raw_hex":"e8db6a74fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5398674592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25925007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398674224,"raw_hex":"e8716a74fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5398674714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4918676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398674224,"raw_hex":"e8940d4b00","size":5,"soft_fail":false,"target_va":5403593360,"va":5398674679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4303304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398674224,"raw_hex":"e8c8a94100","size":5,"soft_fail":false,"target_va":5402978144,"va":5398674835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-21147705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398674224,"raw_hex":"e8c74fbdfe","size":5,"soft_fail":false,"target_va":5377527120,"va":5398674820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset_pipe::texture::compress
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-4713063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398674224,"raw_hex":"e89915b8ff","size":5,"soft_fail":false,"target_va":5393961424,"va":5398674482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset_pipe::texture::srgb_oetf
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-4712990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398674224,"raw_hex":"e8e215b8ff","size":5,"soft_fail":false,"target_va":5393961360,"va":5398674345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4304095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398674224,"raw_hex":"e8dfac4100","size":5,"soft_fail":false,"target_va":5402978880,"va":5398674780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4305498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398674224,"raw_hex":"e85ab24100","size":5,"soft_fail":false,"target_va":5402980304,"va":5398674801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::dynimage::DynamicImage::into_rgba8
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22624398","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398674224,"raw_hex":"e872c7a6fe","size":5,"soft_fail":false,"target_va":5376050064,"va":5398674457,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::dynimage::DynamicImage::to_rgba32f
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22624361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398674224,"raw_hex":"e897c7a6fe","size":5,"soft_fail":false,"target_va":5376049920,"va":5398674276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset_pipe::texture::apply_transfer_function
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":3930,"record_offset":5448,"parent_record_offset":5212,"ancestor_record_offset":5212,"inlinee":90306,"source_line":488,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_direct] asset_pipe::texture::process_regular_image::closure$2
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":3930,"record_offset":6052,"parent_record_offset":5212,"ancestor_record_offset":5212,"inlinee":112275,"source_line":403,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3930,"record_offset":6836,"parent_record_offset":5212,"ancestor_record_offset":5212,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3930,"record_offset":6584,"parent_record_offset":5212,"ancestor_record_offset":5212,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_direct] image::dynimage::DynamicImage::as_rgba8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/dynimage.rs
//     evidence={"module":3930,"record_offset":6808,"parent_record_offset":5212,"ancestor_record_offset":5212,"inlinee":112274,"source_line":391,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_direct] image::dynimage::impl$10::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/dynimage.rs
//     evidence={"module":3930,"record_offset":6024,"parent_record_offset":5212,"ancestor_record_offset":5212,"inlinee":90309,"source_line":929,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3930,"record_offset":6372,"parent_record_offset":6348,"ancestor_record_offset":5212,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3930,"record_offset":6396,"parent_record_offset":6372,"ancestor_record_offset":5212,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3930,"record_offset":6760,"parent_record_offset":6736,"ancestor_record_offset":5212,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3930,"record_offset":6348,"parent_record_offset":6312,"ancestor_record_offset":5212,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3930,"record_offset":6736,"parent_record_offset":6672,"ancestor_record_offset":5212,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3930,"record_offset":6704,"parent_record_offset":6672,"ancestor_record_offset":5212,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3930,"record_offset":6312,"parent_record_offset":6284,"ancestor_record_offset":5212,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3930,"record_offset":6284,"parent_record_offset":6256,"ancestor_record_offset":5212,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3930,"record_offset":6468,"parent_record_offset":6284,"ancestor_record_offset":5212,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3930,"record_offset":6672,"parent_record_offset":6640,"ancestor_record_offset":5212,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3930,"record_offset":6216,"parent_record_offset":6184,"ancestor_record_offset":5212,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3930,"record_offset":6184,"parent_record_offset":6152,"ancestor_record_offset":5212,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3930,"record_offset":6120,"parent_record_offset":6052,"ancestor_record_offset":5212,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3930,"record_offset":6152,"parent_record_offset":6120,"ancestor_record_offset":5212,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3930,"record_offset":6256,"parent_record_offset":6216,"ancestor_record_offset":5212,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3930,"record_offset":6532,"parent_record_offset":6508,"ancestor_record_offset":5212,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3930,"record_offset":5628,"parent_record_offset":5608,"ancestor_record_offset":5212,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3930,"record_offset":5652,"parent_record_offset":5628,"ancestor_record_offset":5212,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3930,"record_offset":7752,"parent_record_offset":7720,"ancestor_record_offset":5212,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3930,"record_offset":5584,"parent_record_offset":5560,"ancestor_record_offset":5212,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3930,"record_offset":5812,"parent_record_offset":5516,"ancestor_record_offset":5212,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3930,"record_offset":6508,"parent_record_offset":6216,"ancestor_record_offset":5212,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3930,"record_offset":6612,"parent_record_offset":6584,"ancestor_record_offset":5212,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3930,"record_offset":6424,"parent_record_offset":6396,"ancestor_record_offset":5212,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3930,"record_offset":5848,"parent_record_offset":5516,"ancestor_record_offset":5212,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3930,"record_offset":5904,"parent_record_offset":5876,"ancestor_record_offset":5212,"inlinee":5972,"source_line":424,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$5::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3930,"record_offset":5876,"parent_record_offset":5848,"ancestor_record_offset":5212,"inlinee":5971,"source_line":468,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$97::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3930,"record_offset":5984,"parent_record_offset":5956,"ancestor_record_offset":5212,"inlinee":5222,"source_line":2039,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::image_buffer_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":3930,"record_offset":5560,"parent_record_offset":5516,"ancestor_record_offset":5212,"inlinee":7583,"source_line":807,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::inner_pixels_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":3930,"record_offset":5516,"parent_record_offset":5484,"ancestor_record_offset":5212,"inlinee":68023,"source_line":892,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::pixels_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":3930,"record_offset":5484,"parent_record_offset":5448,"ancestor_record_offset":5212,"inlinee":13331,"source_line":898,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":3930,"record_offset":5608,"parent_record_offset":5584,"ancestor_record_offset":5212,"inlinee":7585,"source_line":809,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$1
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":3930,"record_offset":5720,"parent_record_offset":5696,"ancestor_record_offset":5212,"inlinee":7586,"source_line":810,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$41::deref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":3930,"record_offset":6092,"parent_record_offset":6052,"ancestor_record_offset":5212,"inlinee":9742,"source_line":1110,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$5::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":3930,"record_offset":5956,"parent_record_offset":5448,"ancestor_record_offset":5212,"inlinee":13334,"source_line":98,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$4,tuple$<> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 469
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3930,"record_offset":1144,"rva":29964032,"va":5398673152}
// 编译实例 RVAs: 0x1c93700
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25923814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398673152,"raw_hex":"e81a6f74fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5398673521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4919863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398673152,"raw_hex":"e837124b00","size":5,"soft_fail":false,"target_va":5403593360,"va":5398673492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t4245605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398673152,"raw_hex":"e865c84000","size":5,"soft_fail":false,"target_va":5402919072,"va":5398673462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset_pipe::texture::compress
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-4711994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398673152,"raw_hex":"e8c619b8ff","size":5,"soft_fail":false,"target_va":5393961424,"va":5398673413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset_pipe::texture::srgb_oetf
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-4711917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398673152,"raw_hex":"e8131ab8ff","size":5,"soft_fail":false,"target_va":5393961360,"va":5398673272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4304977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398673152,"raw_hex":"e851b04100","size":5,"soft_fail":false,"target_va":5402978880,"va":5398673898,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4306380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398673152,"raw_hex":"e8ccb54100","size":5,"soft_fail":false,"target_va":5402980304,"va":5398673919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::dynimage::DynamicImage::into_rgba8
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22623324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398673152,"raw_hex":"e8a4cba6fe","size":5,"soft_fail":false,"target_va":5376050064,"va":5398673383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::dynimage::DynamicImage::to_rgba32f
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22623285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398673152,"raw_hex":"e8cbcba6fe","size":5,"soft_fail":false,"target_va":5376049920,"va":5398673200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset_pipe::texture::apply_transfer_function
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":3930,"record_offset":1348,"parent_record_offset":1144,"ancestor_record_offset":1144,"inlinee":90306,"source_line":488,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_direct] asset_pipe::texture::process_regular_image::closure$4
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":3930,"record_offset":1952,"parent_record_offset":1144,"ancestor_record_offset":1144,"inlinee":112273,"source_line":428,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3930,"record_offset":3448,"parent_record_offset":1144,"ancestor_record_offset":1144,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3930,"record_offset":3196,"parent_record_offset":1144,"ancestor_record_offset":1144,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_direct] image::dynimage::DynamicImage::as_rgba8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/dynimage.rs
//     evidence={"module":3930,"record_offset":3420,"parent_record_offset":1144,"ancestor_record_offset":1144,"inlinee":112274,"source_line":391,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_direct] image::dynimage::impl$10::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/dynimage.rs
//     evidence={"module":3930,"record_offset":1924,"parent_record_offset":1144,"ancestor_record_offset":1144,"inlinee":90309,"source_line":929,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3930,"record_offset":2548,"parent_record_offset":2524,"ancestor_record_offset":1144,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3930,"record_offset":2524,"parent_record_offset":2468,"ancestor_record_offset":1144,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3930,"record_offset":2228,"parent_record_offset":2200,"ancestor_record_offset":1144,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3930,"record_offset":2496,"parent_record_offset":2468,"ancestor_record_offset":1144,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3930,"record_offset":2200,"parent_record_offset":2168,"ancestor_record_offset":1144,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3930,"record_offset":2168,"parent_record_offset":2144,"ancestor_record_offset":1144,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3930,"record_offset":2468,"parent_record_offset":2444,"ancestor_record_offset":1144,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3930,"record_offset":2108,"parent_record_offset":2076,"ancestor_record_offset":1144,"inlinee":4273,"source_line":2090,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3930,"record_offset":2896,"parent_record_offset":1952,"ancestor_record_offset":1144,"inlinee":4270,"source_line":2537,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3930,"record_offset":2280,"parent_record_offset":2108,"ancestor_record_offset":1144,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3930,"record_offset":2144,"parent_record_offset":2108,"ancestor_record_offset":1144,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3930,"record_offset":2020,"parent_record_offset":1992,"ancestor_record_offset":1144,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3930,"record_offset":1992,"parent_record_offset":1952,"ancestor_record_offset":1144,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3930,"record_offset":2372,"parent_record_offset":2348,"ancestor_record_offset":1144,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":3930,"record_offset":2420,"parent_record_offset":2396,"ancestor_record_offset":1144,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$2::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3930,"record_offset":2076,"parent_record_offset":1992,"ancestor_record_offset":1144,"inlinee":4434,"source_line":31,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3930,"record_offset":2920,"parent_record_offset":2896,"ancestor_record_offset":1144,"inlinee":4272,"source_line":53,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":3930,"record_offset":2044,"parent_record_offset":2020,"ancestor_record_offset":1144,"inlinee":4174,"source_line":70,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3930,"record_offset":1528,"parent_record_offset":1508,"ancestor_record_offset":1144,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3930,"record_offset":1552,"parent_record_offset":1528,"ancestor_record_offset":1144,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3930,"record_offset":5100,"parent_record_offset":5068,"ancestor_record_offset":1144,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3930,"record_offset":1484,"parent_record_offset":1460,"ancestor_record_offset":1144,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3930,"record_offset":1712,"parent_record_offset":1416,"ancestor_record_offset":1144,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3930,"record_offset":2348,"parent_record_offset":2076,"ancestor_record_offset":1144,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3930,"record_offset":2308,"parent_record_offset":2108,"ancestor_record_offset":1144,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3930,"record_offset":1748,"parent_record_offset":1416,"ancestor_record_offset":1144,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3930,"record_offset":1804,"parent_record_offset":1776,"ancestor_record_offset":1144,"inlinee":5972,"source_line":424,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$5::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3930,"record_offset":1776,"parent_record_offset":1748,"ancestor_record_offset":1144,"inlinee":5971,"source_line":468,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$97::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3930,"record_offset":1884,"parent_record_offset":1856,"ancestor_record_offset":1144,"inlinee":5222,"source_line":2039,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::image_buffer_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":3930,"record_offset":1460,"parent_record_offset":1416,"ancestor_record_offset":1144,"inlinee":7583,"source_line":807,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::inner_pixels_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":3930,"record_offset":1416,"parent_record_offset":1384,"ancestor_record_offset":1144,"inlinee":68023,"source_line":892,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::pixels_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":3930,"record_offset":1384,"parent_record_offset":1348,"ancestor_record_offset":1144,"inlinee":13331,"source_line":898,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":3930,"record_offset":1508,"parent_record_offset":1484,"ancestor_record_offset":1144,"inlinee":7585,"source_line":809,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$1
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":3930,"record_offset":1620,"parent_record_offset":1596,"ancestor_record_offset":1144,"inlinee":7586,"source_line":810,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$41::deref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":3930,"record_offset":2868,"parent_record_offset":1952,"ancestor_record_offset":1144,"inlinee":9742,"source_line":1110,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$5::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":3930,"record_offset":1856,"parent_record_offset":1348,"ancestor_record_offset":1144,"inlinee":13334,"source_line":98,"archive":"_pdb/A/modules/03930.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::apply_transfer_function
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 488
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x160C2
// 首条位置: {"module":2828,"inlinee":90306}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset_pipe::texture::srgb_eotf
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":2828,"record_offset":44692,"parent_record_offset":44096,"ancestor_record_offset":44096,"inlinee":90307,"source_line":508,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] image::buffer_::ImageBuffer::pixels_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":2828,"record_offset":44140,"parent_record_offset":44096,"ancestor_record_offset":44096,"inlinee":13331,"source_line":898,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] image::buffer_::impl$5::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":2828,"record_offset":44620,"parent_record_offset":44096,"ancestor_record_offset":44096,"inlinee":13334,"source_line":98,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2828,"record_offset":44288,"parent_record_offset":44264,"ancestor_record_offset":44096,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2828,"record_offset":44312,"parent_record_offset":44288,"ancestor_record_offset":44096,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2828,"record_offset":44240,"parent_record_offset":44212,"ancestor_record_offset":44096,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2828,"record_offset":44476,"parent_record_offset":44172,"ancestor_record_offset":44096,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2828,"record_offset":44512,"parent_record_offset":44172,"ancestor_record_offset":44096,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2828,"record_offset":44568,"parent_record_offset":44540,"ancestor_record_offset":44096,"inlinee":5972,"source_line":424,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$5::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2828,"record_offset":44540,"parent_record_offset":44512,"ancestor_record_offset":44096,"inlinee":5971,"source_line":468,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$97::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":2828,"record_offset":44652,"parent_record_offset":44620,"ancestor_record_offset":44096,"inlinee":5222,"source_line":2039,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::image_buffer_len
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":2828,"record_offset":44212,"parent_record_offset":44172,"ancestor_record_offset":44096,"inlinee":7583,"source_line":807,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::inner_pixels_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":2828,"record_offset":44172,"parent_record_offset":44140,"ancestor_record_offset":44096,"inlinee":68023,"source_line":892,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":2828,"record_offset":44264,"parent_record_offset":44240,"ancestor_record_offset":44096,"inlinee":7585,"source_line":809,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$1
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":2828,"record_offset":44380,"parent_record_offset":44356,"ancestor_record_offset":44096,"inlinee":7586,"source_line":810,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::powf
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2828,"record_offset":44728,"parent_record_offset":44692,"ancestor_record_offset":44096,"inlinee":6118,"source_line":412,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::srgb_oetf
// 记录形式: compiled_procedure; instances=1
// 源码行: 500
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2828,"record_offset":47584,"rva":25252240,"va":5393961360}
// 编译实例 RVAs: 0x1815190
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9632850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393961360,"raw_hex":"e852fc9200","size":5,"soft_fail":false,"target_va":5403594240,"va":5393961385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::f32::impl$0::powf
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2828,"record_offset":47700,"parent_record_offset":47584,"ancestor_record_offset":47584,"inlinee":6118,"source_line":412,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::srgb_eotf
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 508
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x160C3
// 首条位置: {"module":2828,"inlinee":90307}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f32::impl$0::powf
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2828,"record_offset":44728,"parent_record_offset":44692,"ancestor_record_offset":44692,"inlinee":6118,"source_line":412,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::process_dds_image
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 516
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x160B1
// 首条位置: {"module":2828,"inlinee":90289}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2828,"record_offset":35664,"parent_record_offset":33784,"ancestor_record_offset":33784,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2828,"record_offset":34812,"parent_record_offset":33784,"ancestor_record_offset":33784,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$6::try_into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":2828,"record_offset":35572,"parent_record_offset":33784,"ancestor_record_offset":33784,"inlinee":4103,"source_line":797,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2828,"record_offset":36284,"parent_record_offset":33784,"ancestor_record_offset":33784,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2828,"record_offset":35156,"parent_record_offset":33784,"ancestor_record_offset":33784,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2828,"record_offset":34896,"parent_record_offset":33784,"ancestor_record_offset":33784,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2828,"record_offset":34320,"parent_record_offset":33784,"ancestor_record_offset":33784,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2828,"record_offset":35416,"parent_record_offset":33784,"ancestor_record_offset":33784,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2828,"record_offset":34220,"parent_record_offset":33784,"ancestor_record_offset":33784,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2828,"record_offset":36756,"parent_record_offset":33784,"ancestor_record_offset":33784,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] ddsfile::Dds::get_depth
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ddsfile","version":"0.5.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ddsfile-0.5.1/src/lib.rs
//     evidence={"module":2828,"record_offset":34840,"parent_record_offset":33784,"ancestor_record_offset":33784,"inlinee":90294,"source_line":249,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] ddsfile::Dds::get_num_array_layers
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ddsfile","version":"0.5.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ddsfile-0.5.1/src/lib.rs
//     evidence={"module":2828,"record_offset":35544,"parent_record_offset":33784,"ancestor_record_offset":33784,"inlinee":90305,"source_line":314,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] ddsfile::Dds::get_num_mipmap_levels
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ddsfile","version":"0.5.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ddsfile-0.5.1/src/lib.rs
//     evidence={"module":2828,"record_offset":35516,"parent_record_offset":33784,"ancestor_record_offset":33784,"inlinee":90304,"source_line":322,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":2828,"record_offset":35184,"parent_record_offset":33784,"ancestor_record_offset":33784,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":2828,"record_offset":34728,"parent_record_offset":33784,"ancestor_record_offset":33784,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":2828,"record_offset":33996,"parent_record_offset":33784,"ancestor_record_offset":33784,"inlinee":7565,"source_line":259,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] std::io::cursor::Cursor::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\io\cursor.rs
//     evidence={"module":2828,"record_offset":34252,"parent_record_offset":33784,"ancestor_record_offset":33784,"inlinee":18799,"source_line":99,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2828,"record_offset":36072,"parent_record_offset":36048,"ancestor_record_offset":33784,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2828,"record_offset":36096,"parent_record_offset":36072,"ancestor_record_offset":33784,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2828,"record_offset":34476,"parent_record_offset":34452,"ancestor_record_offset":33784,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2828,"record_offset":36048,"parent_record_offset":35996,"ancestor_record_offset":33784,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2828,"record_offset":34452,"parent_record_offset":34396,"ancestor_record_offset":33784,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2828,"record_offset":34952,"parent_record_offset":34928,"ancestor_record_offset":33784,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2828,"record_offset":35000,"parent_record_offset":34976,"ancestor_record_offset":33784,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":34424,"parent_record_offset":34396,"ancestor_record_offset":33784,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":35996,"parent_record_offset":35952,"ancestor_record_offset":33784,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":35952,"parent_record_offset":35908,"ancestor_record_offset":33784,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":36168,"parent_record_offset":35952,"ancestor_record_offset":33784,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":34396,"parent_record_offset":34368,"ancestor_record_offset":33784,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2828,"record_offset":35856,"parent_record_offset":35808,"ancestor_record_offset":33784,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2828,"record_offset":35808,"parent_record_offset":35760,"ancestor_record_offset":33784,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2828,"record_offset":35712,"parent_record_offset":35664,"ancestor_record_offset":33784,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2828,"record_offset":35760,"parent_record_offset":35712,"ancestor_record_offset":33784,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2828,"record_offset":35908,"parent_record_offset":35856,"ancestor_record_offset":33784,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::process_dds_image::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":2828,"record_offset":34928,"parent_record_offset":34896,"ancestor_record_offset":33784,"inlinee":90296,"source_line":528,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::process_dds_image::closure$3
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":2828,"record_offset":35040,"parent_record_offset":34896,"ancestor_record_offset":33784,"inlinee":90303,"source_line":529,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":2828,"record_offset":34020,"parent_record_offset":33996,"ancestor_record_offset":33784,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$88::try_from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\num.rs
//     evidence={"module":2828,"record_offset":35596,"parent_record_offset":35572,"ancestor_record_offset":33784,"inlinee":9589,"source_line":248,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2828,"record_offset":36232,"parent_record_offset":36208,"ancestor_record_offset":33784,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2828,"record_offset":34976,"parent_record_offset":34952,"ancestor_record_offset":33784,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2828,"record_offset":34864,"parent_record_offset":34840,"ancestor_record_offset":33784,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2828,"record_offset":36208,"parent_record_offset":35856,"ancestor_record_offset":33784,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2828,"record_offset":34344,"parent_record_offset":34320,"ancestor_record_offset":33784,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2828,"record_offset":36124,"parent_record_offset":36096,"ancestor_record_offset":33784,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2828,"record_offset":34752,"parent_record_offset":34728,"ancestor_record_offset":33784,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2828,"record_offset":34776,"parent_record_offset":34752,"ancestor_record_offset":33784,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":2828,"record_offset":34116,"parent_record_offset":34092,"ancestor_record_offset":33784,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":2828,"record_offset":34164,"parent_record_offset":34140,"ancestor_record_offset":33784,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":2828,"record_offset":34140,"parent_record_offset":34116,"ancestor_record_offset":33784,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":34092,"parent_record_offset":34068,"ancestor_record_offset":33784,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":34068,"parent_record_offset":34044,"ancestor_record_offset":33784,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2828,"record_offset":34044,"parent_record_offset":34020,"ancestor_record_offset":33784,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::process_dds_image::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 518
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B63C
// 首条位置: {"module":3915,"inlinee":112188}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3915,"record_offset":3876,"parent_record_offset":3856,"ancestor_record_offset":3856,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/03915.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3915,"record_offset":3924,"parent_record_offset":3900,"ancestor_record_offset":3856,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/03915.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3915,"record_offset":3900,"parent_record_offset":3876,"ancestor_record_offset":3856,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/03915.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::process_dds_image::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 520
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1B639
// 首条位置: {"module":3915,"inlinee":112185}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3915,"record_offset":2604,"parent_record_offset":2584,"ancestor_record_offset":2584,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/03915.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3915,"record_offset":2652,"parent_record_offset":2628,"ancestor_record_offset":2584,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/03915.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3915,"record_offset":2628,"parent_record_offset":2604,"ancestor_record_offset":2584,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/03915.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::process_dds_image::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 528
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x160B8
// 首条位置: {"module":2828,"inlinee":90296}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2828,"record_offset":34952,"parent_record_offset":34928,"ancestor_record_offset":34928,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2828,"record_offset":35000,"parent_record_offset":34976,"ancestor_record_offset":34928,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2828,"record_offset":34976,"parent_record_offset":34952,"ancestor_record_offset":34928,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::process_dds_image::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 529
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x160BF
// 首条位置: {"module":2828,"inlinee":90303}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2828,"record_offset":35064,"parent_record_offset":35040,"ancestor_record_offset":35040,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2828,"record_offset":35112,"parent_record_offset":35088,"ancestor_record_offset":35040,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2828,"record_offset":35088,"parent_record_offset":35064,"ancestor_record_offset":35040,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::process_image_array
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 580
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x160A0
// 首条位置: {"module":2828,"inlinee":90272}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2828,"record_offset":29536,"parent_record_offset":26232,"ancestor_record_offset":26232,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2828,"record_offset":28528,"parent_record_offset":26232,"ancestor_record_offset":26232,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::array::equality::impl$0::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":2828,"record_offset":28132,"parent_record_offset":26232,"ancestor_record_offset":26232,"inlinee":8003,"source_line":14,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2828,"record_offset":28216,"parent_record_offset":26232,"ancestor_record_offset":26232,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":2828,"record_offset":26544,"parent_record_offset":26232,"ancestor_record_offset":26232,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2828,"record_offset":26888,"parent_record_offset":26232,"ancestor_record_offset":26232,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2828,"record_offset":29500,"parent_record_offset":26232,"ancestor_record_offset":26232,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2828,"record_offset":27896,"parent_record_offset":26232,"ancestor_record_offset":26232,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2828,"record_offset":26768,"parent_record_offset":26232,"ancestor_record_offset":26232,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2828,"record_offset":26652,"parent_record_offset":26232,"ancestor_record_offset":26232,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":2828,"record_offset":30540,"parent_record_offset":26232,"ancestor_record_offset":26232,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2828,"record_offset":26460,"parent_record_offset":26232,"ancestor_record_offset":26232,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] rkyv::util::archived_root
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/util/mod.rs
//     evidence={"module":2828,"record_offset":26684,"parent_record_offset":26232,"ancestor_record_offset":26232,"inlinee":90274,"source_line":147,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] rkyv::vec::impl$4::deref
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/vec/mod.rs
//     evidence={"module":2828,"record_offset":26804,"parent_record_offset":26232,"ancestor_record_offset":26232,"inlinee":90277,"source_line":207,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2828,"record_offset":27228,"parent_record_offset":27204,"ancestor_record_offset":26232,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2828,"record_offset":27252,"parent_record_offset":27228,"ancestor_record_offset":26232,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2828,"record_offset":28080,"parent_record_offset":28056,"ancestor_record_offset":26232,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2828,"record_offset":27204,"parent_record_offset":27168,"ancestor_record_offset":26232,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2828,"record_offset":28056,"parent_record_offset":28000,"ancestor_record_offset":26232,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2828,"record_offset":29584,"parent_record_offset":29560,"ancestor_record_offset":26232,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":28028,"parent_record_offset":28000,"ancestor_record_offset":26232,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":28692,"parent_record_offset":28664,"ancestor_record_offset":26232,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":28664,"parent_record_offset":28640,"ancestor_record_offset":26232,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":27168,"parent_record_offset":27136,"ancestor_record_offset":26232,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":27136,"parent_record_offset":27104,"ancestor_record_offset":26232,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":27324,"parent_record_offset":27136,"ancestor_record_offset":26232,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":28000,"parent_record_offset":27972,"ancestor_record_offset":26232,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2828,"record_offset":27400,"parent_record_offset":27364,"ancestor_record_offset":26232,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2828,"record_offset":28640,"parent_record_offset":28600,"ancestor_record_offset":26232,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2828,"record_offset":27072,"parent_record_offset":27020,"ancestor_record_offset":26232,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2828,"record_offset":27104,"parent_record_offset":27072,"ancestor_record_offset":26232,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2828,"record_offset":26932,"parent_record_offset":26888,"ancestor_record_offset":26232,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2828,"record_offset":27672,"parent_record_offset":27624,"ancestor_record_offset":26232,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":2828,"record_offset":27784,"parent_record_offset":27672,"ancestor_record_offset":26232,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":2828,"record_offset":28760,"parent_record_offset":28600,"ancestor_record_offset":26232,"inlinee":5454,"source_line":13,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":2828,"record_offset":29332,"parent_record_offset":29304,"ancestor_record_offset":26232,"inlinee":5437,"source_line":30,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":2828,"record_offset":27364,"parent_record_offset":27020,"ancestor_record_offset":26232,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":2828,"record_offset":26976,"parent_record_offset":26932,"ancestor_record_offset":26232,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":2828,"record_offset":27020,"parent_record_offset":26976,"ancestor_record_offset":26232,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::blob::impl$3::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/blob.rs
//     evidence={"module":2828,"record_offset":28240,"parent_record_offset":28216,"ancestor_record_offset":26232,"inlinee":90280,"source_line":5,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::blob::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/blob.rs
//     evidence={"module":2828,"record_offset":28296,"parent_record_offset":28272,"ancestor_record_offset":26232,"inlinee":90282,"source_line":196,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":2828,"record_offset":28180,"parent_record_offset":28156,"ancestor_record_offset":26232,"inlinee":6198,"source_line":147,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":2828,"record_offset":28156,"parent_record_offset":28132,"ancestor_record_offset":26232,"inlinee":6955,"source_line":153,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\copied.rs
//     evidence={"module":2828,"record_offset":27576,"parent_record_offset":27508,"ancestor_record_offset":26232,"inlinee":17354,"source_line":32,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\copied.rs
//     evidence={"module":2828,"record_offset":27472,"parent_record_offset":27436,"ancestor_record_offset":26232,"inlinee":17352,"source_line":73,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2828,"record_offset":27436,"parent_record_offset":27400,"ancestor_record_offset":26232,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2828,"record_offset":27624,"parent_record_offset":27576,"ancestor_record_offset":26232,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2828,"record_offset":29188,"parent_record_offset":28852,"ancestor_record_offset":26232,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2828,"record_offset":28716,"parent_record_offset":28692,"ancestor_record_offset":26232,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2828,"record_offset":29560,"parent_record_offset":29536,"ancestor_record_offset":26232,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2828,"record_offset":26732,"parent_record_offset":26708,"ancestor_record_offset":26232,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2828,"record_offset":27924,"parent_record_offset":27896,"ancestor_record_offset":26232,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2828,"record_offset":26508,"parent_record_offset":26484,"ancestor_record_offset":26232,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2828,"record_offset":27828,"parent_record_offset":27508,"ancestor_record_offset":26232,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2828,"record_offset":26612,"parent_record_offset":26572,"ancestor_record_offset":26232,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2828,"record_offset":27280,"parent_record_offset":27252,"ancestor_record_offset":26232,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2828,"record_offset":27732,"parent_record_offset":27672,"ancestor_record_offset":26232,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":2828,"record_offset":26484,"parent_record_offset":26460,"ancestor_record_offset":26232,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2828,"record_offset":27508,"parent_record_offset":27472,"ancestor_record_offset":26232,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2828,"record_offset":26572,"parent_record_offset":26544,"ancestor_record_offset":26232,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/rel_ptr/mod.rs
//     evidence={"module":2828,"record_offset":28428,"parent_record_offset":28404,"ancestor_record_offset":26232,"inlinee":25179,"source_line":212,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::offset
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/rel_ptr/mod.rs
//     evidence={"module":2828,"record_offset":28452,"parent_record_offset":28428,"ancestor_record_offset":26232,"inlinee":25180,"source_line":200,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RelPtr::as_ptr
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/rel_ptr/mod.rs
//     evidence={"module":2828,"record_offset":28404,"parent_record_offset":28376,"ancestor_record_offset":26232,"inlinee":25177,"source_line":418,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::impl$18::to_isize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/rel_ptr/mod.rs
//     evidence={"module":2828,"record_offset":28476,"parent_record_offset":28452,"ancestor_record_offset":26232,"inlinee":25182,"source_line":110,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::util::aligned_vec::impl$0::drop
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/util/aligned_vec.rs
//     evidence={"module":2828,"record_offset":29432,"parent_record_offset":29404,"ancestor_record_offset":26232,"inlinee":90284,"source_line":22,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::util::archived_value
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/util/mod.rs
//     evidence={"module":2828,"record_offset":26708,"parent_record_offset":26684,"ancestor_record_offset":26232,"inlinee":90275,"source_line":63,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::as_ptr
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/vec/mod.rs
//     evidence={"module":2828,"record_offset":28376,"parent_record_offset":28352,"ancestor_record_offset":26232,"inlinee":25175,"source_line":32,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::as_slice
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/vec/mod.rs
//     evidence={"module":2828,"record_offset":26828,"parent_record_offset":26804,"ancestor_record_offset":26232,"inlinee":25174,"source_line":50,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::len
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-0e2f7fb44ee22773/031d581/rkyv/src/vec/mod.rs
//     evidence={"module":2828,"record_offset":26852,"parent_record_offset":26828,"ancestor_record_offset":26232,"inlinee":90278,"source_line":38,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::impl$2::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 666
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2828,"record_offset":49080,"rva":25253008,"va":5393962128}
// 编译实例 RVAs: 0x1815490
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140876b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-16378246","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5393962128,"raw_hex":"e97a1606ff","size":5,"soft_fail":false,"target_va":5377583920,"va":5393962161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::compress
// 记录形式: compiled_procedure; instances=1
// 源码行: 687
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2828,"record_offset":47732,"rva":25252304,"va":5393961424}
// 编译实例 RVAs: 0x18151d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21212389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393961424,"raw_hex":"e81b53bcfe","size":5,"soft_fail":false,"target_va":5372749712,"va":5393962096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da7b0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21211885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393961424,"raw_hex":"e81355bcfe","size":5,"soft_fail":false,"target_va":5372749744,"va":5393961624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9016497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393961424,"raw_hex":"e8b1948900","size":5,"soft_fail":false,"target_va":5402978144,"va":5393961642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9016853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393961424,"raw_hex":"e815968900","size":5,"soft_fail":false,"target_va":5402978880,"va":5393962022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9018262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393961424,"raw_hex":"e8969b8900","size":5,"soft_fail":false,"target_va":5402980304,"va":5393962037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] intel_tex_2::bc5::compress_blocks_into
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4714380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393961424,"raw_hex":"e88cef4700","size":5,"soft_fail":false,"target_va":5398676256,"va":5393961871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] intel_tex_2::bc7::compress_blocks_into
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4714875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393961424,"raw_hex":"e87bf14700","size":5,"soft_fail":false,"target_va":5398676848,"va":5393961968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16608045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393961424,"raw_hex":"e8d39402ff","size":5,"soft_fail":false,"target_va":5377353776,"va":5393961816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2828,"record_offset":48504,"parent_record_offset":47732,"ancestor_record_offset":47732,"inlinee":4220,"source_line":2685,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2828,"record_offset":48820,"parent_record_offset":47732,"ancestor_record_offset":47732,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2828,"record_offset":48848,"parent_record_offset":47732,"ancestor_record_offset":47732,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2828,"record_offset":48336,"parent_record_offset":47732,"ancestor_record_offset":47732,"inlinee":9711,"source_line":2802,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2828,"record_offset":48904,"parent_record_offset":47732,"ancestor_record_offset":47732,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] image::buffer_::ImageBuffer::pixels
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":2828,"record_offset":47876,"parent_record_offset":47732,"ancestor_record_offset":47732,"inlinee":13332,"source_line":720,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":2828,"record_offset":48876,"parent_record_offset":47732,"ancestor_record_offset":47732,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":2828,"record_offset":48736,"parent_record_offset":47732,"ancestor_record_offset":47732,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2828,"record_offset":48632,"parent_record_offset":48608,"ancestor_record_offset":47732,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2828,"record_offset":48656,"parent_record_offset":48632,"ancestor_record_offset":47732,"inlinee":6014,"source_line":171,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2828,"record_offset":49032,"parent_record_offset":49008,"ancestor_record_offset":47732,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2828,"record_offset":48608,"parent_record_offset":48576,"ancestor_record_offset":47732,"inlinee":6013,"source_line":247,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2828,"record_offset":49008,"parent_record_offset":48952,"ancestor_record_offset":47732,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":48980,"parent_record_offset":48952,"ancestor_record_offset":47732,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":48576,"parent_record_offset":48552,"ancestor_record_offset":47732,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":48552,"parent_record_offset":48528,"ancestor_record_offset":47732,"inlinee":6012,"source_line":169,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":48696,"parent_record_offset":48552,"ancestor_record_offset":47732,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2828,"record_offset":48952,"parent_record_offset":48928,"ancestor_record_offset":47732,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$3::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":2828,"record_offset":48528,"parent_record_offset":48504,"ancestor_record_offset":47732,"inlinee":6032,"source_line":50,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] asset_pipe::texture::compress::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset-pipe/src/texture.rs
//     evidence={"module":2828,"record_offset":48464,"parent_record_offset":48440,"ancestor_record_offset":47732,"inlinee":90316,"source_line":692,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2828,"record_offset":48440,"parent_record_offset":48360,"ancestor_record_offset":47732,"inlinee":9715,"source_line":2809,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2828,"record_offset":48360,"parent_record_offset":48336,"ancestor_record_offset":47732,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2828,"record_offset":48008,"parent_record_offset":47988,"ancestor_record_offset":47732,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2828,"record_offset":48032,"parent_record_offset":48008,"ancestor_record_offset":47732,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2828,"record_offset":47964,"parent_record_offset":47940,"ancestor_record_offset":47732,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2828,"record_offset":48192,"parent_record_offset":47904,"ancestor_record_offset":47732,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2828,"record_offset":48928,"parent_record_offset":48904,"ancestor_record_offset":47732,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2828,"record_offset":48228,"parent_record_offset":47904,"ancestor_record_offset":47732,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2828,"record_offset":48284,"parent_record_offset":48256,"ancestor_record_offset":47732,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2828,"record_offset":48256,"parent_record_offset":48228,"ancestor_record_offset":47732,"inlinee":8599,"source_line":463,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$89::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":2828,"record_offset":48408,"parent_record_offset":48384,"ancestor_record_offset":47732,"inlinee":15082,"source_line":1881,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2828,"record_offset":48760,"parent_record_offset":48736,"ancestor_record_offset":47732,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":2828,"record_offset":48784,"parent_record_offset":48760,"ancestor_record_offset":47732,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::image_buffer_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":2828,"record_offset":47940,"parent_record_offset":47904,"ancestor_record_offset":47732,"inlinee":7583,"source_line":807,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::inner_pixels
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":2828,"record_offset":47904,"parent_record_offset":47876,"ancestor_record_offset":47732,"inlinee":9746,"source_line":713,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":2828,"record_offset":48384,"parent_record_offset":48360,"ancestor_record_offset":47732,"inlinee":15081,"source_line":34,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":2828,"record_offset":47988,"parent_record_offset":47964,"ancestor_record_offset":47732,"inlinee":7585,"source_line":809,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::impl$34::image_buffer_len::closure$1
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/buffer.rs
//     evidence={"module":2828,"record_offset":48100,"parent_record_offset":48076,"ancestor_record_offset":47732,"inlinee":7586,"source_line":810,"archive":"_pdb/A/modules/02828.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset_pipe::texture::compress::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 692
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x160CC
// 首条位置: {"module":2828,"inlinee":90316}
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
// 完整结构及原始类型记录: texture.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset_pipe::texture::TextureAssetLibraryLoadResult
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>", "constructor": "enum2$", "arguments": ["asset_pipe::texture::TextureAssetLibraryLoadResult"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures> >", "constructor": "std::sync::once_lock::impl$0::get_or_init::closure_env$0", "arguments": ["enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>", "country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once_lock::impl$0::initialize::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures> >,never$>", "constructor": "std::sync::once_lock::impl$0::initialize::closure_env$0", "arguments": ["enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>", "std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures> >", "never$"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once::impl$2::call_once_force::closure$0<std::sync::once_lock::impl$0::initialize::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures> >,never$> >", "constructor": "std::sync::once::impl$2::call_once_force::closure$0", "arguments": ["std::sync::once_lock::impl$0::initialize::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures> >,never$>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures> >", "constructor": "std::sync::once_lock::impl$0::get_or_init::closure_env$0", "arguments": ["enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>", "country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once_lock::OnceLock::initialize<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures> >,never$>", "constructor": "std::sync::once_lock::OnceLock::initialize", "arguments": ["enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>", "std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures> >", "never$"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once_lock::OnceLock::initialize<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures> >,never$>", "constructor": "std::sync::once_lock::OnceLock::initialize", "arguments": ["enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>", "std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures> >", "never$"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once_lock::impl$0::initialize::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures> >,never$>", "constructor": "std::sync::once_lock::impl$0::initialize::closure_env$0", "arguments": ["enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>", "std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures> >", "never$"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::once::impl$2::call_once_force::closure_env$0<std::sync::once_lock::impl$0::initialize::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures> >,never$> >", "constructor": "std::sync::once::impl$2::call_once_force::closure_env$0", "arguments": ["std::sync::once_lock::impl$0::initialize::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures> >,never$>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<std::sync::once::impl$2::call_once_force::closure_env$0<std::sync::once_lock::impl$0::initialize::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures> >,never$> >,tuple$<ref$<std::sync::once::OnceState> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["std::sync::once::impl$2::call_once_force::closure_env$0<std::sync::once_lock::impl$0::initialize::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures> >,never$> >", "tuple$<ref$<std::sync::once::OnceState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::once::impl$2::call_once_force::closure_env$0<std::sync::once_lock::impl$0::initialize::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures> >,never$> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::once::impl$2::call_once_force::closure_env$0<std::sync::once_lock::impl$0::initialize::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures> >,never$> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset_pipe::texture::TextureAssetLibraryMeta
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<asset_pipe::texture::TextureAssetLibraryMeta>", "constructor": "core::marker::PhantomData", "arguments": ["asset_pipe::texture::TextureAssetLibraryMeta"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::options::Options::from_str_seed<core::marker::PhantomData<asset_pipe::texture::TextureAssetLibraryMeta>,asset_pipe::texture::TextureAssetLibraryMeta>", "constructor": "ron::options::Options::from_str_seed", "arguments": ["core::marker::PhantomData<asset_pipe::texture::TextureAssetLibraryMeta>", "asset_pipe::texture::TextureAssetLibraryMeta"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::options::Options::to_string<asset_pipe::texture::TextureAssetLibraryMeta>", "constructor": "ron::options::Options::to_string", "arguments": ["asset_pipe::texture::TextureAssetLibraryMeta"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset_pipe::texture::TextureAssetMeta
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 60
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::Keys<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta>", "constructor": "std::collections::hash::map::Keys", "arguments": ["relative_path::RelativePathBuf", "asset_pipe::texture::TextureAssetMeta"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<ref$<relative_path::RelativePathBuf>,std::collections::hash::map::Keys<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["ref$<relative_path::RelativePathBuf>", "std::collections::hash::map::Keys<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error>", "constructor": "core::result::Result", "arguments": ["asset_pipe::texture::TextureAssetMeta", "anyhow::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >", "constructor": "tuple$", "arguments": ["relative_path::RelativePathBuf", "enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "easy_parallel::Parallel<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "easy_parallel::Parallel", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<easy_parallel::Parallel<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["easy_parallel::Parallel<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::Sender<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpsc::Sender", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpsc::Sender<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpsc::Sender<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::Receiver<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpsc::Receiver", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpsc::Receiver<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpsc::Receiver<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::SendError<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpsc::SendError", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpsc::SendError<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpsc::SendError<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Packet<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::zero::Packet", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::zero::Packet<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::zero::Packet<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<std::sync::mpsc::Receiver<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["std::sync::mpsc::Receiver<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<std::sync::mpsc::Receiver<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<std::sync::mpsc::Receiver<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<std::sync::mpsc::Receiver<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["std::sync::mpsc::Receiver<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<std::sync::mpsc::Receiver<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<std::sync::mpsc::Receiver<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::list::Channel", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >", "constructor": "std::sync::mpmc::counter::Counter", "arguments": ["std::sync::mpmc::list::Channel<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::array::Channel", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >", "constructor": "std::sync::mpmc::counter::Counter", "arguments": ["std::sync::mpmc::array::Channel<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mpmc::counter::Counter<std::sync::mpmc::array::Channel<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "assoc$", "arguments": ["Output", "tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce<tuple$<>,assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >", "constructor": "core::ops::function::FnOnce", "arguments": ["tuple$<>", "assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >,core::marker::Send>", "constructor": "dyn$", "arguments": ["core::ops::function::FnOnce<tuple$<>,assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >", "core::marker::Send"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >,core::marker::Send>,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >,core::marker::Send>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >,core::marker::Send>,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >,core::marker::Send>,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >,core::marker::Send>,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >,core::marker::Send>,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<std::sync::mpsc::Receiver<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["std::sync::mpsc::Receiver<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "easy_parallel::Parallel::run<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "easy_parallel::Parallel::run", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<asset_pipe::texture::TextureAssetMeta>", "constructor": "core::option::Option", "arguments": ["asset_pipe::texture::TextureAssetMeta"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >", "constructor": "enum2$", "arguments": ["core::option::Option<asset_pipe::texture::TextureAssetMeta>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>", "constructor": "tuple$", "arguments": ["enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >", "relative_path::RelativePathBuf", "asset::texture::desc::TextureAssetDesc"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "easy_parallel::Parallel::each<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>,asset_pipe::texture::compile_texture_assets::closure_env$4>", "constructor": "easy_parallel::Parallel::each", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >", "tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>", "core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>", "asset_pipe::texture::compile_texture_assets::closure_env$4"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "easy_parallel::impl$0::finish::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >,core::marker::Send>,alloc::alloc::Global>,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "easy_parallel::impl$0::finish::closure_env$0", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >", "alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >,core::marker::Send>,alloc::alloc::Global>", "tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<easy_parallel::impl$0::finish::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >,core::marker::Send>,alloc::alloc::Global>,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >,tuple$<> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["easy_parallel::impl$0::finish::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >,core::marker::Send>,alloc::alloc::Global>,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "easy_parallel::impl$0::each::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>,asset_pipe::texture::compile_texture_assets::closure_env$4>", "constructor": "easy_parallel::impl$0::each::closure_env$0", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >", "tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>", "core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>", "asset_pipe::texture::compile_texture_assets::closure_env$4"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<easy_parallel::impl$0::each::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>,asset_pipe::texture::compile_texture_assets::closure_env$4>,tuple$<> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["easy_parallel::impl$0::each::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>,asset_pipe::texture::compile_texture_assets::closure_env$4>", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >,core::marker::Send>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >,core::marker::Send>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<easy_parallel::impl$0::finish::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >,core::marker::Send>,alloc::alloc::Global>,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["easy_parallel::impl$0::finish::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >,core::marker::Send>,alloc::alloc::Global>,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<easy_parallel::impl$0::each::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>,asset_pipe::texture::compile_texture_assets::closure_env$4> >", "constructor": "core::ptr::drop_in_place", "arguments": ["easy_parallel::impl$0::each::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>,asset_pipe::texture::compile_texture_assets::closure_env$4>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["relative_path::RelativePathBuf", "asset_pipe::texture::TextureAssetMeta", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta>", "constructor": "tuple$", "arguments": ["relative_path::RelativePathBuf", "asset_pipe::texture::TextureAssetMeta"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta> >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::equivalent_key::closure_env$0<relative_path::RelativePathBuf,relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta>", "constructor": "hashbrown::map::equivalent_key::closure_env$0", "arguments": ["relative_path::RelativePathBuf", "relative_path::RelativePathBuf", "asset_pipe::texture::TextureAssetMeta"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::remove_entry<tuple$<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$0<relative_path::RelativePathBuf,relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta> >", "constructor": "hashbrown::raw::RawTable::remove_entry", "arguments": ["tuple$<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta>", "alloc::alloc::Global", "hashbrown::map::equivalent_key::closure_env$0<relative_path::RelativePathBuf,relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["relative_path::RelativePathBuf", "asset_pipe::texture::TextureAssetMeta", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::disconnect_senders<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::list::Channel::disconnect_senders", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::disconnect_receivers<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::list::Channel::disconnect_receivers", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::recv<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::list::Channel::recv", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::impl$3::recv::closure$1<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::list::impl$3::recv::closure$1", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::list::Channel::send<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::list::Channel::send", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::zero::Channel", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::impl$6::drop::closure_env$2<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::impl$6::drop::closure_env$2", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Sender::release<std::sync::mpmc::zero::Channel<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >,std::sync::mpmc::impl$6::drop::closure_env$2<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >", "constructor": "std::sync::mpmc::counter::Sender::release", "arguments": ["std::sync::mpmc::zero::Channel<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "std::sync::mpmc::impl$6::drop::closure_env$2<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::impl$15::drop::closure_env$2<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::impl$15::drop::closure_env$2", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::counter::Receiver::release<std::sync::mpmc::zero::Channel<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >,std::sync::mpmc::impl$15::drop::closure_env$2<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >", "constructor": "std::sync::mpmc::counter::Receiver::release", "arguments": ["std::sync::mpmc::zero::Channel<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "std::sync::mpmc::impl$15::drop::closure_env$2<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::disconnect_receivers<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::array::Channel::disconnect_receivers", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::recv<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::array::Channel::recv", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::impl$1::recv::closure$1<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::array::impl$1::recv::closure$1", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::Channel::send<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::array::Channel::send", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::array::impl$1::send::closure$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::array::impl$1::send::closure$0", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::panic::unwind_safe::AssertUnwindSafe<alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >,core::marker::Send>,alloc::alloc::Global> >", "constructor": "core::panic::unwind_safe::AssertUnwindSafe", "arguments": ["alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >,core::marker::Send>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::panicking::try<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,core::panic::unwind_safe::AssertUnwindSafe<alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >,core::marker::Send>,alloc::alloc::Global> > >", "constructor": "std::panicking::try", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >", "core::panic::unwind_safe::AssertUnwindSafe<alloc::boxed::Box<dyn$<core::ops::function::FnOnce<tuple$<>,assoc$<Output,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >,core::marker::Send>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::HashMap<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta,std::hash::random::RandomState>", "constructor": "std::collections::hash::map::HashMap", "arguments": ["relative_path::RelativePathBuf", "asset_pipe::texture::TextureAssetMeta", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,std::collections::hash::map::HashMap<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta,std::hash::random::RandomState> >", "constructor": "ron::ser::impl$11::serialize_field", "arguments": ["ref_mut$<alloc::string::String>", "std::collections::hash::map::HashMap<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$204::deserialize::MapVisitor<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta,std::hash::random::RandomState>", "constructor": "serde::de::impls::impl$204::deserialize::MapVisitor", "arguments": ["relative_path::RelativePathBuf", "asset_pipe::texture::TextureAssetMeta", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$2::deserialize_map::impl$0::fmt<serde::de::impls::impl$204::deserialize::MapVisitor<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta,std::hash::random::RandomState> >", "constructor": "ron::de::impl$2::deserialize_map::impl$0::fmt", "arguments": ["serde::de::impls::impl$204::deserialize::MapVisitor<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ron::de::impl$2::deserialize_map<serde::de::impls::impl$204::deserialize::MapVisitor<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta,std::hash::random::RandomState> >", "constructor": "ron::de::impl$2::deserialize_map", "arguments": ["serde::de::impls::impl$204::deserialize::MapVisitor<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::disconnect<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::zero::Channel::disconnect", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::recv<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::zero::Channel::recv", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::recv::closure$1<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::zero::impl$3::recv::closure$1", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::Channel::send<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::zero::Channel::send", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::send::closure$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::zero::impl$3::send::closure$0", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tuple$<>,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "core::result::Result", "arguments": ["tuple$<>", "tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tuple$<>,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >", "constructor": "enum2$", "arguments": ["core::result::Result<tuple$<>,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tuple$<>,tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::error::SendTimeoutError<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::error::SendTimeoutError", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<std::sync::mpmc::error::SendTimeoutError<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >", "constructor": "enum2$", "arguments": ["std::sync::mpmc::error::SendTimeoutError<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > > >", "constructor": "core::result::Result", "arguments": ["tuple$<>", "enum2$<std::sync::mpmc::error::SendTimeoutError<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > > > >", "constructor": "enum2$", "arguments": ["core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<tuple$<>,enum2$<std::sync::mpmc::error::SendTimeoutError<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::recv::closure_env$1<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::zero::impl$3::recv::closure_env$1", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<std::sync::mpmc::zero::impl$3::recv::closure_env$1<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >", "constructor": "core::option::Option", "arguments": ["std::sync::mpmc::zero::impl$3::recv::closure_env$1<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::recv::closure_env$1<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > > >", "constructor": "enum2$", "arguments": ["core::option::Option<std::sync::mpmc::zero::impl$3::recv::closure_env$1<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::recv::closure_env$1<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::recv::closure_env$1<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::zero::impl$3::send::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::zero::impl$3::send::closure_env$0", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >", "constructor": "core::option::Option", "arguments": ["std::sync::mpmc::zero::impl$3::send::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > > >", "constructor": "enum2$", "arguments": ["core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<std::sync::mpmc::zero::impl$3::send::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpmc::Sender::send<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpmc::Sender::send", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<std::sync::mpmc::error::SendTimeoutError<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<std::sync::mpmc::error::SendTimeoutError<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mpsc::impl$18::fmt<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > > >", "constructor": "std::sync::mpsc::impl$18::fmt", "arguments": ["tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: texture.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: texture.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: asset_pipe::texture::_::impl$0::deserialize
// 信息 ID: 2ca1e4f36e41f469ed0df86f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::options::Options::from_str_seed<core::marker::PhantomData<asset_pipe::texture::TextureAssetLibraryMeta>,asset_pipe::texture::TextureAssetLibraryMeta>; module=3918, inline_record=584; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_map<serde::de::impls::impl$204::deserialize::MapVisitor<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta,std::hash::random::RandomState> >; module=4396, inline_record=16376; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 02ae745ef46f03128fd59960
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset_pipe::texture::_::impl$0::deserialize::__Visitor>; module=4396, inline_record=28228; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset_pipe::texture::_::impl$0::deserialize::__Visitor>; module=4396, inline_record=37604; 不是独立调用边。
// [PDB 内联宿主] ron::de::id::impl$1::deserialize_identifier<asset_pipe::texture::_::impl$0::deserialize::__FieldVisitor>; module=4397, inline_record=4864; 不是独立调用边。
// [PDB 内联宿主] ron::de::id::impl$1::deserialize_identifier<asset_pipe::texture::_::impl$0::deserialize::__FieldVisitor>; module=4397, inline_record=5284; 不是独立调用边。
// [PDB 内联宿主] ron::de::id::impl$1::deserialize_identifier<asset_pipe::texture::_::impl$0::deserialize::__FieldVisitor>; module=4397, inline_record=6096; 不是独立调用边。
// [PDB 内联宿主] ron::de::id::impl$1::deserialize_identifier<asset_pipe::texture::_::impl$0::deserialize::__FieldVisitor>; module=4397, inline_record=6372; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 1a6ca4d11a7f499df99c7632
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset_pipe::texture::_::impl$0::deserialize::__Visitor>; module=4396, inline_record=27724; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset_pipe::texture::_::impl$0::deserialize::__Visitor>; module=4396, inline_record=27932; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset_pipe::texture::_::impl$0::deserialize::__Visitor>; module=4396, inline_record=28088; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset_pipe::texture::_::impl$0::deserialize::__Visitor>; module=4396, inline_record=37120; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset_pipe::texture::_::impl$0::deserialize::__Visitor>; module=4396, inline_record=37328; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset_pipe::texture::_::impl$0::deserialize::__Visitor>; module=4396, inline_record=37480; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 46a1dddc2ea0017dd6f770d2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset_pipe::texture::_::impl$0::deserialize::__Visitor>; module=4396, inline_record=27324; 不是独立调用边。
// [PDB 内联宿主] ron::de::impl$2::deserialize_struct<asset_pipe::texture::_::impl$0::deserialize::__Visitor>; module=4396, inline_record=36736; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::_::impl$0::serialize<ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > > >
// 信息 ID: a5f79ecc2e85196988d8bf61
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref_mut$ -> ["alloc::string::String"]; 字段布局 = unknown。
// [类型关系] ron::ser::Serializer -> ["ref_mut$<alloc::string::String>"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["ron::ser::Serializer<ref_mut$<alloc::string::String> >"]; 字段布局 = unknown。
// [类型关系] asset_pipe::texture::_::impl$0::serialize -> ["ref_mut$<ron::ser::Serializer<ref_mut$<alloc::string::String> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] ron::options::Options::to_string<asset_pipe::texture::TextureAssetLibraryMeta> @ 0x1c8f4f5 -> 0x1c93e40 (direct_call); 同地址别名未裁定。
// [静态直接引用] ron::ser::impl$11::serialize_field<ref_mut$<alloc::string::String>,std::collections::hash::map::HashMap<relative_path::RelativePathBuf,asset_pipe::texture::TextureAssetMeta,std::hash::random::RandomState> > @ 0x1e732ea -> 0x1c94050 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::apply_transfer_function
// 信息 ID: 74cebe4a776d52dd6a12528e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::process_regular_image; module=2828, inline_record=44096; 不是独立调用边。
// [PDB 内联宿主] asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$4,tuple$<> >; module=3930, inline_record=1348; 不是独立调用边。
// [PDB 内联宿主] asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$2,alloc::vec::Vec<u8,alloc::alloc::Global> >; module=3930, inline_record=5448; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::compile_or_load_texture_assets
// 信息 ID: 123b5a203bb1f5053ebe31c0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sync::once::impl$2::call_once_force::closure$0<std::sync::once_lock::impl$0::initialize::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures> >,never$> > | core::ops::function::FnOnce::call_once<std::sync::once::impl$2::call_once_force::closure_env$0<std::sync::once_lock::impl$0::initialize::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,std::sync::once_lock::impl$0::get_or_init::closure_env$0<enum2$<asset_pipe::texture::TextureAssetLibraryLoadResult>,country_core::startup::load_textures::load_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures> >,never$> >,tuple$<ref$<std::sync::once::OnceState> > > @ 0xda84d1 -> 0x180f870 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::compile_or_load_texture_assets::closure$0
// 信息 ID: 527d0f6e8f8c2a191a56ccbc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset_pipe::texture::compile_or_load_texture_assets", "captures": "unknown"}
// [闭包环境类型引用] asset_pipe::texture::compile_or_load_texture_assets::closure_env$0 <- core::ptr::drop_in_place<core::iter::adapters::filter_map::FilterMap<walkdir::IntoIter,asset_pipe::texture::compile_or_load_texture_assets::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] asset_pipe::texture::compile_or_load_texture_assets; module=2828, inline_record=10968; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::compile_or_load_texture_assets::closure$1
// 信息 ID: 6f4d9b07c4f0e451684ae310
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "asset_pipe::texture::compile_or_load_texture_assets", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::compile_or_load_texture_assets; module=2828, inline_record=12000; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::compile_texture_asset
// 信息 ID: 4025d8eb8eff8b3c2854136c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<easy_parallel::impl$0::each::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>,asset_pipe::texture::compile_texture_assets::closure_env$4>,tuple$<> > @ 0x1c8e7f7 -> 0x18113f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::compile_texture_asset::closure$0
// 信息 ID: 0cfc49967c27f09f7b31709f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset_pipe::texture::compile_texture_asset", "captures": "unknown"}
// [闭包环境类型引用] asset_pipe::texture::compile_texture_asset::closure_env$0 <- anyhow::context::impl$0::with_context<std::fs::File,std::io::error::Error,alloc::string::String,asset_pipe::texture::compile_texture_asset::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<std::fs::File,std::io::error::Error,alloc::string::String,asset_pipe::texture::compile_texture_asset::closure_env$0>; module=3915, inline_record=3208; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::compile_texture_assets
// 信息 ID: 460ab6d16bc5e9b8773ce809
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset_pipe::texture::compile_or_load_texture_assets @ 0x180fe7b -> 0x1810120 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::compile_texture_assets::closure$1
// 信息 ID: 7775a7def0b5cc9a058beceb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "asset_pipe::texture::compile_texture_assets", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::compile_texture_assets; module=2828, inline_record=21140; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::compile_texture_assets::closure$2
// 信息 ID: 41af4d8d3a6ea3d11277a96e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "asset_pipe::texture::compile_texture_assets", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::compile_texture_assets; module=2828, inline_record=21768; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::compile_texture_assets::closure$3
// 信息 ID: f0451c4a01a00ee9ab8df2fa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "asset_pipe::texture::compile_texture_assets", "captures": "unknown"}
// [闭包环境类型引用] asset_pipe::texture::compile_texture_assets::closure_env$3 <- core::ptr::drop_in_place<core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] asset_pipe::texture::compile_texture_assets::closure_env$3 <- easy_parallel::Parallel::each<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>,asset_pipe::texture::compile_texture_assets::closure_env$4>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] asset_pipe::texture::compile_texture_assets::closure_env$3 <- core::ops::function::FnOnce::call_once<easy_parallel::impl$0::each::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>,asset_pipe::texture::compile_texture_assets::closure_env$4>,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] asset_pipe::texture::compile_texture_assets::closure_env$3 <- core::ptr::drop_in_place<easy_parallel::impl$0::each::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>,asset_pipe::texture::compile_texture_assets::closure_env$4> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] asset_pipe::texture::compile_texture_assets::closure_env$3 <- core::ops::function::impls::impl$4::call_once<tuple$<tuple$<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc> >,asset_pipe::texture::compile_texture_assets::closure_env$3>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 5 条。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<tuple$<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc> >,asset_pipe::texture::compile_texture_assets::closure_env$3>; module=3930, inline_record=524; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::compile_texture_assets::closure$4
// 信息 ID: 906438d48c94447a4f4883ad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "asset_pipe::texture::compile_texture_assets", "captures": "unknown"}
// [闭包环境类型引用] asset_pipe::texture::compile_texture_assets::closure_env$4 <- easy_parallel::Parallel::each<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>,asset_pipe::texture::compile_texture_assets::closure_env$4>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] asset_pipe::texture::compile_texture_assets::closure_env$4 <- core::ops::function::FnOnce::call_once<easy_parallel::impl$0::each::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>,asset_pipe::texture::compile_texture_assets::closure_env$4>,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] asset_pipe::texture::compile_texture_assets::closure_env$4 <- core::ptr::drop_in_place<easy_parallel::impl$0::each::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>,asset_pipe::texture::compile_texture_assets::closure_env$4> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<easy_parallel::impl$0::each::closure_env$0<tuple$<relative_path::RelativePathBuf,enum2$<core::result::Result<asset_pipe::texture::TextureAssetMeta,anyhow::Error> > >,tuple$<enum2$<core::option::Option<asset_pipe::texture::TextureAssetMeta> >,relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,core::iter::adapters::map::Map<std::collections::hash::map::IntoIter<relative_path::RelativePathBuf,asset::texture::desc::TextureAssetDesc>,asset_pipe::texture::compile_texture_assets::closure_env$3>,asset_pipe::texture::compile_texture_assets::closure_env$4>,tuple$<> >; module=3916, inline_record=8788; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::compress
// 信息 ID: 23e6fd1b57b8486aad1ec80c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$4,tuple$<> > @ 0x1c93805 -> 0x18151d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$4,tuple$<> > @ 0x1c938b5 -> 0x18151d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$2,alloc::vec::Vec<u8,alloc::alloc::Global> > @ 0x1c93c32 -> 0x18151d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$2,alloc::vec::Vec<u8,alloc::alloc::Global> > @ 0x1c93c74 -> 0x18151d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::compress::closure$0
// 信息 ID: c11fa103a8e5964f0df7ffde
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset_pipe::texture::compress", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::compress; module=2828, inline_record=48464; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$1::default
// 信息 ID: 36304b8296365224411aac45
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::compile_texture_assets; module=2828, inline_record=14452; 不是独立调用边。
// [PDB 内联宿主] asset_pipe::texture::compile_texture_assets; module=2828, inline_record=16268; 不是独立调用边。
// [PDB 内联宿主] asset_pipe::texture::compile_texture_assets; module=2828, inline_record=21976; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::impl$2::fmt
// 信息 ID: 37967a5632ae50a84ecee007
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::pad_image
// 信息 ID: 27dad3c89cc3585f25e13c31
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::process_regular_image::closure$3; module=2828, inline_record=46676; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::process_dds_image
// 信息 ID: 591b8b26b0e2adabce903912
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::compile_texture_asset; module=2828, inline_record=33784; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::process_dds_image::closure$0
// 信息 ID: 539644e7f1dde3f62ab2708d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset_pipe::texture::process_dds_image", "captures": "unknown"}
// [闭包环境类型引用] asset_pipe::texture::process_dds_image::closure_env$0 <- anyhow::context::impl$0::with_context<alloc::vec::Vec<u8,alloc::alloc::Global>,std::io::error::Error,alloc::string::String,asset_pipe::texture::process_dds_image::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<alloc::vec::Vec<u8,alloc::alloc::Global>,std::io::error::Error,alloc::string::String,asset_pipe::texture::process_dds_image::closure_env$0>; module=3915, inline_record=3856; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::process_dds_image::closure$1
// 信息 ID: 0e63da137b3f61c123e11683
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "asset_pipe::texture::process_dds_image", "captures": "unknown"}
// [闭包环境类型引用] asset_pipe::texture::process_dds_image::closure_env$1 <- anyhow::context::impl$0::with_context<ddsfile::Dds,enum2$<ddsfile::error::Error>,alloc::string::String,asset_pipe::texture::process_dds_image::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<ddsfile::Dds,enum2$<ddsfile::error::Error>,alloc::string::String,asset_pipe::texture::process_dds_image::closure_env$1>; module=3915, inline_record=2584; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::process_dds_image::closure$2
// 信息 ID: c7f89a6748c076f9c3ddfa1c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "asset_pipe::texture::process_dds_image", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::compile_texture_asset; module=2828, inline_record=34928; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::process_dds_image::closure$3
// 信息 ID: ac70780794a1cac689de6e20
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "asset_pipe::texture::process_dds_image", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::compile_texture_asset; module=2828, inline_record=35040; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::process_image_array
// 信息 ID: 90eb9c64b6073759a193abb1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::compile_texture_asset; module=2828, inline_record=26232; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::process_regular_image
// 信息 ID: 2303a06ea265f701ab5fb3d3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset_pipe::texture::compile_texture_asset @ 0x181179c -> 0x1814120 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset_pipe::texture::compile_texture_asset @ 0x1811d5e -> 0x1814120 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::process_regular_image::closure$0
// 信息 ID: e2016000f991ae6a715d950f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "asset_pipe::texture::process_regular_image", "captures": "unknown"}
// [闭包环境类型引用] asset_pipe::texture::process_regular_image::closure_env$0 <- anyhow::context::impl$0::with_context<enum2$<image::dynimage::DynamicImage>,enum2$<image::error::ImageError>,alloc::string::String,asset_pipe::texture::process_regular_image::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<enum2$<image::dynimage::DynamicImage>,enum2$<image::error::ImageError>,alloc::string::String,asset_pipe::texture::process_regular_image::closure_env$0>; module=3915, inline_record=1956; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::process_regular_image::closure$2
// 信息 ID: 47f0a6f7ff8cb92b1cf4d387
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "asset_pipe::texture::process_regular_image", "captures": "unknown"}
// [闭包环境类型引用] asset_pipe::texture::process_regular_image::closure_env$2 <- asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$2,alloc::vec::Vec<u8,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$2,alloc::vec::Vec<u8,alloc::alloc::Global> >; module=3930, inline_record=6052; 不是独立调用边。
// [PDB 内联宿主] asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$2,alloc::vec::Vec<u8,alloc::alloc::Global> >; module=3930, inline_record=6872; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::process_regular_image::closure$3
// 信息 ID: 9a8e09a233b8e760eab81708
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "asset_pipe::texture::process_regular_image", "captures": "unknown"}
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset_pipe::texture::process_regular_image @ 0x1814674 -> 0x1814d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset_pipe::texture::process_regular_image @ 0x1814706 -> 0x1814d90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::process_regular_image::closure$4
// 信息 ID: e4ea2bd183aeda935014e281
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "asset_pipe::texture::process_regular_image", "captures": "unknown"}
// [闭包环境类型引用] asset_pipe::texture::process_regular_image::closure_env$4 <- asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$4,tuple$<> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$4,tuple$<> >; module=3930, inline_record=1952; 不是独立调用边。
// [PDB 内联宿主] asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$4,tuple$<> >; module=3930, inline_record=3484; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::resize_image
// 信息 ID: 4a7a60447a521a8a191e57e4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::process_regular_image; module=2828, inline_record=45128; 不是独立调用边。
// [PDB 内联宿主] asset_pipe::texture::process_regular_image::closure$3; module=2828, inline_record=46576; 不是独立调用边。
// [PDB 内联宿主] asset_pipe::texture::process_regular_image::closure$3; module=2828, inline_record=46768; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::srgb_eotf
// 信息 ID: 3dbafcff8d3afd13b15d66bb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset_pipe::texture::process_regular_image; module=2828, inline_record=44692; 不是独立调用边。
// [PDB 内联宿主] asset_pipe::texture::process_regular_image; module=2828, inline_record=44760; 不是独立调用边。
// [PDB 内联宿主] asset_pipe::texture::process_regular_image; module=2828, inline_record=44824; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::srgb_oetf
// 信息 ID: 99648bf38e789802fbc031fe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$4,tuple$<> > @ 0x1c93778 -> 0x1815190 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$4,tuple$<> > @ 0x1c9378a -> 0x1815190 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$4,tuple$<> > @ 0x1c9379d -> 0x1815190 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$2,alloc::vec::Vec<u8,alloc::alloc::Global> > @ 0x1c93ba9 -> 0x1815190 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$2,alloc::vec::Vec<u8,alloc::alloc::Global> > @ 0x1c93bbd -> 0x1815190 (direct_call); 同地址别名未裁定。
// [静态直接引用] asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$2,alloc::vec::Vec<u8,alloc::alloc::Global> > @ 0x1c93bd2 -> 0x1815190 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$2,alloc::vec::Vec<u8,alloc::alloc::Global> >
// 信息 ID: 81a64bc9ce8ed022e3c7c5d5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] asset_pipe::texture::with_rgba8 -> ["asset_pipe::texture::process_regular_image::closure_env$2", "alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset_pipe::texture::process_regular_image @ 0x1814647 -> 0x1c93b30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset_pipe::texture::with_rgba8<asset_pipe::texture::process_regular_image::closure_env$4,tuple$<> >
// 信息 ID: d1376d3954d14b4b87f16f2f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> []; 字段布局 = unknown。
// [类型关系] asset_pipe::texture::with_rgba8 -> ["asset_pipe::texture::process_regular_image::closure_env$4", "tuple$<>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset_pipe::texture::process_regular_image @ 0x18146ed -> 0x1c93700 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
