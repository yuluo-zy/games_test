// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/render/texture_export.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: e83b598a9a179e08325230aa7f51a6131dce03f6df2916a485ba0fcbe3c6a9e0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: texture_export.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 545, 1253, 1656
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use arrayvec; // namespace candidate
//   package identities: [{"name":"arrayvec","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.7.6"}]
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use image; // namespace candidate
//   package identities: [{"name":"image","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.24.9"},{"name":"image","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.25.1"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use turbosloth; // namespace candidate
//   package identities: [{"name":"turbosloth","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::render::texture_export::export_packaged_textures
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":545,"record_offset":844,"rva":11087648,"va":5379796768}
// 编译实例 RVAs: 0xa92f20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::backtrace::Backtrace>::capture
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4862113","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e8a1304a00","size":5,"soft_fail":false,"target_va":5384659264,"va":5379797146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::DirBuilder>::_create
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4890144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e8209e4a00","size":5,"soft_fail":false,"target_va":5384689616,"va":5379799467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t4857191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e8671d4a00","size":5,"soft_fail":false,"target_va":5384656240,"va":5379799044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::parent
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4857075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e8f31c4a00","size":5,"soft_fail":false,"target_va":5384656512,"va":5379799432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$rhapsody..raw..texture..Texture$u20$as$u20$rhapsody..raw..respool..HandleRef$LT$rhapsody..raw..texture..TextureSchema$GT$$GT$::cold
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3666554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e87af23700","size":5,"soft_fail":false,"target_va":5383464496,"va":5379797937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=14
//     evidence={"assembly":"\tcallq\t-4118673","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e86f27c1ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379797372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: texture_export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10462001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e8cf5c60ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5379800844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a2fd0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: texture_export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10422891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e895f560ff","size":5,"soft_fail":false,"target_va":5369376720,"va":5379799606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140523d10
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: texture_export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5701109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e80b02a9ff","size":5,"soft_fail":false,"target_va":5374098704,"va":5379799808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140698b50
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: texture_export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4170950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e83a5bc0ff","size":5,"soft_fail":false,"target_va":5375626064,"va":5379797009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096d120
//     owner=unresolved_namespace; count=2
//     target_aliases=6 candidates; complete list: texture_export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1207437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e87393edff","size":5,"soft_fail":false,"target_va":5378593056,"va":5379800488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140970e60
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: texture_export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1191583","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e861d1edff","size":5,"soft_fail":false,"target_va":5378608736,"va":5379800314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140975970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture_export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1170767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e8b122eeff","size":5,"soft_fail":false,"target_va":5378627952,"va":5379798714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140975bd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: texture_export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1172086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e88a1deeff","size":5,"soft_fail":false,"target_va":5378628560,"va":5379800641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098ee20
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: texture_export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1068072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e8d8b3efff","size":5,"soft_fail":false,"target_va":5378731552,"va":5379799619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098f9c0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: texture_export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1065116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e864bfefff","size":5,"soft_fail":false,"target_va":5378734528,"va":5379799639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140990f00
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: texture_export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1060538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e846d1efff","size":5,"soft_fail":false,"target_va":5378739968,"va":5379800501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140994b90
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: texture_export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1044834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e89e0ef0ff","size":5,"soft_fail":false,"target_va":5378755472,"va":5379800301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409c8480
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture_export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-831634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e86e4ff3ff","size":5,"soft_fail":false,"target_va":5378966656,"va":5379798285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409c9fa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture_export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-825088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e80069f3ff","size":5,"soft_fail":false,"target_va":5378973600,"va":5379798683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409cb640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture_export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-819161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e82780f3ff","size":5,"soft_fail":false,"target_va":5378979392,"va":5379798548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ca1bc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture_export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2155852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e84ce52000","size":5,"soft_fail":false,"target_va":5381954496,"va":5379798639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411af900
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture_export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7454459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e8fbbe7100","size":5,"soft_fail":false,"target_va":5387254016,"va":5379799552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411afa90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture_export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7457543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e807cb7100","size":5,"soft_fail":false,"target_va":5387254416,"va":5379796868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b0dd90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture_export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17280543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e81fae0701","size":5,"soft_fail":false,"target_va":5397077392,"va":5379796844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b0df60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture_export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17280752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e8f0ae0701","size":5,"soft_fail":false,"target_va":5397077856,"va":5379797099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd3c40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture_export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36965140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e8140b3402","size":5,"soft_fail":false,"target_va":5416762432,"va":5379797287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd4bb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture_export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36969132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e8ac1a3402","size":5,"soft_fail":false,"target_va":5416766384,"va":5379797247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd5590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture_export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36968319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e87f173402","size":5,"soft_fail":false,"target_va":5416768912,"va":5379800588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd56b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture_export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36968985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e8191a3402","size":5,"soft_fail":false,"target_va":5416769200,"va":5379800210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd57e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture_export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36968882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e8b2193402","size":5,"soft_fail":false,"target_va":5416769504,"va":5379800617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: texture_export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37259974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e8c68a3802","size":5,"soft_fail":false,"target_va":5417057040,"va":5379797061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: texture_export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37258744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e8f8853802","size":5,"soft_fail":false,"target_va":5417057056,"va":5379798307,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4995739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e89b3a4c00","size":5,"soft_fail":false,"target_va":5384794864,"va":5379799120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36820860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e87cd73102","size":5,"soft_fail":false,"target_va":5416621536,"va":5379800671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36821503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e8ffd93102","size":5,"soft_fail":false,"target_va":5416622240,"va":5379800732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36820843","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e86bd73102","size":5,"soft_fail":false,"target_va":5416621632,"va":5379800784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36821617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e871da3102","size":5,"soft_fail":false,"target_va":5416622320,"va":5379800698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::load_texture::load_packaged_texture_image
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1038597","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e805d90f00","size":5,"soft_fail":false,"target_va":5380836512,"va":5379797910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_mod_textures::is_moddable_texture_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-710702","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e8d227f5ff","size":5,"soft_fail":false,"target_va":5379087120,"va":5379797817,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::io::free_functions::save_buffer_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"image","version":"0.24.9","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3163455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e83f453000","size":5,"soft_fail":false,"target_va":5382962992,"va":5379799532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::BufferDesc::new_gpu_to_cpu
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3689716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e8f44c3800","size":5,"soft_fail":false,"target_va":5383487824,"va":5379798103,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_mapped_buffer
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3923906","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e8c2df3b00","size":5,"soft_fail":false,"target_va":5383722048,"va":5379798137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::format::vk_format_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3689531","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e83b4c3800","size":5,"soft_fail":false,"target_va":5383488288,"va":5379798752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::gpu_scope::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_gpu_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4001725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e8bd0f3d00","size":5,"soft_fail":false,"target_va":5383799984,"va":5379798254,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::rcache::RenderResourceCache::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t4055689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379796768,"raw_hex":"e889e23d00","size":5,"soft_fail":false,"target_va":5383853424,"va":5379797730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*400(%r15)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5379796768,"raw_hex":"41ff9790010000","size":7,"soft_fail":false,"va":5379797545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":545,"record_offset":4828,"parent_record_offset":844,"ancestor_record_offset":844,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":545,"record_offset":3284,"parent_record_offset":844,"ancestor_record_offset":844,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":545,"record_offset":3312,"parent_record_offset":844,"ancestor_record_offset":844,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":545,"record_offset":1632,"parent_record_offset":844,"ancestor_record_offset":844,"inlinee":7507,"source_line":46,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$1::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":545,"record_offset":1756,"parent_record_offset":844,"ancestor_record_offset":844,"inlinee":10143,"source_line":91,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":545,"record_offset":7256,"parent_record_offset":844,"ancestor_record_offset":844,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":545,"record_offset":4912,"parent_record_offset":844,"ancestor_record_offset":844,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":545,"record_offset":4660,"parent_record_offset":844,"ancestor_record_offset":844,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":545,"record_offset":6628,"parent_record_offset":844,"ancestor_record_offset":844,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":545,"record_offset":1852,"parent_record_offset":844,"ancestor_record_offset":844,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":545,"record_offset":1092,"parent_record_offset":844,"ancestor_record_offset":844,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":545,"record_offset":1124,"parent_record_offset":844,"ancestor_record_offset":844,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::chunks_exact
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":545,"record_offset":4628,"parent_record_offset":844,"ancestor_record_offset":844,"inlinee":27399,"source_line":1242,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":545,"record_offset":4396,"parent_record_offset":844,"ancestor_record_offset":844,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":545,"record_offset":3424,"parent_record_offset":844,"ancestor_record_offset":844,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$98::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":545,"record_offset":4496,"parent_record_offset":844,"ancestor_record_offset":844,"inlinee":9636,"source_line":2051,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::texture_export::png_export_encoding
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/texture_export.rs
//     evidence={"module":545,"record_offset":3560,"parent_record_offset":844,"ancestor_record_offset":844,"inlinee":33871,"source_line":190,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] image::dynimage::save_buffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/dynimage.rs
//     evidence={"module":545,"record_offset":6748,"parent_record_offset":844,"ancestor_record_offset":844,"inlinee":9681,"source_line":1111,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":545,"record_offset":7172,"parent_record_offset":844,"ancestor_record_offset":844,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::TextureDesc::dims_2d
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/texture.rs
//     evidence={"module":545,"record_offset":3532,"parent_record_offset":844,"ancestor_record_offset":844,"inlinee":21753,"source_line":623,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::TextureHandleExt::desc
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/texture.rs
//     evidence={"module":545,"record_offset":3504,"parent_record_offset":844,"ancestor_record_offset":844,"inlinee":13930,"source_line":89,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::impl$58::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/texture.rs
//     evidence={"module":545,"record_offset":4744,"parent_record_offset":844,"ancestor_record_offset":844,"inlinee":13934,"source_line":301,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::executor::GraphExecutor::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/executor.rs
//     evidence={"module":545,"record_offset":2880,"parent_record_offset":844,"ancestor_record_offset":844,"inlinee":13682,"source_line":278,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":545,"record_offset":6664,"parent_record_offset":844,"ancestor_record_offset":844,"inlinee":7539,"source_line":3123,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":545,"record_offset":4772,"parent_record_offset":844,"ancestor_record_offset":844,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] turbosloth::store::PackageManifest::asset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/store.rs
//     evidence={"module":545,"record_offset":1152,"parent_record_offset":844,"ancestor_record_offset":844,"inlinee":30962,"source_line":165,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":2204,"parent_record_offset":2180,"ancestor_record_offset":844,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":2228,"parent_record_offset":2204,"ancestor_record_offset":844,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":2252,"parent_record_offset":2228,"ancestor_record_offset":844,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":2180,"parent_record_offset":2116,"ancestor_record_offset":844,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":545,"record_offset":4876,"parent_record_offset":4852,"ancestor_record_offset":844,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":545,"record_offset":2148,"parent_record_offset":2116,"ancestor_record_offset":844,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":545,"record_offset":2116,"parent_record_offset":2088,"ancestor_record_offset":844,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":545,"record_offset":2088,"parent_record_offset":2060,"ancestor_record_offset":844,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$93::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":545,"record_offset":1436,"parent_record_offset":1268,"ancestor_record_offset":844,"inlinee":21307,"source_line":2666,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":545,"record_offset":2992,"parent_record_offset":2952,"ancestor_record_offset":844,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":545,"record_offset":3164,"parent_record_offset":2880,"ancestor_record_offset":844,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":545,"record_offset":2952,"parent_record_offset":2880,"ancestor_record_offset":844,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":545,"record_offset":1900,"parent_record_offset":1876,"ancestor_record_offset":844,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":545,"record_offset":1716,"parent_record_offset":1676,"ancestor_record_offset":844,"inlinee":6277,"source_line":245,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_display
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":545,"record_offset":1792,"parent_record_offset":1756,"ancestor_record_offset":844,"inlinee":6351,"source_line":212,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":545,"record_offset":1676,"parent_record_offset":1632,"ancestor_record_offset":844,"inlinee":6326,"source_line":23,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":545,"record_offset":3672,"parent_record_offset":3648,"ancestor_record_offset":844,"inlinee":4748,"source_line":254,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$0::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":545,"record_offset":3648,"parent_record_offset":3624,"ancestor_record_offset":844,"inlinee":4746,"source_line":50,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec_impl.rs
//     evidence={"module":545,"record_offset":3696,"parent_record_offset":3672,"ancestor_record_offset":844,"inlinee":4750,"source_line":72,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::truncate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec_impl.rs
//     evidence={"module":545,"record_offset":3720,"parent_record_offset":3696,"ancestor_record_offset":844,"inlinee":4751,"source_line":76,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::handle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":545,"record_offset":3048,"parent_record_offset":2952,"ancestor_record_offset":844,"inlinee":13685,"source_line":61,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::QueryPoolCreateInfo::query_count
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":545,"record_offset":2924,"parent_record_offset":2880,"ancestor_record_offset":844,"inlinee":13684,"source_line":7695,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":545,"record_offset":3076,"parent_record_offset":2952,"ancestor_record_offset":844,"inlinee":13687,"source_line":24,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":545,"record_offset":3100,"parent_record_offset":3076,"ancestor_record_offset":844,"inlinee":13688,"source_line":11,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":545,"record_offset":3124,"parent_record_offset":3100,"ancestor_record_offset":844,"inlinee":13689,"source_line":16,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":545,"record_offset":1292,"parent_record_offset":1268,"ancestor_record_offset":844,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":545,"record_offset":4796,"parent_record_offset":4772,"ancestor_record_offset":844,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":545,"record_offset":2716,"parent_record_offset":2688,"ancestor_record_offset":844,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":545,"record_offset":7280,"parent_record_offset":7256,"ancestor_record_offset":844,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":545,"record_offset":1600,"parent_record_offset":1152,"ancestor_record_offset":844,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":545,"record_offset":4852,"parent_record_offset":4828,"ancestor_record_offset":844,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=64
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":545,"record_offset":1876,"parent_record_offset":1852,"ancestor_record_offset":844,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":545,"record_offset":3384,"parent_record_offset":3360,"ancestor_record_offset":844,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":545,"record_offset":3016,"parent_record_offset":2992,"ancestor_record_offset":844,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":545,"record_offset":1232,"parent_record_offset":1208,"ancestor_record_offset":844,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":545,"record_offset":3252,"parent_record_offset":2880,"ancestor_record_offset":844,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":545,"record_offset":1364,"parent_record_offset":1340,"ancestor_record_offset":844,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":545,"record_offset":1392,"parent_record_offset":1364,"ancestor_record_offset":844,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":545,"record_offset":3336,"parent_record_offset":3312,"ancestor_record_offset":844,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_checked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":545,"record_offset":4708,"parent_record_offset":4684,"ancestor_record_offset":844,"inlinee":11953,"source_line":2153,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_mut_checked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":545,"record_offset":4528,"parent_record_offset":4496,"ancestor_record_offset":844,"inlinee":8567,"source_line":2192,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_mut_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":545,"record_offset":4564,"parent_record_offset":4528,"ancestor_record_offset":844,"inlinee":11985,"source_line":2092,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":545,"record_offset":4452,"parent_record_offset":4424,"ancestor_record_offset":844,"inlinee":8427,"source_line":448,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":545,"record_offset":4424,"parent_record_offset":4396,"ancestor_record_offset":844,"inlinee":11639,"source_line":533,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":545,"record_offset":3360,"parent_record_offset":3336,"ancestor_record_offset":844,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":545,"record_offset":1180,"parent_record_offset":1152,"ancestor_record_offset":844,"inlinee":6791,"source_line":343,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":545,"record_offset":1208,"parent_record_offset":1180,"ancestor_record_offset":844,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$90::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":545,"record_offset":4684,"parent_record_offset":4660,"ancestor_record_offset":844,"inlinee":11952,"source_line":1896,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":545,"record_offset":1316,"parent_record_offset":1292,"ancestor_record_offset":844,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":545,"record_offset":3192,"parent_record_offset":3164,"ancestor_record_offset":844,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":545,"record_offset":7196,"parent_record_offset":7172,"ancestor_record_offset":844,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":545,"record_offset":3216,"parent_record_offset":3192,"ancestor_record_offset":844,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":545,"record_offset":7220,"parent_record_offset":7196,"ancestor_record_offset":844,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$17::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":545,"record_offset":4292,"parent_record_offset":4268,"ancestor_record_offset":844,"inlinee":4362,"source_line":3323,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":545,"record_offset":6716,"parent_record_offset":6664,"ancestor_record_offset":844,"inlinee":7542,"source_line":3441,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":545,"record_offset":6688,"parent_record_offset":6664,"ancestor_record_offset":844,"inlinee":7541,"source_line":3417,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::store::impl$6::asset::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/store.rs
//     evidence={"module":545,"record_offset":1268,"parent_record_offset":1180,"ancestor_record_offset":844,"inlinee":30964,"source_line":168,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::texture_export::export_packaged_textures::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x112CE
// 首条位置: {"module":1253,"inlinee":70350}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1253,"record_offset":34276,"parent_record_offset":34256,"ancestor_record_offset":34256,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1253,"record_offset":34324,"parent_record_offset":34300,"ancestor_record_offset":34256,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1253,"record_offset":34300,"parent_record_offset":34276,"ancestor_record_offset":34256,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::texture_export::export_packaged_textures::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 116
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1409C
// 首条位置: {"module":1656,"inlinee":82076}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::texture::impl$58::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/raw/texture.rs
//     evidence={"module":1656,"record_offset":27792,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":13934,"source_line":301,"archive":"_pdb/C/modules/01656.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":1656,"record_offset":27820,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/01656.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphResourceApi::export
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":1656,"record_offset":28012,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":13028,"source_line":105,"archive":"_pdb/C/modules/01656.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphResourceApi::import
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":1656,"record_offset":27644,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":12355,"source_line":79,"archive":"_pdb/C/modules/01656.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::impl$6::import_mut_with_access_type
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":1656,"record_offset":27732,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":12379,"source_line":1321,"archive":"_pdb/C/modules/01656.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":1656,"record_offset":27888,"parent_record_offset":27572,"ancestor_record_offset":27572,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/01656.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::texture_export::export_2d::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":1656,"record_offset":27952,"parent_record_offset":27916,"ancestor_record_offset":27572,"inlinee":33540,"source_line":37700,"archive":"_pdb/C/modules/01656.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::texture_export::export_2d_array::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":1656,"record_offset":27856,"parent_record_offset":27820,"ancestor_record_offset":27572,"inlinee":29369,"source_line":37750,"archive":"_pdb/C/modules/01656.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::impl$0::into_graph_borrow
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":1656,"record_offset":27696,"parent_record_offset":27668,"ancestor_record_offset":27572,"inlinee":8323,"source_line":186,"archive":"_pdb/C/modules/01656.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::impl$6::export_with_access_type
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":1656,"record_offset":28036,"parent_record_offset":28012,"ancestor_record_offset":27572,"inlinee":12390,"source_line":1341,"archive":"_pdb/C/modules/01656.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::impl$6::import_with_access_type
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/api.rs
//     evidence={"module":1656,"record_offset":27668,"parent_record_offset":27644,"ancestor_record_offset":27572,"inlinee":12356,"source_line":1308,"archive":"_pdb/C/modules/01656.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::texture_export::export_packaged_textures::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 172
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x112CD
// 首条位置: {"module":1253,"inlinee":70349}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1253,"record_offset":33528,"parent_record_offset":33508,"ancestor_record_offset":33508,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1253,"record_offset":33576,"parent_record_offset":33552,"ancestor_record_offset":33508,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1253,"record_offset":33552,"parent_record_offset":33528,"ancestor_record_offset":33508,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01253.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::texture_export::png_export_encoding
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 190
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x844F
// 首条位置: {"module":545,"inlinee":33871}
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
// 完整结构及原始类型记录: texture_export.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: texture_export.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: texture_export.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::render::texture_export::export_packaged_textures
// 信息 ID: 7ade40ad607b74403e486804
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure$0::closure_env$9,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x68e662 -> 0xa92f20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::texture_export::export_packaged_textures::closure$0
// 信息 ID: 9e95e6494525f66d6287313d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::texture_export::export_packaged_textures", "captures": "unknown"}
// [闭包环境类型引用] country_core::render::texture_export::export_packaged_textures::closure_env$0 <- anyhow::context::impl$0::with_context<turbosloth::store::PackageManifest,std::io::error::Error,alloc::string::String,country_core::render::texture_export::export_packaged_textures::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<turbosloth::store::PackageManifest,std::io::error::Error,alloc::string::String,country_core::render::texture_export::export_packaged_textures::closure_env$0>; module=1253, inline_record=34256; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::texture_export::export_packaged_textures::closure$1
// 信息 ID: 0daece78593656fd3666faad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::render::texture_export::export_packaged_textures", "captures": "unknown"}
// [闭包环境类型引用] country_core::render::texture_export::export_packaged_textures::closure_env$1 <- rhapsody::rg::executor::GraphExecutionRecorder::compile_and_execute<rhapsody::rg::executor::GraphExecutorSerial,tuple$<>,country_core::render::texture_export::export_packaged_textures::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::render::texture_export::export_packaged_textures::closure_env$1 <- rhapsody::rg::compile::compile<tuple$<>,country_core::render::texture_export::export_packaged_textures::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] rhapsody::rg::compile::compile<tuple$<>,country_core::render::texture_export::export_packaged_textures::closure_env$1>; module=1656, inline_record=27572; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::texture_export::export_packaged_textures::closure$2
// 信息 ID: eeffc07482f17d24393cafbd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::render::texture_export::export_packaged_textures", "captures": "unknown"}
// [闭包环境类型引用] country_core::render::texture_export::export_packaged_textures::closure_env$2 <- anyhow::context::impl$0::with_context<tuple$<>,enum2$<image::error::ImageError>,alloc::string::String,country_core::render::texture_export::export_packaged_textures::closure_env$2>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<tuple$<>,enum2$<image::error::ImageError>,alloc::string::String,country_core::render::texture_export::export_packaged_textures::closure_env$2>; module=1253, inline_record=33508; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::texture_export::png_export_encoding
// 信息 ID: f83c812f3299ba0b9fa1fc2a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::texture_export::export_packaged_textures; module=545, inline_record=3560; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
